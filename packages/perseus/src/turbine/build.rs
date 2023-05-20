use super::Turbine;
use crate::{
    errors::*,
    i18n::{TranslationsManager, Translator},
    init::PerseusAppBase,
    path::*,
    plugins::PluginAction,
    reactor::{RenderMode, RenderStatus},
    router::{match_route, FullRouteVerdict},
    server::get_path_slice,
    state::{BuildPaths, StateGeneratorInfo, TemplateState},
    stores::MutableStore,
    template::Entity,
    utils::{minify, ssr_fallible},
};
use futures::{
    future::{try_join_all, BoxFuture},
    FutureExt,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};
use sycamore::web::SsrNode;

impl<M: MutableStore, T: TranslationsManager> Turbine<M, T> {
    /// Builds your whole app for being run on a server. Do not use this
    /// function if you want to export your app.
    ///
    /// This returns an `Arc<Error>`, since any errors are passed to plugin
    /// actions for further processing.
    pub async fn build(&mut self) -> Result<(), Arc<Error>> {
        self.plugins
            .functional_actions
            .build_actions
            .before_build
            .run((), self.plugins.get_plugin_data())
            .map_err(|err| Arc::new(err.into()))?;
        let res = self.build_internal(false).await;
        if let Err(err) = res {
            let err: Arc<Error> = Arc::new(err.into());
            self.plugins
                .functional_actions
                .build_actions
                .after_failed_build
                .run(err.clone(), self.plugins.get_plugin_data())
                .map_err(|err| Arc::new(err.into()))?;

            Err(err)
        } else {
            self.plugins
                .functional_actions
                .build_actions
                .after_successful_build
                .run((), self.plugins.get_plugin_data())
                .map_err(|err| Arc::new(err.into()))?;

            Ok(())
        }
    }

