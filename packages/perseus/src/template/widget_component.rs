use crate::path::PathWithoutLocale;
use sycamore::{prelude::Scope, view::View, web::Html};

/// An alternative to [`Widget`] that delays the rendering of the widget until
/// the rest of the page has loaded.
///
/// Normally, a widget will have its state generated at the earliest possible
/// opportunity (e.g. if it only uses bujild state, it will be generated at
/// build-time, but one using request state would have to wait until
/// request-time) and its contents prerendered with the pages that use it.
/// However, sometimes, you may have a particularly 'heavy' widget that involves
/// a large amount of state. If you're finding a certain page is loading a
/// bit slowly due to such a widget, then you may wish to use `DelayedWidget`
/// instead, which will generate state as usual, but, when it comes time to
/// actually render the widget in this page, a placeholder will be inserted, and
/// the whole widget will only be rendered on the browser-side with an
/// asynchronous fetch of the state.
///
/// Usually, you won't need to delay a widget, and choosing to use this over
/// [`Widget`] should be based on real-world testing.
///
/// Note that using other widgets inside a delayed widget will cause those other
/// widgets to be delayed in this context. Importantly, a widget that is delayed
/// in one page can be non-delayed in another page: think of widgets as little
/// modules that are imported into pages. Delaying is just one importing
/// strategy, by that logic. In fact, one of the reasons you may wish to delay a
/// widget's load is if it has a very large nesting of depdendencies, which
/// would slow down server-side processing (although fetching on the
/// browser-side will almost always be quite a bit slower). Again, you should
/// base your choices with delaying on empirical data!
// Internally, the reason we can just return a `View::empty()` on the engine-side is because
// delayed widgets are guaranteed to only ever be fetched using the subsequent loads system,
// which means their state will automatically be sorted out by that, independent of any
// dependencies. I mention that browser-side loading of delayed widgets with deeply nested
// dependencies will lead to poor performance because you have to fetch each layer first, before
// proceeding with the next. This is the same as the engine-side, except 'fetching' there means a
// quick filesystem check and maybe a brief render, whereas, on the browser-side, it means a network
// request that will do the exact same thing: it will *always* take longer, unless the network speed
// is greater than infinity.
#[sycamore::component]
#[allow(unused_variables)]
pub fn DelayedWidget<G: Html>(cx: Scope, path: &str) -> View<G> {
    // On the engine-side, we expect absolutely nothing, no matter what
    // TODO Hydration??
    #[cfg(not(target_arch = "wasm32"))]
    return View::empty();

    // On the browser-side, we expect a `TemplateNodeType` (i.e. `HydrateNode` or
    // `DomNode`)
    #[cfg(target_arch = "wasm32")]
    {
        // Handle leading and trailing slashes
        let path = path.strip_prefix('/').unwrap_or(&path);
        let path = path.strip_suffix('/').unwrap_or(&path);

        let path = PathWithoutLocale(format!("__capsule/{}", path));

        return browser_widget(cx, path);
    }
}

/// A Sycamore component for rendering a Perseus widget by its path (not
/// including the `__capsule/` prefix). This will handle state generation and
/// prerendering automatically, signalling the calling page/widget (widgets can
/// be nested) if this widget is incompatible with the caller. That would occur
/// when the widget, for example, needs request-state, but the page uses only
/// build-state. Perseus wants to build the page at build-time, but the widget
/// prevents this. To solve this, you could either use [`DelayedWidget`] (which
/// will prevent rendering the widget until the browser-side), or you could
/// allow rescheduling of the relevant page's template, which gives Perseus
/// permission to delay rendering a build-time page until request-time, in
/// this case. Revalidating and incrementally generated widgets will cause
/// similar issues.
#[sycamore::component]
pub fn Widget<G: Html>(cx: Scope, path: &str) -> View<G> {
    // Handle leading and trailing slashes
    let path = path.strip_prefix('/').unwrap_or(path);
    let path = path.strip_suffix('/').unwrap_or(path);

    let path = PathWithoutLocale(format!("__capsule/{}", path));

    // On the engine-side, we expect an `SsrNode`
    #[cfg(not(target_arch = "wasm32"))]
    return engine_widget(cx, path);

    // On the browser-side, we expect a `TemplateNodeType` (i.e. `HydrateNode` or
    // `DomNode`)
    #[cfg(target_arch = "wasm32")]
    return browser_widget(cx, path);
}

/// The internal browser-side logic for widgets, both delayed and not.
#[cfg(target_arch = "wasm32")]
fn browser_widget<G: Html>(cx: Scope, path: PathWithoutLocale) -> View<G> {
    use crate::{
        errors::ClientInvariantError,
        path::PathMaybeWithLocale,
        reactor::Reactor,
        router::{match_route, RouteInfo, RouteVerdict},
        template::PreloadInfo,
    };

    let reactor = Reactor::from_cx(cx);
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

    match verdict {
        RouteVerdict::Found(RouteInfo {
            path: _,
            entity: capsule,
            was_incremental_match,
            locale,
        }) => {
            // Declare the dependency relationship so state store eviction works nicely
            reactor
                .state_store
                .declare_dependency(&full_path, &caller_path);

            match capsule.render_widget_for_template_client(
                full_path,
                cx,
                PreloadInfo {
                    locale,
                    was_incremental_match,
                },
            ) {
                Ok(view) => view,
                Err(err) => reactor.error_views.handle_widget(err, cx),
            }
        }
        // Widgets are all resolved on the server-side, meaning they are checked then too (be it at
        // build-time or request-time). If this happpens, the user is rendering an invalid
        // widget on the browser-side only.
        _ => reactor.error_views.handle_widget(
            ClientInvariantError::BadWidgetRouteMatch {
                path: (*path).to_string(),
            }
            .into(),
            cx,
        ),
    }
}

