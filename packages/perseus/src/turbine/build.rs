use super::Turbine;
use crate::{
    errors::*,
    i18n::TranslationsManager,
    init::PerseusAppBase,
    path::*,
    plugins::PluginAction,
    reactor::{RenderMode, RenderStatus},
    state::{BuildPaths, StateGeneratorInfo, TemplateState},
    stores::MutableStore,
    template::Entity,
    utils::{minify, ssr_fallible},
};
use futures::future::try_join_all;
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
        let locales = self.locales.get_all();

        // Build all the global states, for each locale, in parallel
        let mut global_state_futs = Vec::new();
        for locale in locales.into_iter() {
            global_state_futs
                .push(self.build_global_state_for_locale(locale.to_string(), exporting));
        }
        let global_states_by_locale = try_join_all(global_state_futs).await?;
        let global_states_by_locale = HashMap::from_iter(global_states_by_locale.into_iter());
        // Cache these for use by every other template
        self.global_states_by_locale = global_states_by_locale;

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
        )
        .await?;
        self.html_shell = Some(html_shell);

        Ok(())
    }
    /// Builds the global state for a given locale, returning a tuple of the
    /// locale and the state generated. This will also write the global
    /// state to the immutable store (there is no such thing as revalidation
    /// for global state, and it is *extremely* unlikely that there ever will
    /// be).
    async fn build_global_state_for_locale(
        &self,
        locale: String,
        exporting: bool,
    ) -> Result<(String, TemplateState), ServerError> {
        let gsc = &self.global_state_creator;

        if exporting && (gsc.uses_request_state() || gsc.can_amalgamate_states()) {
            return Err(ExportError::GlobalStateNotExportable.into());
        }

        let global_state = if gsc.uses_build_state() {
            // Generate the global state and write it to a file
            let global_state = gsc.get_build_state(locale.clone()).await?;
            self.immutable_store
                .write(
                    // We put the locale at the end to prevent confusion with any pages
                    &format!("static/global_state_{}.json", &locale),
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
        Ok((locale, global_state))
    }
    /// This returns the fragment of the render configuration generated by this
    /// template/capsule.
    // Note we use page/template rhetoric here, but this coudl equally be
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
                let stripped = page_path.strip_suffix('/').unwrap_or(page_path);
                let stripped = stripped.strip_prefix('/').unwrap_or(stripped);
                let mut stripped = stripped.to_string();
                page_path = &mut stripped;

                let full_path = format!("{}/{}", &entity.get_path(), &page_path);
                render_cfg_frag.insert(full_path, entity.get_path());
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
                &format!("static/{}.extra.json", entity.get_path()),
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
                    // We created these from the same loop as we render each path for each locale
                    // from, so this is safe to `.unwrap()`
                    let global_state = self.global_states_by_locale.get(locale).unwrap().clone();
                    path_futs.push(self.build_path_or_widget_for_locale(
                        path,
                        entity,
                        &extra,
                        locale,
                        global_state,
                        exporting,
                    ));
                }
            }
            try_join_all(path_futs).await?;
        }

        Ok(render_cfg_frag)
    }
    /// The path this accepts is the path *within* the entity, not including the
    /// entity's name! It is assumed that the path provided to this function
    /// has been stripped of extra leading/trailing forward slashes.
    ///
    /// This function will do nothing for entities that are not either basic or
    /// build-state-generating.
    ///
    /// This function is `super`-public because it's used to generate
    /// incremental pages. Because of this, it also takes in the most
    /// up-to-date global state.
    pub(super) async fn build_path_or_widget_for_locale(
        &self,
        path: PurePath,
        entity: &Entity<SsrNode>,
        extra: &TemplateState,
        locale: &str,
        global_state: TemplateState,
        exporting: bool,
    ) -> Result<(), ServerError> {
        let translator = self
            .translations_manager
            .get_translator_for_locale(locale.to_string())
            .await?;

        let full_path_without_locale = PathWithoutLocale(match entity.uses_build_paths() {
            true => format!("{}/{}", &entity.get_path(), path.0),
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
                    path: (*full_path_without_locale).clone(),
                    locale: translator.get_locale(),
                    extra: extra.clone(),
                })
                .await?;
            // Write the state to the appropriate store (mutable if the entity revalidates)
            let state_str = build_state.state.to_string();
            if entity.revalidates() {
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
            return Ok(());
        };

        // For templates (*not* capsules), we'll render the full content (with
        // dependencies), and the head (which capsules don't have), provided
        // it's not always going to be useless (i.e. if this uses request state)
        if !entity.is_capsule && !entity.uses_request_state() {
            // Render the head (which has no dependencies)
            let head_str =
                entity.render_head_str(state.clone(), global_state.clone(), &translator)?;
            let head_str = minify(&head_str, true)?;
            if entity.revalidates() {
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

            // Weird block creation because this has to be `Send`able later for serving
            let (prerendered, render_status, widget_states) = {
                // Construct the render mode we're using, which is needed because we don't
                // know what dependencies are in a page/widget until we actually render it,
                // which means we might find some that can't be built at build-time.
                let render_status = Rc::new(RefCell::new(RenderStatus::Ok));
                let widget_states = Rc::new(RefCell::new(HashMap::new()));
                let mode = RenderMode::Build {
                    render_status: render_status.clone(),
                    widget_render_cfg: self.render_cfg.clone(),
                    entities: self.entities.clone(),
                    immutable_store: self.immutable_store.clone(),
                    widget_states: widget_states.clone(),
                };

                // Now prerender the actual content (a bit roundabout for error handling)
                let prerendered = ssr_fallible(move |cx| {
                    entity.render_for_template_server(
                        full_path.clone(),
                        state,
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

                (prerendered, render_status, widget_states)
            };

            // Check how the render went
            match render_status {
                RenderStatus::Ok => {
                    let prerendered = minify(&prerendered, true)?;
                    // Write that prerendered HTML to a static file (whose presence is used to
                    // indicate that this page/widget was fine to be built at
                    // build-time, and will not change at request-time;
                    // therefore this will be blindly returned at request-time).
                    // We also write a JSON file with a map of all the widget states, since the
                    // browser will need to know them for hydration.
                    if entity.revalidates() {
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
                }
                RenderStatus::Err(err) => return Err(err),
                // One of the dependencies couldn't be built at build-time,
                // so, by not writing a prerender to the store, we implicitly
                // reschedule it (unless this hasn't been allowed by the user,
                // or if we're exporting)
                RenderStatus::Cancelled => {
                    if exporting {
                        return Err(ExportError::DependenciesNotExportable {
                            template_name: entity.get_path(),
                        }
                        .into());
                    } else if !entity.can_be_rescheduled {
                        return Err(ServerError::TemplateCannotBeRescheduled {
                            template_name: entity.get_path(),
                        });
                    }
                }
            }
        }

        Ok(())
    }
}
