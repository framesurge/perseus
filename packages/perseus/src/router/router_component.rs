use crate::{
    checkpoint,
    i18n::detect_locale,
    i18n::Locales,
    router::{
        get_initial_view, get_subsequent_view, GetSubsequentViewProps, InitialView, RouterLoadState,
    },
    router::{PerseusRoute, RouteInfo, RouteVerdict},
    template::{RenderCtx, TemplateMap, TemplateNodeType},
    utils::get_path_prefix_client,
    ErrorPages,
};
use std::collections::HashMap;
use std::rc::Rc;
use sycamore::{
    prelude::{
        component, create_effect, create_signal, on_mount, view, ReadSignal, Scope, Signal, View,
    },
    Prop,
};
use sycamore_futures::spawn_local_scoped;
use sycamore_router::{HistoryIntegration, RouterBase};
use web_sys::Element;

// We don't want to bring in a styling library, so we do this the old-fashioned
// way! We're particularly comprehensive with these because the user could
// *potentially* stuff things up with global rules https://medium.com/@jessebeach/beware-smushed-off-screen-accessible-text-5952a4c2cbfe
const ROUTE_ANNOUNCER_STYLES: &str = r#"
    margin: -1px;
    padding: 0;
    border: 0;
    clip: rect(0 0 0 0);
    height: 1px;
    width: 1px;
    overflow: hidden;
    position: absolute;
    white-space: nowrap;
    word-wrap: normal;
"#;

/// Get the view we should be rendering at the moment, based on a route verdict.
/// This should be called on every route change to update the page. This doesn't
/// actually render the view, it just returns it for rendering. Note that this
/// may return error pages.
///
/// This function is designed for managing subsequent views only, since the
/// router component should be instantiated *after* the initial view
/// has been hydrated.
///
/// If the page needs to redirect to a particular locale, then this function
/// will imperatively do so.
async fn get_view(
    cx: Scope<'_>,
    verdict: RouteVerdict<TemplateNodeType>,
) -> View<TemplateNodeType> {
    checkpoint("router_entry");
    match &verdict {
        RouteVerdict::Found(RouteInfo {
            path,
            template,
            locale,
            was_incremental_match,
        }) => {
            get_subsequent_view(GetSubsequentViewProps {
                cx,
                path: path.clone(),
                template: template.clone(),
                was_incremental_match: *was_incremental_match,
                locale: locale.clone(),
                route_verdict: verdict,
            })
            .await
        }
        // For subsequent loads, this should only be possible if the dev forgot `link!()`
        RouteVerdict::LocaleDetection(path) => {
            let render_ctx = RenderCtx::from_ctx(cx);
            let dest = detect_locale(path.clone(), &render_ctx.locales);
            // Since this is only for subsequent loads, we know the router is instantiated
            // This shouldn't be a replacement navigation, since the user has deliberately
            // navigated here
            sycamore_router::navigate(&dest);
            View::empty()
        }
        RouteVerdict::NotFound => {
            let render_ctx = RenderCtx::from_ctx(cx);
            checkpoint("not_found");
            // TODO Update the router state here (we need a path though...)
            // This function only handles subsequent loads, so this is all we have
            render_ctx
                .error_pages
                .get_view_and_render_head(cx, "", 404, "not found", None)
        }
    }
}

/// The properties that the router takes.
#[derive(Debug, Prop)]
pub(crate) struct PerseusRouterProps {
    /// The error pages the app is using.
    pub error_pages: ErrorPages<TemplateNodeType>,
    /// The locales settings the app is using.
    pub locales: Locales,
    /// The templates the app is using.
    pub templates: TemplateMap<TemplateNodeType>,
    /// The render configuration of the app (which lays out routing information,
    /// among other things).
    pub render_cfg: HashMap<String, String>,
    /// The maximum size of the page state store, before pages are evicted
    /// to save memory in the browser.
    pub pss_max_size: usize,
}

