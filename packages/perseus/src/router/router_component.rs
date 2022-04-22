use crate::{
    checkpoint,
    i18n::Locales,
    internal::{
        error_pages::ErrorPageData,
        i18n::{detect_locale, ClientTranslationsManager},
        router::{PerseusRoute, RouteInfo, RouteVerdict},
        shell::{app_shell, get_initial_state, InitialState, ShellProps},
    },
    state::{FrozenApp, GlobalState, PageStateStore, ThawPrefs},
    templates::{RouterLoadState, RouterState, TemplateNodeType},
    DomNode, ErrorPages, Html,
};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use sycamore::{
    prelude::{
        cloned, component, create_effect, create_signal, view, NodeRef, ReadSignal, Scope, Signal,
        View,
    },
    Prop,
};
use sycamore_router::{HistoryIntegration, Router, RouterProps};
use web_sys::Element;

// We don't want to bring in a styling library, so we do this the old-fashioned way!
// We're particualrly comprehensive with these because the user could *potentially* stuff things up with global rules
// https://medium.com/@jessebeach/beware-smushed-off-screen-accessible-text-5952a4c2cbfe
const ROUTE_ANNOUNCER_STYLES: &str = r#"
    margin: 0;
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

/// The properties that `on_route_change` takes. See the shell properties for the details for most of these.
#[derive(Debug, Clone)]
struct OnRouteChangeProps<G: Html> {
    locales: Rc<Locales>,
    container_rx: NodeRef<G>,
    router_state: RouterState,
    pss: PageStateStore,
    global_state: GlobalState,
    frozen_app: Rc<RefCell<Option<(FrozenApp, ThawPrefs)>>>,
    translations_manager: Rc<RefCell<ClientTranslationsManager>>,
    error_pages: Rc<ErrorPages<DomNode>>,
    initial_container: Option<Element>,
    is_first: Rc<Cell<bool>>,
    #[cfg(all(feature = "live-reload", debug_assertions))]
    live_reload_indicator: ReadSignal<bool>,
}

/// The function that runs when a route change takes place. This can also be run at any time to force the current page to reload.
fn on_route_change<G: Html>(
    verdict: RouteVerdict<TemplateNodeType>,
    OnRouteChangeProps {
        locales,
        container_rx,
        router_state,
        pss,
        global_state,
        frozen_app,
        translations_manager,
        error_pages,
        initial_container,
        is_first,
        #[cfg(all(feature = "live-reload", debug_assertions))]
        live_reload_indicator,
    }: OnRouteChangeProps<G>,
) {
    wasm_bindgen_futures::spawn_local(async move {
        let container_rx_elem = container_rx
            .get::<DomNode>()
            .unchecked_into::<web_sys::Element>();
        checkpoint("router_entry");
        match &verdict {
            // Perseus' custom routing system is tightly coupled to the template system, and returns exactly what we need for the app shell!
            // If a non-404 error occurred, it will be handled in the app shell
            RouteVerdict::Found(RouteInfo {
                path,
                template,
                locale,
                was_incremental_match,
            }) => {
                app_shell(
                    // TODO Make this not allocate so much
                    ShellProps {
                        path: path.clone(),
                        template: template.clone(),
                        was_incremental_match: *was_incremental_match,
                        locale: locale.clone(),
                        router_state,
                        translations_manager,
                        error_pages,
                        initial_container: initial_container.unwrap(),
                        container_rx_elem,
                        page_state_store: pss,
                        global_state,
                        frozen_app,
                        route_verdict: verdict,
                        is_first,
                        #[cfg(all(feature = "live-reload", debug_assertions))]
                        live_reload_indicator,
                    },
                )
                .await
            }
            // If the user is using i18n, then they'll want to detect the locale on any paths missing a locale
            // Those all go to the same system that redirects to the appropriate locale
            // Note that `container` doesn't exist for this scenario
            RouteVerdict::LocaleDetection(path) => detect_locale(path.clone(), &locales),
            // To get a translator here, we'd have to go async and dangerously check the URL
            // If this is an initial load, there'll already be an error message, so we should only proceed if the declaration is not `error`
            // BUG If we have an error in a subsequent load, the error message appears below the current page...
            RouteVerdict::NotFound => {
                checkpoint("not_found");
                if let InitialState::Error(ErrorPageData { url, status, err }) = get_initial_state()
                {
                    let initial_container = initial_container.unwrap();
                    // We need to move the server-rendered content from its current container to the reactive container (otherwise Sycamore can't work with it properly)
                    // If we're not hydrating, there's no point in moving anything over, we'll just fully re-render
                    #[cfg(feature = "hydrate")]
                    {
                        let initial_html = initial_container.inner_html();
                        container_rx_elem.set_inner_html(&initial_html);
                    }
                    initial_container.set_inner_html("");
                    // Make the initial container invisible
                    initial_container
                        .set_attribute("style", "display: none;")
                        .unwrap();
                    // Hydrate the error pages
                    // Right now, we don't provide translators to any error pages that have come from the server
                    error_pages.render_page(&url, status, &err, None, &container_rx_elem);
                } else {
                    // This is an error from navigating within the app (probably the dev mistyped a link...), so we'll clear the page
                    container_rx_elem.set_inner_html("");
                    error_pages.render_page("", 404, "not found", None, &container_rx_elem);
                }
            }
        };
    });
}

