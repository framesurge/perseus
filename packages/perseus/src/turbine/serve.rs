use std::{cell::RefCell, collections::HashMap, pin::Pin, rc::Rc};

use chrono::{DateTime, Utc};
use futures::{Future, FutureExt, future::try_join_all};
use serde_json::Value;
use sycamore::web::SsrNode;

use crate::{Request, StateGeneratorInfo, Template, errors::*, i18n::{TranslationsManager, Translator}, internal::{PageData, PageDataPartial}, router::{RouteVerdictAtomic, match_route_atomic}, server::get_path_slice, stores::MutableStore, template::{RenderMode, States, TemplateState, UnknownStateType}};
use super::Turbine;

/// This is `PageDataPartial`, but it keeps the state as `TemplateState` for internal convenience.
struct StateAndHead {
    state: TemplateState,
    head: String,
}

impl<M: MutableStore, T: TranslationsManager> Turbine<M, T> {
    /// Gets the state for the given path. This will render the head, but it will *not* render
    /// the contents, and, as a result, will not engage with any dependencies this page/widget
    /// may have. If this is used to get the state of a capsule, the head will of course be
    /// empty.
    ///
    /// This returns a tuple of the state and head, along with the updated global state, which
    /// will be needed in determining the headers to render.
    pub async fn get_state_for_path(
        &self,
        path: &str, // This must not contain the locale, but it *will* contain the entity name
        locale: &str,
        entity_name: &str,
        was_incremental: bool,
        req: Request,
    ) -> Result<(PageDataPartial, TemplateState), ServerError> {
        let (
            StateAndHead { state, head },
            global_state
        ) = self.get_state_for_path_internal(path, locale, entity_name, was_incremental, req, None, None).await?;

        Ok((
            PageDataPartial {
                state: state.state,
                head,
            },
            global_state,
        ))
    }
    /// Gets the full page data for the given path. This will generate the state, render the head, and render
    /// the content of the page, resolving all widget dependencies.
    ///
    /// Like `.get_state_for_path()`, this returns the page data and the global state in a tuple.
    ///
    /// # Pitfalls
    /// This currently uses a layer-based dependency resolution algorithm, as a widget may itself have widgets.
    /// However, the widgets a page/widget uses may be dependent on its state, and therefore we cannot enumerate
    /// the entire dependency tree without knowing all the states involved. Therefore, we go layer-by-layer.
    /// Currently, we wait for each layer to be fully complete before proceeding to the next one, which leads to
    /// a layer taking as long as the longest state generation within it. This can lead to poor render times when
    /// widgets are highly nested, a pattern that should be avoided as much as possible.
    ///
    /// In future, this will build with maximal parallelism by not waiting for each layer to be finished building
    /// before proceeding to the next one.
    pub async fn get_initial_load_for_path(
        &self,
        path: &str,
        locale: &str,
        entity_name: &str,
        was_incremental: bool,
        req: Request,
    ) -> Result<(PageData, TemplateState), ServerError> {
        // Get the latest global state, which we'll share around
        let global_state = self.get_full_global_state_for_locale(locale, clone_req(&req)).await?;
        // We'll need the template (this is an initial load, so it's not a capsule) multiple times
        let template = self.templates.get(entity_name).ok_or(ServeError::PageNotFound { path: path.to_string() })?;
        // TODO Return not found if this is actually a capsule (terrible idea!!!)
        // Begin by generating the state for this page (we don't need to clone the global state, because this will give it back)
        let (page_state, global_state) = self.get_state_for_path_internal(path, locale, entity_name, was_incremental, clone_req(&req), Some(template), Some(global_state)).await?;

        // Yes, this is created twice; no, we don't care
        // If we're interacting with the stores, this is the path this page/widget will be under
        let path_encoded = format!("{}-{}", locale, urlencoding::encode(path));

        // The page state generation process will have updated any prerendered fragments of this page, which means they're guaranteed to
        // be up-to-date. Importantly, if any of the dependencies weren't build-safe, or if the page uses request-state (which means,
        // as explained above, we don't actually know what the dependencies are yet, let alone if they're build-safe), this fragment
        // won't exist. Basically, if it exists, we can return it straight away with no extra work. Otherwise, we'll have to do a layer-by-layer
        // render, which can handle non-build-safe dependencies. We call this a 'fragment' because it's not a complete HTML shell etc. (TODO?)
        let prerendered_fragment_res = if template.revalidates() {
            self
                .mutable_store
                .read(&format!("static/{}.html", &path_encoded))
                .await
        } else {
            self
                .immutable_store
                .read(&format!("static/{}.html", &path_encoded))
                .await
        };
        // Propagate any errors, but if the asset wasn't found, then record that as `None`
        let prerendered_fragment = match prerendered_fragment_res {
            Ok(fragment) => Some(fragment),
            Err(StoreError::NotFound { .. }) => None,
            Err(err) => return Err(err.into()),
        };

        if let Some(prerendered_fragment) = prerendered_fragment {
            // If there was a prerendered fragment, there will also be a record of the widget
            // states we need to send to the client
            let widget_states = if template.revalidates() {
                self
                    .mutable_store
                    .read(&format!("static/{}.widgets.json", &path_encoded))
                    .await?
            } else {
                self
                    .immutable_store
                    .read(&format!("static/{}.widgets.json", &path_encoded))
                    .await?
            };
            let widget_states = match serde_json::from_str::<HashMap<String, (String, Value)>>(&widget_states) {
                Ok(widget_states) => widget_states,
                Err(err) => return Err(ServerError::InvalidPageState { source: err })
            };
            Ok((PageData {
                content: prerendered_fragment,
                head: page_state.head,
                state: page_state.state.state,
                widget_states,
            }, global_state))
        } else {
            // It's time for layer-by-layer dependency resolution, but we need a translator first
            let translator = self
                    .translations_manager
                    .get_translator_for_locale(locale.to_string())
                    .await?;

            // This will block
            let (final_widget_states, prerendered) = self.render_all(
                HashMap::new(), // This starts empty
                path.to_string(),
                locale.to_string(),
                page_state.state.clone(),
                template,
                global_state.clone(),
                &req,
                &translator,
            ).await?;
            // Convert the `TemplateState`s into `Value`s
            let final_widget_states = final_widget_states
                .into_iter()
                .map(|(k, (v, s))| (k, (v, s.state)))
                .collect::<HashMap<_, _>>();

            Ok((PageData {
                content: prerendered,
                head: page_state.head,
                state: page_state.state.state,
                widget_states: final_widget_states,
            }, global_state))
        }
    }
    /// Recurses through each layer of dependencies and eventually renders the given page.
    ///
    /// This returns a tuple of widget states and the prerendered result.
    ///
    /// This is deliberately synchronous to avoid making `Self` `Sync`, which is impossible
    /// with Perseus' current design. Thus, this blocks when resolving each layer.
    fn render_all<'a>(
        &'a self,
        // This is a map of widget paths to their states and capsule names, which we'll populate as we go through. That way,
        // we can just run the exact same render over and over again, getting to a new layer each time, since,
        // if a widget finds its state in this, it'll use it. This will be progressively accumulated over
        // many layers.
        widget_states: HashMap<String, (String, TemplateState)>,
        path: String,
        locale: String,
        state: TemplateState,
        entity: &'a Template<SsrNode>, // This will recurse, so this could be a template of capsule
        global_state: TemplateState,
        req: &'a Request,
        translator: &'a Translator
    ) -> Pin<Box<dyn Future<Output = Result<(HashMap<String, (String, TemplateState)>, String), ServerError>> + 'a>> {
        // Misleadingly, this only has the locale if we're using i18n!
        let full_path_with_locale = match locale.as_str() {
            "xx-XX" => path.to_string(),
            locale => format!("{}/{}", &locale, &path),
        };

        // We put this in an `Rc` so it can be put in the context and given to multiple widgets, but it will
        // never be changed (we could have a lot of states here, so we want to minimize cloning where possible)
        let widget_states_rc = Rc::new(widget_states);
        // This will be used to store the paths of widgets that haven't yet been resolved. It will be cleared
        // between layers.
        let unresolved_widget_accumulator = Rc::new(RefCell::new(Vec::new()));
        // Now we want to render the page in the dependency resolution mode (as opposed to the build mode,
        // which just cancels the render if it finds any non-build-safe widgets).
        let mode = RenderMode::Request {
            widget_states: widget_states_rc.clone(),
            // This is a bunch of `Arc`s
            templates: self.templates.clone(),
            unresolved_widget_accumulator: unresolved_widget_accumulator.clone(),
        };

        // Start the first render. This registers all our mode stuff on `cx`,
        // which is dropped when this is done. So, we can safely get the widget states
        // back.
        let prerendered = sycamore::render_to_string(|cx| {
            entity.render_for_template_server(
                full_path_with_locale.clone(),
                state.clone(),
                global_state.clone(),
                mode.clone(),
                cx,
                &translator,
            )
        });
        // As explained above, this should never fail, because all references have been dropped
        let mut widget_states = Rc::try_unwrap(widget_states_rc).unwrap();

        // We'll just have accumulated a ton of unresolved widgets, probably. If not, then we're done! If yes,
        // we'll need to build all their states.
        let mut accumulator = Rc::try_unwrap(unresolved_widget_accumulator).unwrap().into_inner();

        async move {
            if accumulator.is_empty() {
                Ok((widget_states, prerendered))
            } else {
                // First, deduplicate (relevant if the same widget is used more than once). We don't
                // care about unstable sorting because these are strings.
                accumulator.sort_unstable();
                accumulator.dedup();

                let mut futs = Vec::new();
                for widget_path in accumulator.into_iter() {
                    let global_state = global_state.clone();
                    let locale = locale.clone();
                    futs.push(async move {
                        // Resolve the route
                        // Get a route verdict to determine the capsule this widget path maps to
                        let localized_widget_path = format!("{}/{}", &locale, &widget_path);
                        let path_slice = get_path_slice(&localized_widget_path);
                        let verdict = match_route_atomic(
                            &path_slice,
                            &self.render_cfg,
                            &self.templates,
                            &self.locales
                        );
                        let route_info = match verdict {
                            RouteVerdictAtomic::Found(route_info) => route_info,
                            RouteVerdictAtomic::LocaleDetection(_) => return Err(ServerError::ResolveDepLocaleRedirection {
                                locale: locale.to_string(),
                                widget: widget_path.to_string(),
                            }),
                            RouteVerdictAtomic::NotFound => return Err(ServerError::ResolveDepNotFound {
                                locale: locale.to_string(),
                                widget: widget_path.to_string(),
                            }),
                        };
                        let capsule_name = route_info.template.get_path();

                        // Now build the state
                        let state = self.get_state_for_path_internal(
                            &widget_path,
                            &locale,
                            &capsule_name,
                            route_info.was_incremental_match,
                            clone_req(req),
                            // We do happen to actually have this from the routing
                            Some(route_info.template),
                            Some(global_state)
                        ).await?;

                        // Return the tuples that'll go into `widget_states`
                        Ok((widget_path, (capsule_name, state.0.state)))
                    });
                }
                let tuples = try_join_all(futs).await?;
                widget_states.extend(tuples);

                // We've rendered this layer, and we're ready for the next one
                self.render_all(
                    widget_states,
                    path,
                    locale,
                    state,
                    entity,
                    global_state,
                    req,
                    translator
                ).await
            }
        }.boxed_local()
    }

    /// The internal version allows sharing a global state so we don't constantly regenerate it in recursion.
    async fn get_state_for_path_internal(
        &self,
        path: &str, // This must not contain the locale, but it *will* contain the entity name
        locale: &str,
        entity_name: &str,
        was_incremental: bool,
        req: Request,
        // If these are `None`, we'll generate them
        entity: Option<&Template<SsrNode>>, // Not for recursion, just convenience
        global_state: Option<TemplateState>,
    ) -> Result<(StateAndHead, TemplateState), ServerError> {
        // TODO Check if locale supported

        // This could be very different from the build-time global state
        let global_state = match global_state {
            Some(global_state) => global_state,
            None => self.get_full_global_state_for_locale(locale, clone_req(&req)).await?
        };

        let entity = match entity {
            Some(entity) => entity,
            None => self.templates.get(entity_name).ok_or(ServeError::PageNotFound { path: path.to_string() })?,
        };

        // If we're interacting with the stores, this is the path this page/widget will be under
        let path_encoded = format!("{}-{}", locale, urlencoding::encode(path));

        // Any work we do with the build logic will expect the path without the template name, so we need to
        // strip it (this could only fail if we'd mismatches the path to the entity name, which would be
        // either a malformed request or a *critical* Perseus routing bug)
        let pure_path = path.strip_prefix(entity_name).ok_or(ServerError::TemplateNameNotInPath)?;

        // If the entity is basic (i.e. has no state), bail early
        if entity.is_basic() {
            // Get the head (since this is basic, it has no state, and therefore
            // this would've been written at build-time)
            let head = self
                .immutable_store
                .read(&format!("static/{}.json", &path_encoded))
                .await?;

            return Ok((
                StateAndHead {
                    // No, this state is never written anywhere at build-time
                    state: TemplateState::empty(),
                    head
                },
                global_state,
            ));
        }

        // No matter what we end up doing, we're probably going to need this (which will always exist)
        let build_extra = match self.immutable_store
                                    .read(&format!("static/{}.extra.json", entity.get_path()))
                                    .await
        {
            Ok(state) => {
                TemplateState::from_str(&state).map_err(|err| ServerError::InvalidBuildExtra {
                    template_name: entity.get_path(),
                    source: err,
                })?
            }
            // If this happens, then the immutable store has been tampered with, since
            // the build logic generates some kind of state for everything
            Err(_) => {
                return Err(ServerError::MissingBuildExtra {
                    template_name: entity.get_path(),
                })
            }
        };
        // We'll need this too for any sort of state generation
        let build_info = StateGeneratorInfo {
            path: path.to_string(),
            locale: locale.to_string(),
            extra: build_extra.clone(),
        };

        // The aim of this next block is purely to ensure that whatever is in the im/mutable
        // store is the latest and most valid version of the build state, if we're even using
        // build state.
        //
        // If incremental and generated and not revalidating; get from immutable.
        // If incremental and not generated; generate.
        // If incremental and generated and revalidating; either get from mutable or revalidate.
        // If not incremental and revalidating; either get from mutable or revalidate.
        // If not incremental and not revalidating; get from immutable.
        if was_incremental {
            // If we have something in the appropriate store, then this has already been generated
            let res = if entity.revalidates() {
                self
                    .mutable_store
                    .read(&format!("static/{}.json", &path_encoded))
                    .await
            } else {
                self
                    .immutable_store
                    .read(&format!("static/{}.json", &path_encoded))
                    .await
            };
            // Propagate any errors, but if the asset wasn't found, then record that as `None`
            let built_state = match res {
                Ok(built_state) => Some(built_state),
                Err(StoreError::NotFound { .. }) => None,
                Err(err) => return Err(err.into()),
            };

            if let Some(built_state) = built_state {
                // This has been generated already, so we need to check for the possibility of revalidation
                let should_revalidate = self.page_or_widget_should_revalidate(&path_encoded, &entity, build_info.clone(), clone_req(&req)).await?;
                if should_revalidate {
                    // We need to rebuild, which we can do with the build-time logic
                    self.build_path_or_widget_for_locale(pure_path.to_string(), &entity, &build_extra, locale, global_state.clone(), false).await?;
                } else {
                    // We don't need to revalidate, so whatever is in the immutable store is valid
                }
            } else {
                // This is a new page, we need to actually generate it (which will handle any revalidation timestamps etc.).
                // For this, we can use the usual build state logic, which will perform a full render, unless the
                // dependencies aren't build-safe. Of course, we can guarantee if we're actually generating it now that
                // it won't be revalidating.
                // We can provide the most up-to-date global state to this
                self.build_path_or_widget_for_locale(pure_path.to_string(), &entity, &build_extra, locale, global_state.clone(), false).await?;
            }
        } else {
            let should_revalidate = self.page_or_widget_should_revalidate(&path_encoded, &entity, build_info.clone(), clone_req(&req)).await?;
            if should_revalidate {
                // We need to rebuild, which we can do with the build-time logic
                self.build_path_or_widget_for_locale(pure_path.to_string(), &entity, &build_extra, locale, global_state.clone(), false).await?;
            } else {
                // We don't need to revalidate, so whatever is in the immutable store is valid
            }
        }

        // Whatever is in the im/mutable store is now valid and up-to-date, so fetch it
        let build_state = if entity.uses_build_state() {
            let state_str = if entity.revalidates() {
                self
                    .mutable_store
                    .read(&format!("static/{}.json", &path_encoded))
                    .await?
            } else {
                self
                    .immutable_store
                    .read(&format!("static/{}.json", &path_encoded))
                    .await?
            };
            TemplateState::from_str(&state_str)
                .map_err(|err| ServerError::InvalidPageState { source: err })?
        } else {
            TemplateState::empty()
        };

        // Now get the request state if we're using it (of course, this must be re-generated
        // for every request)
        let request_state = if entity.uses_request_state() {
            entity.get_request_state(build_info.clone(), clone_req(&req)).await?
        } else {
            TemplateState::empty()
        };

        // Now handle the possibility of amalgamation
        let states = States {
            build_state,
            request_state,
        };
        let final_state = if states.both_defined() && entity.can_amalgamate_states() {
            entity.amalgamate_states(build_info, states.build_state, states.request_state).await?
        } else if states.both_defined() && !entity.can_amalgamate_states() {
            // We have both states, but can't amalgamate, so prioritze request state, as
            // it's more personalized and more recent
            states.request_state
        } else {
            // This only errors if both are defined, and we just checked that
            states.get_defined().unwrap()
        };

        // We now need to render the head. Whatever is on the im/mutable store is the most up-to-date, and that
        // won't have been written if we have an entity that uses request state (since it would always be invalid).
        // Therefore, if we don't use request state, it'll be in the appropriate store, otherwise we'll need to
        // render it ourselves. Of course, capsules don't have heads.
        let head_str = if !entity.is_capsule {
            if entity.uses_request_state() {
                // We only need a translator if we actually have to render the head, so we
                // won't get it until now
                let translator = self
                    .translations_manager
                    .get_translator_for_locale(locale.to_string())
                    .await?;
                entity.render_head_str(final_state.clone(), global_state.clone(), &translator)
            } else {
                // The im/mutable store was updated by the last whole block (since any incremental generation
                // or revalidation would have re-written the head if request state isn't being used)
                if entity.revalidates() {
                    self
                        .mutable_store
                        .read(&format!("static/{}.head.html", &path_encoded))
                        .await?
                } else {
                    self
                        .immutable_store
                        .read(&format!("static/{}.head.html", &path_encoded))
                        .await?
                }
            }
        } else {
            String::new()
        };

        Ok((
            StateAndHead {
                state: final_state,
                head: head_str,
            },
            global_state
        ))
    }

    /// Checks timestamps and runs user-provided logic to determine if the given widget/path should
    /// revalidatde at the present time.
    async fn page_or_widget_should_revalidate(
        &self,
        path_encoded: &str,
        entity: &Template<SsrNode>,
        build_info: StateGeneratorInfo<UnknownStateType>,
        req: Request,
    ) -> Result<bool, ServerError> {
        let mut should_revalidate = false;
        // If it revalidates after a certain period of time, we need to check that
        // BEFORE the custom logic (clearly documented)
        if entity.revalidates_with_time() {
            // Get the time when it should revalidate (RFC 3339)
            // This will be updated, so it's in a mutable store
            let datetime_to_revalidate_str = self.mutable_store
                .read(&format!("static/{}.revld.txt", path_encoded))
                .await?;
            let datetime_to_revalidate = DateTime::parse_from_rfc3339(&datetime_to_revalidate_str)
                .map_err(|err| ServerError::ServeError(ServeError::BadRevalidate { source: err }))?;
            // Get the current time (UTC)
            let now = Utc::now();

            // If the datetime to revalidate is still in the future, end with `false` (the custom
            // logic is only executed if the time-based one passes)
            if datetime_to_revalidate > now {
                return Ok(false);
            }
            should_revalidate = true;
        }

        // Now run the user's custom revalidation logic
        if entity.revalidates_with_logic() {
            should_revalidate = entity.should_revalidate(
                build_info,
                req
            ).await?;
        }
        Ok(should_revalidate)
}
    /// Gets the full global state from the state generated at build-time and the generator itself.
    /// This assumes that the provided locale is supported.
    ///
    /// This should only be called once per API call.
    async fn get_full_global_state_for_locale(
        &self,
        locale: &str,
        req: Request,
    ) -> Result<TemplateState, ServerError> {
        let gsc = &self.global_state_creator;
        // We know the locale is supported
        let built_state = self.global_states_by_locale.get(locale).unwrap();

        let global_state = if gsc.uses_request_state() {
            let req_state = gsc.get_request_state(locale.to_string(), req).await?;
            // If we have a non-empty build-time state, we'll need to amalgamate
            if !built_state.is_empty() {
                if gsc.can_amalgamate_states() {
                    gsc.amalgamate_states(locale.to_string(), built_state.clone(), req_state)
                       .await?
                } else {
                    // No amalgamation capability, request time state takes priority
                    req_state
                }
            } else {
                req_state
            }
        } else {
            // This global state is purely generated at build-time (or nonexistent)
            built_state.clone()
        };

        Ok(global_state)
    }
}

/// Clones a `Request` from its internal parts.
fn clone_req(raw: &Request) -> Request {
    let mut builder = Request::builder();

    for (name, val) in raw.headers() {
        builder = builder.header(name, val);
    }

    builder
        .uri(raw.uri())
        .method(raw.method())
        .version(raw.version())
        // We always use an empty body because, in a Perseus request, only the URI matters
        // Any custom data should therefore be sent in headers (if you're doing that, consider a
        // dedicated API)
        .body(())
        .unwrap() // This should never fail...
}