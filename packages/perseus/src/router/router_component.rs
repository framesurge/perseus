use crate::{ErrorPages, checkpoint, i18n::Locales, i18n::detect_locale, reactor::Reactor, router::{
        get_initial_view, get_subsequent_view, GetSubsequentViewProps, InitialView,
        RouterLoadState, SubsequentView,
    }, router::{PerseusRoute, RouteInfo, RouteManager, RouteVerdict}, template::{RenderCtx, TemplateMap, TemplateNodeType}, utils::get_path_prefix_client};
use std::collections::HashMap;
use std::rc::Rc;
use sycamore::{
    prelude::{
        component, create_effect, create_ref, create_signal, on_mount, view, ReadSignal, Scope,
        View,
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
/// This should be called on every route change to update the page. This will
/// return the view it returns, and may add to the scope disposers. Note that
/// this may return error pages.
///
/// This function is designed for managing subsequent views only, since the
/// router component should be instantiated *after* the initial view
/// has been hydrated.
///
/// If the page needs to redirect to a particular locale, then this function
/// will imperatively do so.
async fn set_view<'a>(
    cx: Scope<'a>,
    route_manager: &'a RouteManager<'a, TemplateNodeType>,
    verdict: RouteVerdict<TemplateNodeType>,
) {
    checkpoint("router_entry");
    match &verdict {
        RouteVerdict::Found(RouteInfo {
            path,
            template,
            locale,
            was_incremental_match,
        }) => {
            let subsequent_view = get_subsequent_view(GetSubsequentViewProps {
                cx,
                route_manager,
                path: path.clone(),
                template: template.clone(),
                was_incremental_match: *was_incremental_match,
                locale: locale.clone(),
                route_verdict: verdict,
            })
            .await;
            // Display any errors now
            if let SubsequentView::Error(view) = subsequent_view {
                route_manager.update_view(view);
            }
        }
        // For subsequent loads, this should only be possible if the dev forgot `link!()`
        RouteVerdict::LocaleDetection(path) => {
            let render_ctx = RenderCtx::from_ctx(cx);
            let dest = detect_locale(path.clone(), &render_ctx.locales);
            // Since this is only for subsequent loads, we know the router is instantiated
            // This shouldn't be a replacement navigation, since the user has deliberately
            // navigated here
            sycamore_router::navigate(&dest);
        }
        RouteVerdict::NotFound => {
            let render_ctx = RenderCtx::from_ctx(cx);
            checkpoint("not_found");
            // TODO Update the router state here (we need a path though...)
            // This function only handles subsequent loads, so this is all we have
            route_manager.update_view(render_ctx.error_pages.get_view_and_render_head(
                cx,
                "",
                404,
                "not found",
                None,
            ))
        }
    }
}

/// The properties that the router takes.
#[derive(Debug, Prop)]
pub(crate) struct PerseusRouterProps {
    /// The error pages the app is using.
    pub error_pages: Rc<ErrorPages<TemplateNodeType>>,
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
pub(crate) fn perseus_router(
    cx: Scope,
    reactor: Reactor<TemplateNodeType>,
) -> View<TemplateNodeType> {
    // Put the reactor into context so pages can access it easily
    reactor.add_self_to_cx(cx);

    // Add handlers to set up the app (this does not include those for view management or error handling)
    reactor.start(cx).unwrap(); // TODO

    let initial_view = reactor.get_initial_view(cx);

    // Create the route manager (note: this is cheap to clone)
    // TODO Add support for collecting the disposers of widgets
    let route_manager = RouteManager::new(cx);
    let route_manager = create_ref(cx, route_manager);


    // Prepare the initial view for hydration (because we have everything we need in
    // global window variables, this can be synchronous)
    let initial_view = get_initial_view(cx, path.to_string(), route_manager);
    match initial_view {
        // Any errors are simply returned, it's our responsibility to display them
        InitialView::Error(view) => route_manager.update_view(view),
        // If we need to redirect, then we'll create a fake view that will just execute that code
        // (which we can guarantee to run after the router is ready)
        InitialView::Redirect(dest) => {
            let dest = dest.clone();
            on_mount(cx, move || {
                sycamore_router::navigate_replace(&dest);
            });
        }
        // A successful render has already been displayed to the root
        InitialView::Success => (),
    };

    // This allows us to not run the subsequent load code on the initial load (we
    // need a separate one for the reload commander)
    let is_initial = create_signal(cx, true);

    // Create a `Route` to pass through Sycamore with the information we need
    let route = PerseusRoute {
        verdict: RouteVerdict::NotFound,
        cx: Some(cx),
    };

    // TODO Clear current widget disposers and exchange that empty list with the next one
    // TODO checkpoint("page_interactive");
    // TODO Router state loaded updates

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
                            set_view(cx, route_manager, verdict.clone()).await;
                        });
                    }
                });

                // This template is reactive, and will be updated as necessary
                // However, the server has already rendered initial load content elsewhere, so we move that into here as well in the app shell
                // The main reason for this is that the router only intercepts click events from its children

                view! { cx,
                        (*route_manager.view.get())
                }
            }
        )
    }
}