/// The internal engine-side logic for widgets.
#[cfg(not(target_arch = "wasm32"))]
fn engine_widget<G: Html>(cx: Scope, path: PathWithoutLocale) -> View<G> {
    use crate::errors::{ClientError, ServerError};
    use crate::path::PathMaybeWithLocale;
    use crate::reactor::{Reactor, RenderMode, RenderStatus};
    use crate::state::TemplateState;
    use futures::executor::block_on;
    use sycamore::prelude::*;

    // This will always be rendered with access to the Perseus render context, which
    // we will be working with a lot!
    let reactor = Reactor::<G>::from_cx(cx);
    match &reactor.render_mode {
        RenderMode::Build {
            render_status,
            widget_render_cfg,
            immutable_store,
            entities,
            widget_states,
        } => {
            // If the render status isn't good, don't even bother proceeding, and fail-fast
            // instead
            if !matches!(*render_status.borrow(), RenderStatus::Ok) {
                return View::empty();
            }

            // Check if we're in the render config (which will just contain widgets at this
            // point, since they're built first, and the rendering we're in now
            // for templates is executed afterward)
            if let Some(capsule_name) = widget_render_cfg.get(&*path) {
                let capsule = match entities.get(capsule_name) {
                    Some(capsule) => capsule,
                    // The render configuration is invalid, but the build creates it...
                    None => unreachable!("render configuration was invalid by the time of build-time widget rendering (this is a bug)"),
                };
                // Make sure this capsule would be safe for building
                // If this were an incrementally generated widget, we wouldn't have even gotten
                // this far, as it wouldn't be in the render config
                if capsule.uses_request_state() || capsule.revalidates() {
                    *render_status.borrow_mut() = RenderStatus::Cancelled;
                    View::empty()
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
                    // Since this widget has state built at build-time that will never change, it
                    // *must* be in the immutable store (only revalidating
                    // states go into the mutable store, and this would be
                    // `false` in the map if it revalidated!). The immutable
                    // store is really just a filesystem API, and we have no choice
                    // but to block here.
                    let state = match block_on(
                        immutable_store.read(&format!("static/{}.json", path_encoded)),
                    ) {
                        Ok(state) => state,
                        Err(err) => {
                            *render_status.borrow_mut() = RenderStatus::Err(err.into());
                            return View::empty();
                        }
                    };
                    let state = match TemplateState::from_str(&state) {
                        Ok(state) => state,
                        Err(err) => {
                            *render_status.borrow_mut() =
                                RenderStatus::Err(ServerError::InvalidPageState { source: err });
                            return View::empty();
                        }
                    };

                    // Add this to the list of widget states so they can be written for later use
                    widget_states.borrow_mut().insert(
                        path.to_string(),
                        (capsule_name.to_string(), state.state.clone()),
                    );

                    match capsule.render_widget_for_template_server(
                        PathMaybeWithLocale::new(&path, &locale),
                        state,
                        cx,
                    ) {
                        Ok(view) => view,
                        Err(err) => {
                            *render_status.borrow_mut() =
                                RenderStatus::Err(ServerError::ClientError(err));
                            View::empty()
                        }
                    }
                }
            } else {
                // This widget will be incrementally generated (TODO should we try to build it
                // now?). It could also just not exist, but we can't confirm
                // that until request time (since incremenally generated
                // page could also be invalid). Remember that this will only get through if the
                // user has explicitly allowed deferring renders.
                *render_status.borrow_mut() = RenderStatus::Cancelled;
                View::empty()
            }
        }
        // Note: this will only happen for initial loads.
        RenderMode::Request {
            widget_states,
            entities,
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
                    Ok((capsule_name, state)) => {
                        // Get the capsule this widget was generated by
                        let capsule = match entities.get(capsule_name) {
                            Some(capsule) => capsule,
                            // We successfully got a proper routing match for this, so finding that
                            // the capsule then doesn't exist should not
                            // be possible
                            None => unreachable!(),
                        };
                        // Use that to render the widget for the server-side (this should *not*
                        // create a new reactor)
                        match capsule.render_widget_for_template_server(
                            full_path,
                            state.clone(),
                            cx,
                        ) {
                            Ok(view) => view,
                            // We'll render any errors to the whole widget, even if they might be
                            // internal (but they *really* shouldn't be,
                            // since those should've been handled when trying to fetch
                            // the state, as there's no active syste etc. on the engine-side)
                            Err(err) => error_views.handle_widget(err, cx),
                        }
                    }
                    // We're to render an error page with the given error data (which will not
                    // impact the rest of the page). Since this whole `Request`
                    // variant can only happen for initial loads, and since this is a `ServerError`,
                    // we'll make this take up the widget.
                    Err(err_data) => {
                        let err = ClientError::ServerError {
                            status: err_data.status,
                            message: err_data.msg.to_string(),
                        };

                        error_views.handle_widget(err, cx)
                    }
                },
                None => {
                    // Just add this path to the list of unresolved ones, and it will be resolved in
                    // time for the next pass
                    unresolved_widget_accumulator.borrow_mut().push(path);
                    View::empty()
                }
            }
        }
        RenderMode::Head => panic!("widgets cannot be used in heads"),
        RenderMode::Error => panic!("widgets cannot be used in error views"),
        // This would be exceptionally weird...
        RenderMode::Headers => panic!("widgets cannot be used in headers"),
    }
}