    pub(super) async fn build_internal(&mut self, exporting: bool) -> Result<(), ServerError> {
        // Build the global state (also adds it to the immutable store)
        self.global_state = self.build_global_state(exporting).await?;

        let mut render_cfg = HashMap::new();

        // Now build every capsule's state in parallel (capsules are never rendered
        // outside a page)
        let mut capsule_futs = Vec::new();
        for capsule in self.entities.values() {
            if capsule.is_capsule {
                capsule_futs.push(self.build_template_or_capsule(capsule, exporting));
            }
        }
        let capsule_render_cfg_frags = try_join_all(capsule_futs).await?;
        // Add to the render config as appropriate
        for fragment in capsule_render_cfg_frags.into_iter() {
            render_cfg.extend(fragment.into_iter());
        }

        // We now update the render config with everything we learned from the capsule
        // building so the actual renders of the pages can resolve the widgets
        // they have (if they have one that's not in here, it wasn't even built,
        // and would cancel the render).
        self.render_cfg = render_cfg.clone();

        // Now build every template's state in parallel
        let mut template_futs = Vec::new();
        for template in self.entities.values() {
            if !template.is_capsule {
                template_futs.push(self.build_template_or_capsule(template, exporting));
            }
        }
        let template_render_cfg_frags = try_join_all(template_futs).await?;
        // Add to the render config as appropriate
        for fragment in template_render_cfg_frags.into_iter() {
            render_cfg.extend(fragment.into_iter());
        }

        // Now write the render config to the immutable store
        self.immutable_store
            .write(
                "render_conf.json",
                &serde_json::to_string(&render_cfg).unwrap(),
            )
            .await?;
        self.render_cfg = render_cfg;

        // And build the HTML shell (so that this does the exact same thing as
        // instantiating from files)
        let html_shell = PerseusAppBase::<SsrNode, M, T>::get_html_shell(
            self.index_view_str.to_string(),
            &self.root_id,
            &self.render_cfg,
            &self.plugins,
            self.get_path_prefix_server().as_ref(),
        )
        .await?;
        self.html_shell = Some(html_shell);

        Ok(())
    }
    /// Builds the global state, returning the state generated. This will also
    /// write the global state to the immutable store (there is no such
    /// thing as revalidation for global state, and it is *extremely*
    /// unlikely that there ever will be).
    async fn build_global_state(&self, exporting: bool) -> Result<TemplateState, ServerError> {
        let gsc = &self.global_state_creator;

        if exporting && (gsc.uses_request_state() || gsc.can_amalgamate_states()) {
            return Err(ExportError::GlobalStateNotExportable.into());
        }

        let global_state = if gsc.uses_build_state() {
            // Generate the global state and write it to a file
            let global_state = gsc.get_build_state().await?;
            self.immutable_store
                .write(
                    // We put the locale at the end to prevent confusion with any pages
                    "static/global_state.json",
                    &global_state.state.to_string(),
                )
                .await?;
            global_state
        } else {
            // If there's no build-time handler, we'll give an empty state. This will
            // be very unexpected if the user is generating at request-time, since all
            // the pages they have at build-time will be unable to access the global state.
            // We could either completely disable build-time rendering when there's
            // request-time global state generation, or we could give the user smart
            // errors and let them manage this problem themselves by gating their
            // usage of global state at build-time, since `.try_get_global_state()`
            // will give a clear `Ok(None)` at build-time. For speed, the latter
            // approach has been chosen.
            //
            // This is one of the biggest 'gotchas' in Perseus, and is clearly documented!
            TemplateState::empty()
        };
        Ok(global_state)
    }
    /// This returns the fragment of the render configuration generated by this
    /// template/capsule, including the additions of any extra widgets that
    /// needed to be incrementally built ahead of time.
    // Note we use page/template rhetoric here, but this could equally be
    // widget/capsule
    async fn build_template_or_capsule(
        &self,
        entity: &Entity<SsrNode>,
        exporting: bool,
    ) -> Result<HashMap<String, String>, ServerError> {
        // If we're exporting, ensure that all the capsule's strategies are export-safe
        // (not requiring a server)
        if exporting
            && (entity.revalidates() ||
                entity.uses_incremental() ||
                entity.uses_request_state() ||
                // We check amalgamation as well because it involves request state, even if that wasn't provided
                entity.can_amalgamate_states())
        {
            return Err(ExportError::TemplateNotExportable {
                template_name: entity.get_path(),
            }
            .into());
        }

        let mut render_cfg_frag = HashMap::new();

        // We extract the paths and extra state for rendering outside, but we handle the
        // render config inside this block
        let (paths, extra) = if entity.uses_build_paths() {
            let BuildPaths { mut paths, extra } = entity.get_build_paths().await?;

            // Add all the paths to the render config (stripping erroneous slashes as we go)
            for mut page_path in paths.iter_mut() {
                // Strip any erroneous slashes
                let stripped = page_path.strip_prefix('/').unwrap_or(page_path);
                let mut stripped = stripped.to_string();
                page_path = &mut stripped;

                let full_path = format!("{}/{}", &entity.get_path(), &page_path);
                // And perform another strip for index pages to work
                let full_path = full_path.strip_suffix('/').unwrap_or(&full_path);
                let full_path = full_path.strip_prefix('/').unwrap_or(full_path);
                render_cfg_frag.insert(full_path.to_string(), entity.get_path());
            }

            // Now if the page uses ISR, add an explicit `/*` in there after the template
            // root path. Incremental rendering requires build-time path generation.
            if entity.uses_incremental() {
                render_cfg_frag.insert(format!("{}/*", &entity.get_path()), entity.get_path());
            }

            (paths, extra)
        } else {
            // There's no facility to generate extra paths for this template, so it only
            // renders itself

            // The render config should map the only page this generates to the template of
            // the same name
            render_cfg_frag.insert(entity.get_path(), entity.get_path());
            // No extra state, one empty path for the index
            (vec![String::new()], TemplateState::empty())
        };
        // We write the extra state even if it's empty
        self.immutable_store
            .write(
                &format!(
                    "static/{}.extra.json",
                    urlencoding::encode(&entity.get_path())
                ),
                &extra.state.to_string(),
            )
            .await?;

        // We now have a populated render config, so we should build each path in
        // parallel for each locale, if we can. Yes, the function we're calling
        // will also write the revalidation text, but, if you're not using build
        // state or being basic, the you're using request state, which means
        // revalidation is completely irrelevant, since you're revalidating on
        // every load.
        if entity.uses_build_state() || entity.is_basic() {
            let mut path_futs = Vec::new();
            for path in paths.into_iter() {
                for locale in self.locales.get_all() {
                    let path = PurePath(path.clone());
                    path_futs.push(self.build_path_or_widget_for_locale(
                        path,
                        entity,
                        &extra,
                        locale,
                        self.global_state.clone(),
                        exporting,
                        false,
                    ));
                }
            }
            // Extend the render configuration with any incrementally generated widgets
            let render_cfg_exts = try_join_all(path_futs).await?;
            for ext in render_cfg_exts {
                render_cfg_frag.extend(ext.into_iter());
            }
        }

        Ok(render_cfg_frag)
    }
    /// The path this accepts is the path *within* the entity, not including the
    /// entity's name! It is assumed that the path provided to this function
    /// has been stripped of extra leading/trailing forward slashes. This
    /// returns a map of widgets that needed to be incrementally built ahead
    /// of time to avoid rescheduling that should be added to the render
    /// config.
    ///
    /// This function will do nothing for entities that are not either basic or
    /// build-state-generating.
    ///
    /// This function is `super`-public because it's used to generate
    /// incremental pages. Because of this, it also takes in the most
    /// up-to-date global state.
    #[allow(clippy::too_many_arguments)] // Internal function
    pub(super) async fn build_path_or_widget_for_locale(
        &self,
        path: PurePath,
        entity: &Entity<SsrNode>,
        extra: &TemplateState,
        locale: &str,
        global_state: TemplateState,
        exporting: bool,
        // This is used in request-time incremental generation
        force_mutable: bool,
    ) -> Result<HashMap<String, String>, ServerError> {
        let translator = self
            .translations_manager
            .get_translator_for_locale(locale.to_string())
            .await?;

        let full_path_without_locale = PathWithoutLocale(match entity.uses_build_paths() {
            // Note the stripping of trailing `/`s here (otherwise index build paths fail)
            true => {
                let full = format!("{}/{}", &entity.get_path(), path.0);
                let full = full.strip_suffix('/').unwrap_or(&full);
                full.strip_prefix('/').unwrap_or(full).to_string()
            }
            // We don't want to concatenate the name twice if we don't have to
            false => entity.get_path(),
        });
        // Create the encoded path, which always includes the locale (even if it's
        // `xx-XX` in a non-i18n app)
        //
        // BUG: insanely nested paths won't work whatsoever if the filename is too long,
        // maybe hash instead?
        let full_path_encoded = format!(
            "{}-{}",
            translator.get_locale(),
            urlencoding::encode(&full_path_without_locale)
        );
        // And we'll need the full path with the locale for the `PageProps`
        // If it's `xx-XX`, we should just have it without the locale (this may be
        // interacted with by users)
        let locale = translator.get_locale();
        let full_path = PathMaybeWithLocale::new(&full_path_without_locale, &locale);

        // First, if this page revalidates, write a timestamp about when it was built to
        // the mutable store (this will be updated to keep track)
        if entity.revalidates_with_time() {
            let datetime_to_revalidate = entity
                .get_revalidate_interval()
                .unwrap()
                .compute_timestamp();
            // Note that different locales do have different revalidation schedules
            self.mutable_store
                .write(
                    &format!("static/{}.revld.txt", full_path_encoded),
                    &datetime_to_revalidate.to_string(),
                )
                .await?;
        }

        let state = if entity.is_basic() {
            // We don't bother writing the state of basic entities
            TemplateState::empty()
        } else if entity.uses_build_state() {
            let build_state = entity
                .get_build_state(StateGeneratorInfo {
                    // IMPORTANT: It is very easy to break Perseus here; always make sure this is
                    // the pure path, without the template name!
                    // TODO Compat mode for v0.3.0x?
                    path: (*path).clone(),
                    locale: translator.get_locale(),
                    extra: extra.clone(),
                })
                .await?;
            // Write the state to the appropriate store (mutable if the entity revalidates)
            let state_str = build_state.state.to_string();
            if force_mutable || entity.revalidates() {
                self.mutable_store
                    .write(&format!("static/{}.json", full_path_encoded), &state_str)
                    .await?;
            } else {
                self.immutable_store
                    .write(&format!("static/{}.json", full_path_encoded), &state_str)
                    .await?;
            }

            build_state
        } else {
            // There's nothing we can do with any other sort of template at build-time
            return Ok(HashMap::new());
        };

        // For templates (*not* capsules), we'll render the full content (with
        // dependencies), and the head (which capsules don't have), provided
        // it's not always going to be useless (i.e. if this uses request state)
        if !entity.is_capsule && !entity.uses_request_state() {
            // Render the head (which has no dependencies)
            let head_str =
                entity.render_head_str(state.clone(), global_state.clone(), &translator)?;
            let head_str = minify(&head_str, true)?;
            if force_mutable || entity.revalidates() {
                self.mutable_store
                    .write(
                        &format!("static/{}.head.html", full_path_encoded),
                        &head_str,
                    )
                    .await?;
            } else {
                self.immutable_store
                    .write(
                        &format!("static/{}.head.html", full_path_encoded),
                        &head_str,
                    )
                    .await?;
            }

            // This stores a list of widgets that are able to be incrementally generated,
            // but that weren't in the build paths listing of their capsules.
            // These were, however, used at build-time by another page, meaning
            // they should be automatically built for convenience --- we can
            // therefore avoid an unnecessary build reschedule.
            //
            // Note that incremental generation is entirely side-effect based, so this
            // function call just internally maintains a series of additions to
            // the render configuration to be added properly once we're out of
            // all the `async`. This *could* lead to race conditions on the
            // immutable store, but Tokio should handle this.
            self.build_render(
                entity,
                full_path,
                &full_path_encoded,
                translator,
                state,
                global_state,
                exporting,
                force_mutable,
                // Start off with what's already known
                self.render_cfg.clone(),
            )
            .await
        } else {
            Ok(HashMap::new())
        }
    }