/// The Perseus router. This is used internally in the Perseus engine, and you
/// shouldn't need to access this directly unless you're building a custom
/// engine. Note that this actually encompasses your entire app, and takes no
/// child properties.
///
/// Note: this deliberately has a snake case name, and should be called directly
/// with `cx` as the first argument, allowing the `AppRoute` generic
/// creates with `create_app_root!` to be provided easily. That given `cx`
/// property will be used for all context registration in the app.
#[component]
pub(crate) fn perseus_router(
    cx: Scope,
    PerseusRouterProps {
        error_pages,
        locales,
        templates,
        render_cfg,
        pss_max_size,
    }: PerseusRouterProps,
) -> View<TemplateNodeType> {
    // Now create an instance of `RenderCtx`, which we'll insert into context and
    // use everywhere throughout the app (this contains basically everything Perseus
    // needs in terms of infrastructure)
    let render_ctx = RenderCtx::new(
        pss_max_size,
        locales,   // Pretty light
        templates, // Already has `Rc`s
        Rc::new(render_cfg),
        Rc::new(error_pages),
    )
    .set_ctx(cx);

    // Get the current path, removing any base paths to avoid relative path locale
    // redirection loops (in previous versions of Perseus, we used Sycamore to
    // get the path, and it strips this out automatically)
    // Note that this does work with full URL paths, because
    // `get_path_prefix_client` does automatically get just the pathname
    // component.
    let path_prefix = get_path_prefix_client();
    let path = web_sys::window().unwrap().location().pathname().unwrap();
    let path = if path.starts_with(&path_prefix) {
        path.strip_prefix(&path_prefix).unwrap()
    } else {
        &path
    };
    // Prepare the initial view for hydration (because we have everything we need in
    // global window variables, this can be synchronous)
    let initial_view = get_initial_view(cx, path.to_string());
    let initial_view = match initial_view {
        InitialView::View(initial_view) => initial_view,
        // if we need to redirect, then we'll create a fake view that will just execute that code
        // (which we can guarantee to run after the router is ready)
        InitialView::Redirect(dest) => {
            let dest = dest.clone();
            on_mount(cx, move || {
                sycamore_router::navigate_replace(&dest);
            });
            // Note that using an empty view doesn't lead to any layout shift, since
            // redirects have nothing rendered on the server-side (so this is actually
            // correct hydration)
            View::empty()
        }
    };
    // Define a `Signal` for the view we're going to be currently rendering, which
    // will contain the current page, or some kind of error message
    let curr_view: &Signal<View<TemplateNodeType>> = create_signal(cx, initial_view);
    // This allows us to not run the subsequent load code on the initial load (we
    // need a separate one for the reload commander)
    let is_initial = create_signal(cx, true);
    let is_initial_reload_commander = create_signal(cx, true);

    // Create a `Route` to pass through Sycamore with the information we need
    let route = PerseusRoute {
        verdict: RouteVerdict::NotFound,
        // TODO Can we pass through a context instead and use these are necessary?
        templates: render_ctx.templates.clone(),
        render_cfg: render_ctx.render_cfg.clone(),
        locales: render_ctx.locales.clone(),
    };

    // Create a derived state for the route announcement
    // We do this with an effect because we only want to update in some cases (when
    // the new page is actually loaded) We also need to know if it's the first
    // page (because we don't want to announce that, screen readers will get that
    // one right)
    let route_announcement = create_signal(cx, String::new());
    let mut is_first_page = true; // This is different from the first page load (this is the first page as a
                                  // whole)
    let load_state = render_ctx.router.get_load_state_rc();
    create_effect(cx, move || {
        if let RouterLoadState::Loaded { path, .. } = &*load_state.get() {
            if is_first_page {
                // This is the first load event, so the next one will be for a new page (or at
                // least something that we should announce, if this page reloads then the
                // content will change, that would be from thawing)
                is_first_page = false;
            } else {
                // TODO Validate approach with reloading
                // A new page has just been loaded and is interactive (this event only fires
                // after all rendering and hydration is complete)
                // Set the announcer to announce the title, falling back to the first `h1`, and
                // then falling back again to the path
                let document = web_sys::window().unwrap().document().unwrap();
                // If the content of the provided element is empty, this will transform it into
                // `None`
                let make_empty_none = |val: Element| {
                    let val = val.inner_html();
                    if val.is_empty() {
                        None
                    } else {
                        Some(val)
                    }
                };
                let title = document
                    .query_selector("title")
                    .unwrap()
                    .and_then(make_empty_none);
                let announcement = match title {
                    Some(title) => title,
                    None => {
                        let first_h1 = document
                            .query_selector("h1")
                            .unwrap()
                            .and_then(make_empty_none);
                        match first_h1 {
                            Some(val) => val,
                            // Our final fallback will be the path
                            None => path.to_string(),
                        }
                    }
                };

                route_announcement.set(announcement);
            }
        }
    });

    // Listen for changes to the reload commander and reload as appropriate
    let router_state = &render_ctx.router;
    create_effect(cx, move || {
        router_state.reload_commander.track();
        // Using a tracker of the initial state separate to the main one is fine,
        // because this effect is guaranteed to fire on page load (they'll both be set)
        if *is_initial_reload_commander.get_untracked() {
            is_initial_reload_commander.set(false);
        } else {
            // Get the route verdict and re-run the function we use on route changes
            // This has to be untracked, otherwise we get an infinite loop that will
            // actually break client browsers (I had to manually kill Firefox...)
            // TODO Investigate how the heck this actually caused an infinite loop...
            let verdict = router_state.get_last_verdict();
            let verdict = match verdict {
                Some(verdict) => verdict,
                // If the first page hasn't loaded yet, terminate now
                None => return,
            };
            spawn_local_scoped(cx, async move {
                let new_view = get_view(cx, verdict.clone()).await;
                curr_view.set(new_view);
            });
        }
    });

    // This section handles live reloading and HSR freezing
    // We used to have an indicator shared to the macros, but that's no longer used
    #[cfg(all(feature = "live-reload", debug_assertions))]
    {
        use crate::state::Freeze;
        // Set up a oneshot channel that we can use to communicate with the WS system
        // Unfortunately, we can't share senders/receivers around without bringing in
        // another crate And, Sycamore's `RcSignal` doesn't like being put into
        // a `Closure::wrap()` one bit
        let (live_reload_tx, live_reload_rx) = futures::channel::oneshot::channel();
        crate::spawn_local_scoped(cx, async move {
            match live_reload_rx.await {
                // This will trigger only once, and then can't be used again
                // That shouldn't be a problem, because we'll reload immediately
                Ok(_) => {
                    #[cfg(all(feature = "hsr"))]
                    {
                        let frozen_state = render_ctx.freeze();
                        crate::state::hsr_freeze(frozen_state).await;
                    }
                    crate::state::force_reload();
                    // We shouldn't ever get here unless there was an error, the
                    // entire page will be fully reloaded
                }
                _ => (),
            }
        });

        // If live reloading is enabled, connect to the server now
        // This doesn't actually perform any reloading or the like, it just signals
        // places that have access to the render context to do so (because we need that
        // for state freezing/thawing)
        crate::state::connect_to_reload_server(live_reload_tx);
    }

    // This handles HSR thawing
    #[cfg(all(feature = "hsr", debug_assertions))]
    {
        crate::spawn_local_scoped(cx, async move {
            // We need to make sure we don't run this more than once, because that would
            // lead to a loop It also shouldn't run on any pages after the
            // initial load
            if render_ctx.is_first.get() {
                render_ctx.is_first.set(false);
                crate::state::hsr_thaw(&render_ctx).await;
            }
        });
    };

    // Append the route announcer to the end of the document body
    let document = web_sys::window().unwrap().document().unwrap();
    let announcer = document.create_element("p").unwrap();
    announcer.set_attribute("aria-live", "assertive").unwrap();
    announcer.set_attribute("role", "alert").unwrap();
    announcer
        .set_attribute("style", ROUTE_ANNOUNCER_STYLES)
        .unwrap();
    announcer.set_id("__perseus_route_announcer");
    let body_elem: Element = document.body().unwrap().into();
    body_elem
        .append_with_node_1(&announcer.clone().into())
        .unwrap();
    // Update the announcer's text whenever the `route_announcement` changes
    create_effect(cx, move || {
        let ra = route_announcement.get();
        announcer.set_inner_html(&ra);
    });

    view! { cx,
        // This is a lower-level version of `Router` that lets us provide a `Route` with the data we want
        RouterBase(
            integration = HistoryIntegration::new(),
            route = route,
            view = move |cx, route: &ReadSignal<PerseusRoute>| {
                // Do this on every update to the route, except the first time, when we'll use the initial load
                create_effect(cx, move || {
                    route.track();

                    if *is_initial.get_untracked() {
                        is_initial.set(false);
                    } else {
                        spawn_local_scoped(cx, async move {
                            let route = route.get();
                            let verdict = route.get_verdict();
                            let new_view = get_view(cx, verdict.clone()).await;
                            curr_view.set(new_view);
                        });
                    }
                });

                // This template is reactive, and will be updated as necessary
                // However, the server has already rendered initial load content elsewhere, so we move that into here as well in the app shell
                // The main reason for this is that the router only intercepts click events from its children

                view! { cx,
                        (*curr_view.get())
                }
            }
        )
    }
}