/// The properties that the router takes.
#[derive(Debug, Prop)]
pub struct PerseusRouterProps {
    /// The error pages the app is using.
    pub error_pages: ErrorPages<DomNode>,
    /// The locales settings the app is using.
    pub locales: Locales,
}

/// The Perseus router. This is used internally in the Perseus engine, and you shouldn't need to access this directly unless
/// you're building a custom engine. Note that this actually encompasses your entire app, and takes no child properties.
///
/// The `AppRoute` generic provided to this should be generated with `create_app_root!` and provided through Sycamore's
/// [higher-order components system](https://github.com/sycamore-rs/sycamore/blob/master/examples/higher-order-components/src/main.rs).
#[component]
pub fn perseus_router<G: Html, AppRoute: PerseusRoute<TemplateNodeType> + 'static>(
    cx: Scope,
    PerseusRouterProps {
        error_pages,
        locales,
    }: PerseusRouterProps,
) -> View<G> {
    // Get the root that the server will have injected initial load content into
    // This will be moved into a reactive `<div>` by the app shell
    // This is an `Option<Element>` until we know we aren't doing locale detection (in which case it wouldn't exist)
    let initial_container = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("#__perseus_content_initial")
        .unwrap();
    // And create a node reference that we can use as a handle to the reactive verison
    let container_rx = NodeRef::new();

    let translations_manager = Rc::new(RefCell::new(ClientTranslationsManager::new(&locales)));
    // Now that we've used the reference, put the locales in an `Rc`
    let locales = Rc::new(locales);
    // Get the error pages in an `Rc` so we aren't creating hundreds of them
    let error_pages = Rc::new(error_pages);

    // Create the router state we'll need
    let router_state = RouterState::default();
    // Create a page state store to use
    let pss = PageStateStore::default();
    // Create a new global state set to `None`, which will be updated and handled entirely by the template macro from here on
    let global_state = GlobalState::default();
    // Instantiate an empty frozen app that can persist across templates (with interior mutability for possible thawing)
    let frozen_app: Rc<RefCell<Option<(FrozenApp, ThawPrefs)>>> = Rc::new(RefCell::new(None));
    // Set up a mutable property for whether or not this is the first load of the first page
    let is_first = Rc::new(Cell::new(true));

    // If we're using live reload, set up an indicator so that our listening to the WebSocket at the top-level (where we don't have the render context that we need for freezing/thawing)
    // can signal the templates to perform freezing/thawing
    // It doesn't matter what the initial value is, this is just a flip-flop
    #[cfg(all(feature = "live-reload", debug_assertions))]
    let live_reload_indicator = create_signal(cx, true);

    // Create a derived state for the route announcement
    // We do this with an effect because we only want to update in some cases (when the new page is actually loaded)
    // We also need to know if it's the first page (because we don't want to announce that, screen readers will get that one right)
    let route_announcement = create_signal(cx, String::new());
    let mut is_first_page = true; // This is different from the first page load (this is the first page as a whole)
    create_effect(cx, move || {
        if let RouterLoadState::Loaded { path, .. } = &*router_state.get_load_state().get() {
            if is_first_page {
                // This is the first load event, so the next one will be for a new page (or at least something that we should announce, if this page reloads then the content will change, that would be from thawing)
                is_first_page = false;
            } else {
                // TODO Validate approach with reloading
                // A new page has just been loaded and is interactive (this event only fires after all rendering and hydration is complete)
                // Set the announcer to announce the title, falling back to the first `h1`, and then falling back again to the path
                let document = web_sys::window().unwrap().document().unwrap();
                // If the content of the provided element is empty, this will transform it into `None`
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
                    .map(make_empty_none)
                    .flatten();
                let announcement = match title {
                    Some(title) => title,
                    None => {
                        let first_h1 = document
                            .query_selector("h1")
                            .unwrap()
                            .map(make_empty_none)
                            .flatten();
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

    // Set up the function we'll call on a route change
    // Set up the properties for the function we'll call in a route change
    let on_route_change_props = OnRouteChangeProps {
        locales,
        container_rx: container_rx.clone(),
        router_state: router_state.clone(),
        pss,
        global_state,
        frozen_app,
        translations_manager,
        error_pages,
        initial_container,
        is_first,
        #[cfg(all(feature = "live-reload", debug_assertions))]
        live_reload_indicator: *live_reload_indicator,
    };

    // Listen for changes to the reload commander and reload as appropriate
    let reload_commander = router_state.reload_commander.clone();
    create_effect(cx, move || {
        // This is just a flip-flop, but we need to add it to the effect's dependencies
        let _ = reload_commander.get();
        // Get the route verdict and re-run the function we use on route changes
        let verdict = match router_state.get_last_verdict() {
            Some(verdict) => verdict,
            // If the first page hasn't loaded yet, terminate now
            None => return,
        };
        on_route_change(verdict, on_route_change_props.clone());
    });

    // TODO State thawing in HSR
    // If live reloading is enabled, connect to the server now
    // This doesn't actually perform any reloading or the like, it just signals places that have access to the render context to do so (because we need that for state freezing/thawing)
    #[cfg(all(feature = "live-reload", debug_assertions))]
    crate::state::connect_to_reload_server(live_reload_indicator);

    view! { cx,

        Router {
            integration: HistoryIntegration::new(),
            view: |cx, route: &ReadSignal<AppRoute>| {
                // Sycamore's reactivity is broken by a future, so we need to explicitly add the route to the reactive dependencies here
                // We do need the future though (otherwise `container_rx` doesn't link to anything until it's too late)
                create_effect(cx, move || {
                    let verdict = route.get().get_verdict().clone();
                    on_route_change(verdict, on_route_change_props.clone());
                });

                // This template is reactive, and will be updated as necessary
                // However, the server has already rendered initial load content elsewhere, so we move that into here as well in the app shell
                // The main reason for this is that the router only intercepts click events from its children
                view! { cx,
                    div {
                        div(id="__perseus_content_rx", class="__perseus_content", ref=container_rx) {}
                        p(id = "__perseus_route_announcer", aria_live = "assertive", role = "alert", style = ROUTE_ANNOUNCER_STYLES) { (route_announcement.get()) }
                    }
                }
            }
        }
    }
}