    /// This enables recursion for rendering incrementally rendered widgets
    /// (ideally, users would include the widgets they want at build-time in
    /// their build paths, but, if we don't do this, then unnecessary
    /// build reschedulings would abound a little too much).
    ///
    /// Each iteration of this will return a map to extend the render
    /// configuration with, but the function maintains its own internal
    /// render configuration passed through arguments, because it's designed to
    /// be part of the larger asynchronous build process, therefore modifying
    /// the root-level render config is not safe until the end.
    #[allow(clippy::too_many_arguments)] // Internal function
    fn build_render<'a>(
        &'a self,
        entity: &'a Entity<SsrNode>,
        full_path: PathMaybeWithLocale,
        full_path_encoded: &'a str,
        translator: Translator,
        state: TemplateState,
        global_state: TemplateState,
        exporting: bool,
        force_mutable: bool,
        mut render_cfg: HashMap<String, String>,
    ) -> BoxFuture<'a, Result<HashMap<String, String>, ServerError>> {
        async move {
            let (prerendered, render_status, widget_states, paps) = {
                // Construct the render mode we're using, which is needed because we don't
                // know what dependencies are in a page/widget until we actually render it,
                // which means we might find some that can't be built at build-time.
                let render_status = Rc::new(RefCell::new(RenderStatus::Ok));
                let widget_states = Rc::new(RefCell::new(HashMap::new()));
                let possibly_incremental_paths = Rc::new(RefCell::new(Vec::new()));
                let mode = RenderMode::Build {
                    render_status: render_status.clone(),
                    // Make sure what we have is passed through
                    widget_render_cfg: render_cfg.clone(),
                    immutable_store: self.immutable_store.clone(),
                    widget_states: widget_states.clone(),
                    possibly_incremental_paths: possibly_incremental_paths.clone(),
                };

                // Now prerender the actual content
                let prerendered = ssr_fallible(|cx| {
                    entity.render_for_template_server(
                        full_path.clone(),
                        state.clone(),
                        global_state.clone(),
                        mode.clone(),
                        cx,
                        &translator,
                    )
                })?;
                let render_status = render_status.take();

                // With the prerender over, all references to this have been dropped
                // TODO Avoid cloning everything here
                let widget_states = (*widget_states).clone().into_inner();
                // let widget_states = Rc::try_unwrap(widget_states).unwrap().into_inner();
                // We know this is a `HashMap<String, (String, Value)>`, which will work
                let widget_states = serde_json::to_string(&widget_states).unwrap();
                let paps = (*possibly_incremental_paths).clone().into_inner();

                (prerendered, render_status, widget_states, paps)
            };

            // Check how the render went
            match render_status {
                RenderStatus::Ok => {
                    // `Ok` does not necessarily mean all is well: anything in `possibly_incremental_paths`
                    // constitutes a widget that could not be rendered because it wasn't in the
                    // render config (either needs to be incrementally rendered, or it doesn't exist).
                    if paps.is_empty() {
                        let prerendered = minify(&prerendered, true)?;
                        // Write that prerendered HTML to a static file (whose presence is used to
                        // indicate that this page/widget was fine to be built at
                        // build-time, and will not change at request-time;
                        // therefore this will be blindly returned at request-time).
                        // We also write a JSON file with a map of all the widget states, since the
                        // browser will need to know them for hydration.
                        if force_mutable || entity.revalidates() {
                            self.mutable_store
                                .write(&format!("static/{}.html", full_path_encoded), &prerendered)
                                .await?;
                            self.mutable_store
                                .write(
                                    &format!("static/{}.widgets.json", full_path_encoded),
                                    &widget_states,
                                )
                                .await?;
                        } else {
                            self.immutable_store
                                .write(&format!("static/{}.html", full_path_encoded), &prerendered)
                                .await?;
                            self.immutable_store
                                .write(
                                    &format!("static/{}.widgets.json", full_path_encoded),
                                    &widget_states,
                                )
                                .await?;
                        }
                        // In this path, we haven't accumulated any extensions to the render config, and we can jsut return
                        // whatever we were given (thereby preserving the effect of previous recursions)
                        Ok(render_cfg)
                    } else {
                        let mut futs = Vec::new();
                        for path in paps {
                            let locale = translator.get_locale();
                            let render_cfg = render_cfg.clone();
                            let global_state = global_state.clone();
                            futs.push(async move {
                                // It's also possible that these widgets just don't exist, so check that
                                let localized_path = PathMaybeWithLocale::new(&path, &locale);
                                let path_slice = get_path_slice(&localized_path);
                                let verdict = match_route(
                                    &path_slice,
                                    &render_cfg,
                                    &self.entities,
                                    &self.locales,
                                );

                                match verdict.into_full(&self.entities) {
                                    FullRouteVerdict::Found(route_info) => {
                                        let capsule_name = route_info.entity.get_path();
                                        // This will always exist
                                        let capsule_extra = match self
                                            .immutable_store
                                            .read(&format!(
                                                "static/{}.extra.json",
                                                urlencoding::encode(&capsule_name)
                                            ))
                                            .await
                                        {
                                            Ok(state) => {
                                                TemplateState::from_str(&state).map_err(|err| ServerError::InvalidBuildExtra {
                                                    template_name: capsule_name.clone(),
                                                    source: err,
                                                })?
                                            }
                                            // If this happens, then the immutable store has been tampered with, since
                                            // the build logic generates some kind of state for everything
                                            Err(_) => {
                                                return Err(ServerError::MissingBuildExtra {
                                                    template_name: capsule_name,
                                                })
                                            }
                                        };

                                        // The `path` is a `PathWithoutLocale`, and we need to strip the capsule name.
                                        // Because this is produced from the widget component in the first place, it should
                                        // be perfectly safe to unwrap everything here (any failures indicate users hand-rolling
                                        // widgets or a Perseus bug).
                                        let pure_path = path
                                            .strip_prefix(&capsule_name)
                                            .expect("couldn't strip capsule name from widget (unless you're hand-rolling widgets, this is a Perseus bug)");
                                        let pure_path = pure_path.strip_prefix('/').unwrap_or(pure_path);
                                        let pure_path = PurePath(pure_path.to_string());

                                        // This will perform the same process as we've done, recursing as necessary (yes, double recursion),
                                        // and it will return a set of render configuration extenstions too. This does NOT use the render
                                        // configuration internally, so we can avoid infinite loops.
                                        let exts = self.build_path_or_widget_for_locale(
                                            pure_path,
                                            route_info.entity,
                                            &capsule_extra,
                                            &locale,
                                            global_state.clone(),
                                            exporting,
                                            // It's incremental generation, but this will *end up* acting
                                            // like it was in build paths all along, so don't force the mutable
                                            // store unless we're being asked to from a higher level
                                            force_mutable,
                                        )
                                                       .await?;

                                        let mut render_cfg_ext = HashMap::new();

                                        render_cfg_ext.extend(exts.into_iter());
                                        // Now add this actual capsule itself
                                        render_cfg_ext.insert(path.0, capsule_name);

                                        Ok(render_cfg_ext)
                                    }
                                    FullRouteVerdict::LocaleDetection(_) => {
                                        Err(ServerError::ResolveDepLocaleRedirection {
                                            locale: locale.to_string(),
                                            widget: path.to_string(),
                                        })
                                    }
                                    FullRouteVerdict::NotFound { .. } => Err(ServerError::ResolveDepNotFound { widget: path.to_string(), locale: locale.to_string() }),
                                }
                            });
                        }
                        let render_cfg_exts = try_join_all(futs).await?;
                        // Add all those extensions to our internal copy, which will be used for recursion
                        for ext in render_cfg_exts {
                            render_cfg.extend(ext.into_iter());
                        }


                        // We've rendered all the possibly incremental widgets, failing if any were actually just nonexistent.
                        // However, because building a widget means building its state, not prerendering it, we don't know
                        // if we're going to have to do all this again because one of the widgets has yet another incremental
                        // dependency. So, restart the whole render process!
                        self.build_render(entity, full_path, full_path_encoded, translator, state, global_state, exporting, force_mutable, render_cfg).await
                    }
                }
                RenderStatus::Err(err) => Err(err),
                // One of the dependencies couldn't be built at build-time,
                // so, by not writing a prerender to the store, we implicitly
                // reschedule it (unless this hasn't been allowed by the user,
                // or if we're exporting).
                //
                // Important: this will **not** be returned for pages including
                // incremental widgets that haven't been built yet, those are handled
                // through `Ok`. Potentially non-existent widgets will also be handled
                // through there.
                RenderStatus::Cancelled => {
                    if exporting {
                        Err(ExportError::DependenciesNotExportable {
                            template_name: entity.get_path(),
                        }
                                   .into())
                    } else if !entity.can_be_rescheduled {
                        Err(ServerError::TemplateCannotBeRescheduled {
                            template_name: entity.get_path(),
                        })
                    } else {
                        // It's fine to reschedule later, so just return the widgets we have
                        Ok(render_cfg)
                    }
                }
            }
        }.boxed()
    }
}
