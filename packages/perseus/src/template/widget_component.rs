use crate::path::PathWithoutLocale;
#[cfg(engine)]
use sycamore::reactive::create_child_scope;
use sycamore::web::View;

use super::Capsule;

impl<P: Clone + 'static> Capsule<P> {
    /// Creates a component for a single widget that this capsule can produce,
    /// based on the given path. This is designed to be used inside the
    /// Sycamore `view!` macro.
    ///
    /// Note that this will not behave like a normal Sycamore component, and it
    /// is effectively a normal function (for now).
    ///
    /// The path provided to this should not include the name of the capsule
    /// itself. For example, if the capsule path is `foo`, and you want the
    /// `bar` widget within `foo` (i.e. `foo/bar`), you should provide
    /// `/bar` to this function. If you want to render the index widget, just
    /// use `/` or the empty string (leading forward slashes will automatically
    /// be normalized).
    pub fn widget(
        &self,
        // This is a `PurePath`, meaning it *does not* have a locale or the capsule name!
        path: &str,
        props: P,
    ) -> View {
        self.__widget(path, props, false)
    }
    /// An alternative to `.widget()` that delays the rendering of the widget
    /// until the rest of the page has loaded.
    ///
    /// Normally, a widget will have its state generated at the earliest
    /// possible opportunity (e.g. if it only uses build state, it will be
    /// generated at build-time, but one using request state would have to
    /// wait until request-time) and its contents prerendered with the pages
    /// that use it. However, sometimes, you may have a particularly 'heavy'
    /// widget that involves a large amount of state. If you're finding a
    /// certain page is loading a bit slowly due to such a widget, then you
    /// may wish to use `DelayedWidget` instead, which will generate state
    /// as usual, but, when it comes time to actually render the widget in
    /// this page, a placeholder will be inserted, and the whole widget will
    /// only be rendered on the browser-side with an asynchronous fetch of
    /// the state.
    ///
    /// Usually, you won't need to delay a widget, and choosing to use this over
    /// `.widget()` should be based on real-world testing.
    ///
    /// Note that using other widgets inside a delayed widget will cause those
    /// other widgets to be delayed in this context. Importantly, a widget
    /// that is delayed in one page can be non-delayed in another page:
    /// think of widgets as little modules that are imported into pages.
    /// Delaying is just one importing strategy, by that logic. In fact, one
    /// of the reasons you may wish to delay a widget's load is if it has a
    /// very large nesting of depdendencies, which would slow down
    /// server-side processing (although fetching on the browser-side will
    /// almost always be quite a bit slower). Again, you should
    /// base your choices with delaying on empirical data!
    pub fn delayed_widget(&self, path: &str, props: P) -> View {
        self.__widget(path, props, true)
    }

    /// The internal widget component logic. Note that this ignores scope
    /// disposers entirely, as all scopes used are children of the given,
    /// which is assumed to be the page-level scope. As such, widgets will
    /// automatically be cleaned up with pages.
    #[allow(unused_variables)]
    fn __widget(&self, path: &str, props: P, delayed: bool) -> View {
        // Handle leading and trailing slashes
        let path = path.strip_prefix('/').unwrap_or(path);
        let path = path.strip_suffix('/').unwrap_or(path);

        // This will also add `__capsule/` implicitly
        let path = format!("{}/{}", self.inner.get_path(), path);
        // This is needed for index widgets
        let path = path.strip_suffix('/').unwrap_or(&path);
        let path = PathWithoutLocale(path.to_string());

        #[cfg(engine)]
        return {
            let view = if delayed {
                // On the engine-side, delayed widgets should just render their
                // fallback views
                let fallback_fn = self.fallback.as_ref().unwrap();
                (fallback_fn)(props)
            } else {
                self.engine_widget(path, props)
            };

            view
        };
        // On the browser-side, delayed and non-delayed are the same (it just matters as
        // to what's been preloaded)
        #[cfg(any(client, doc))]
        return self.browser_widget(path, props);
    }

    /// The internal browser-side logic for widgets, both delayed and not.
    #[cfg(any(client, doc))]
    fn browser_widget(&self, path: PathWithoutLocale, props: P) -> View {
        use crate::{
            errors::ClientInvariantError,
            path::PathMaybeWithLocale,
            reactor::Reactor,
            router::{match_route, FullRouteInfo, FullRouteVerdict},
            template::PreloadInfo,
        };

        let reactor = Reactor::from_cx();
        // This won't panic, because widgets won't be rendered until the initial laod is
        // ready for them
        let locale = reactor.get_translator().get_locale();
        let full_path = PathMaybeWithLocale::new(&path, &locale);
        // This has the locale, and is used as the identifier for the calling page in
        // the PSS. This will be `Some(..)` as long as we're not running in an error
        // page (in which case we should immediately terminate anyway) or the like.
        let caller_path = reactor
            .router_state
            .get_path()
            .expect("tried to include widget in bad environment (probably an error view)");

        // Figure out route information for this
        let path_segments = full_path
            .split('/')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>(); // This parsing is identical to the Sycamore router's
        let verdict = match_route(
            &path_segments,
            &reactor.render_cfg,
            &reactor.entities,
            &reactor.locales,
        );

        match verdict.into_full(&reactor.entities) {
            FullRouteVerdict::Found(FullRouteInfo {
                path: _,
                entity,
                was_incremental_match,
                locale,
            }) => {
                // We have the capsule we want as `self`, but we also need to run the routing
                // algorithm to handle incremental matching and localization.
                // Obviously, the router should return the same capsule as we
                // actually have, otherwise there would be some *seriously* weird stuff going
                // on! If you're seeing this as a user, my best suggestion is
                // that you might have two templates that somehow overlap: e.g.
                // `foo/bar` and `gloo/bar`. You might have used `GLOO.widget()`,
                // but that somehow put out `foo/bar` as the path. This should not be possible,
                // and will, unless you have seriously modified the router or
                // other internals, indicate a Perseus bug: please report this!
                debug_assert_eq!(entity.get_path(), self.inner.get_path());

                match self.render_widget_for_template_client(
                    full_path,
                    caller_path,
                    props,
                    PreloadInfo {
                        locale,
                        was_incremental_match,
                    },
                ) {
                    Ok(view) => view,
                    Err(err) => reactor.error_views.handle_widget(err),
                }
            }
            // Widgets are all resolved on the server-side, meaning they are checked then too (be it
            // at build-time or request-time). If this happpens, the user is rendering
            // an invalid widget on the browser-side only.
            _ => reactor.error_views.handle_widget(
                ClientInvariantError::BadWidgetRouteMatch {
                    path: (*path).to_string(),
                }
                .into(),
            ),
        }
    }

    /// The internal engine-side logic for widgets.
    #[cfg(engine)]
    fn engine_widget(&self, path: PathWithoutLocale, props: P) -> View {
        use std::sync::Arc;

        use crate::error_views::ErrorViews;
        use crate::errors::{ClientError, ServerError, StoreError};
        use crate::path::PathMaybeWithLocale;
        use crate::reactor::{Reactor, RenderMode, RenderStatus};
        use crate::state::TemplateState;
        use futures::executor::block_on;

        // This will always be rendered with access to the Perseus render context, which
        // we will be working with a lot!
        let reactor = Reactor::from_cx();
        match &reactor.render_mode {
            RenderMode::Build {
                render_status,
                widget_render_cfg,
                immutable_store,
                widget_states,
                possibly_incremental_paths,
            } => {
                // If the render status isn't good, don't even bother proceeding, and fail-fast
                // instead
                if !matches!(*render_status.borrow(), RenderStatus::Ok) {
                    return View::new();
                }

                // Check if we're in the render config (which will just contain widgets at this
                // point, since they're built first, and the rendering we're in now
                // for templates is executed afterward)
                if let Some(capsule_name) = widget_render_cfg.get(&*path) {
                    // Make sure this capsule would be safe for building
                    // If this were an incrementally generated widget, we wouldn't have even gotten
                    // this far, as it wouldn't be in the render config
                    if self.inner.uses_request_state() || self.inner.revalidates() {
                        *render_status.borrow_mut() = RenderStatus::Cancelled;
                        View::new()
                    } else {
                        // This won't panic, because the reactor has been fully instantiated with a
                        // translator on the engine-side (unless we're in an error
                        // page, which is totally invalid)
                        let locale = reactor.get_translator().get_locale();
                        // Get the path in a way we can work with
                        let path_encoded = format!(
                            "{}-{}",
                            &locale,
                            // The user provided this
                            urlencoding::encode(&path)
                        );
                        // Since this widget has state built at build-time that will never change,
                        // it *must* be in the immutable store (only
                        // revalidating states go into the mutable store,
                        // and this would be `false` in the map if it
                        // revalidated!). The immutable store is really just
                        // a filesystem API, and we have no choice
                        // but to block here.
                        let state = match block_on(
                            immutable_store.read(&format!("static/{}.json", path_encoded)),
                        ) {
                            Ok(state) => state,
                            // If there's no state file, we'll assume an empty state
                            Err(StoreError::NotFound { .. }) => "null".to_string(),
                            Err(err) => {
                                *render_status.borrow_mut() = RenderStatus::Err(err.into());
                                return View::new();
                            }
                        };
                        let state = match TemplateState::from_str(&state) {
                            Ok(state) => state,
                            Err(err) => {
                                *render_status.borrow_mut() =
                                    RenderStatus::Err(ServerError::InvalidPageState {
                                        source: err,
                                    });
                                return View::new();
                            }
                        };

                        let localized_path = PathMaybeWithLocale::new(&path, &locale);

                        // Add this to the list of widget states so they can be written for later
                        // use
                        widget_states.borrow_mut().insert(
                            localized_path.clone(),
                            (capsule_name.to_string(), state.state.clone()),
                        );

                        match self.render_widget_for_template_server(localized_path, state, props) {
                            Ok(view) => view,
                            Err(err) => {
                                *render_status.borrow_mut() =
                                    RenderStatus::Err(ServerError::ClientError(err));
                                View::new()
                            }
                        }
                    }
                } else {
                    // Either this widget can be incrementally generated, or it doesn't exist. We'll
                    // yield to the build process, which will build this if it's incremental, and
                    // just throw an error if it's not.
                    //
                    // Note that reschedulings can't arise from this, as incremental generation is
                    // a flexible pattern: it can be either build-time or request-time. Only request
                    // state or revalidation can trigger that.
                    possibly_incremental_paths.borrow_mut().push(path);
                    // We don't change the render status, because that would prevent other widgets
                    // from loading (and there might be multiple incrementals).
                    View::new()
                }
            }
            // Note: this will only happen for initial loads.
            RenderMode::Request {
                widget_states,
                error_views,
                unresolved_widget_accumulator,
            } => {
                // This won't panic, because the reactor has been fully instantiated with a
                // translator on the engine-side (unless we're in an error page,
                // which is totally invalid)
                let locale = reactor.get_translator().get_locale();
                let full_path = PathMaybeWithLocale::new(&path, &locale);
                // Check if we've already built this widget (i.e. are we up to this layer, or a
                // later one?)
                match widget_states.get(&full_path) {
                    Some(res) => match res {
                        // There were no problems with getting the state
                        Ok(state) => {
                            // Use that to render the widget for the server-side (this should *not*
                            // create a new reactor)
                            match self.render_widget_for_template_server(
                                full_path,
                                state.clone(),
                                props,
                            ) {
                                Ok(view) => view,
                                // We'll render any errors to the whole widget, even if they might
                                // be internal (but they *really*
                                // shouldn't be, since those
                                // should've been handled when trying to fetch
                                // the state, as there's no active syste etc. on the engine-side)
                                Err(err) => error_views.handle_widget(err),
                            }
                        }
                        // We're to render an error page with the given error data (which will not
                        // impact the rest of the page). Since this whole `Request`
                        // variant can only happen for initial loads, and since this is a
                        // `ServerError`, we'll make this take up the
                        // widget.
                        Err(err_data) => {
                            let err = ClientError::ServerError {
                                status: err_data.status,
                                message: err_data.msg.to_string(),
                            };

                            error_views.handle_widget(err)
                        }
                    },
                    None => {
                        // Just add this path to the list of unresolved ones, and it will be
                        // resolved in time for the next pass
                        unresolved_widget_accumulator.borrow_mut().push(path);
                        View::new()
                    }
                }
            }
            RenderMode::Head => panic!("widgets cannot be used in heads"),
            RenderMode::Error => panic!("widgets cannot be used in error views"),
            // This would be exceptionally weird...
            RenderMode::Headers => panic!("widgets cannot be used in headers"),
        }
    }
}
