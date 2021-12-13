pub mod app;

use crate::app::{get_app_root, get_error_pages, get_locales, get_plugins, get_templates_map};
use perseus::{
    checkpoint, create_app_route,
    internal::{
        error_pages::ErrorPageData,
        i18n::{detect_locale, ClientTranslationsManager},
        router::{RouteInfo, RouteVerdict},
        shell::{app_shell, get_initial_state, get_render_cfg, InitialState},
    },
    plugins::PluginAction,
    templates::TemplateNodeType,
    DomNode,
};
use std::cell::RefCell;
use std::rc::Rc;
use sycamore::prelude::{cloned, create_effect, view, NodeRef, ReadSignal};
use sycamore_router::{HistoryIntegration, Router, RouterProps};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// The entrypoint into the app itself. This will be compiled to Wasm and actually executed, rendering the rest of the app.
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let plugins = get_plugins::<DomNode>();

    checkpoint("begin");
    // Panics should always go to the console
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    plugins
        .functional_actions
        .client_actions
        .start
        .run((), plugins.get_plugin_data());
    checkpoint("initial_plugins_complete");

    // Get the root we'll be injecting the router into
    let root = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector(&format!("#{}", get_app_root(&plugins)))
        .unwrap()
        .unwrap();

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

    // Create a mutable translations manager to control caching
    let locales = get_locales(&plugins);
    let translations_manager = Rc::new(RefCell::new(ClientTranslationsManager::new(&locales)));
    // Get the error pages in an `Rc` so we aren't creating hundreds of them
    let error_pages = Rc::new(get_error_pages(&plugins));

    // Create the router we'll use for this app, based on the user's app definition
    create_app_route! {
        name => AppRoute,
        // The render configuration is injected verbatim into the HTML shell, so it certainly should be present
        render_cfg => &get_render_cfg().expect("render configuration invalid or not injected"),
        // TODO avoid unnecessary allocation here (major problem!)
        // The `G` parameter is ambient here for `RouteVerdict`
        templates => &get_templates_map::<G>(&get_plugins()),
        locales => &get_locales::<G>(&get_plugins())
    }

    // Put the locales into an `Rc` so we can use them in locale detection (which is inside a future)
    let locales = Rc::new(locales);

    sycamore::render_to(
        move || {
            view! {
                Router(RouterProps::new(HistoryIntegration::new(), move |route: ReadSignal<AppRoute<TemplateNodeType>>| {
                    create_effect(cloned!((container_rx) => move || {
                        // Sycamore's reactivity is broken by a future, so we need to explicitly add the route to the reactive dependencies here
                        // We do need the future though (otherwise `container_rx` doesn't link to anything until it's too late)
                        let _ = route.get();
                        wasm_bindgen_futures::spawn_local(cloned!((locales, route, container_rx, translations_manager, error_pages, initial_container) => async move {
                            let container_rx_elem = container_rx.get::<DomNode>().unchecked_into::<web_sys::Element>();
                            checkpoint("router_entry");
                            match &route.get().as_ref().0 {
                                // Perseus' custom routing system is tightly coupled to the template system, and returns exactly what we need for the app shell!
                                // If a non-404 error occurred, it will be handled in the app shell
                                RouteVerdict::Found(RouteInfo {
                                    path,
                                    template,
                                    locale,
                                    was_incremental_match
                                }) => app_shell(
                                    path.clone(),
                                    (template.clone(), *was_incremental_match),
                                    locale.clone(),
                                    // We give the app shell a translations manager and let it get the `Rc<Translator>` itself (because it can do async safely)
                                    Rc::clone(&translations_manager),
                                    Rc::clone(&error_pages),
                                    (initial_container.unwrap().clone(), container_rx_elem.clone())
                                ).await,
                                // If the user is using i18n, then they'll want to detect the locale on any paths missing a locale
                                // Those all go to the same system that redirects to the appropriate locale
                                // Note that `container` doesn't exist for this scenario
                                RouteVerdict::LocaleDetection(path) => detect_locale(path.clone(), &locales),
                                // To get a translator here, we'd have to go async and dangerously check the URL
                                // If this is an initial load, there'll already be an error message, so we should only proceed if the declaration is not `error`
                                // BUG If we have an error in a subsequent load, the error message appears below the current page...
                                RouteVerdict::NotFound => {
                                    checkpoint("not_found");
                                    if let InitialState::Error(ErrorPageData { url, status, err }) = get_initial_state() {
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
                                        initial_container.set_attribute("style", "display: none;").unwrap();
                                        // Hydrate the error pages
                                        // Right now, we don't provide translators to any error pages that have come from the server
                                        error_pages.render_page(&url, &status, &err, None, &container_rx_elem);
                                    } else {
                                        // This is an error from navigating within the app (probably the dev mistyped a link...), so we'll clear the page
                                        container_rx_elem.set_inner_html("");
                                        error_pages.render_page("", &404, "not found", None, &container_rx_elem);
                                    }
                                },
                            };
                        }));
                    }));
                    // This template is reactive, and will be updated as necessary
                    // However, the server has already rendered initial load content elsewhere, so we move that into here as well in the app shell
                    // The main reason for this is that the router only intercepts click events from its children
                    view! {
                        div(id="__perseus_content_rx", class="__perseus_content", ref=container_rx) {}
                    }
                }))
            }
        },
        &root,
    );

    Ok(())
}
