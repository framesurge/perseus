use app::{get_error_pages, get_locales, get_templates_map, APP_ROOT};
use perseus::router::{RouteInfo, RouteVerdict};
use perseus::shell::get_render_cfg;
use perseus::{app_shell, create_app_route, detect_locale, ClientTranslationsManager, DomNode};
use std::cell::RefCell;
use std::rc::Rc;
use sycamore::prelude::{template, StateHandle};
use sycamore_router::{HistoryIntegration, Router, RouterProps};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// The entrypoint into the app itself. This will be compiled to Wasm and actually executed, rendering the rest of the app.
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // Panics should always go to the console
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    // Get the root (for the router) we'll be injecting page content into
    let root = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector(APP_ROOT)
        .unwrap()
        .unwrap();

    // Create a mutable translations manager to control caching
    let translations_manager =
        Rc::new(RefCell::new(ClientTranslationsManager::new(&get_locales())));
    // Get the error pages in an `Rc` so we aren't creating hundreds of them
    let error_pages = Rc::new(get_error_pages());

    // Create the router we'll use for this app, based on the user's app definition
    create_app_route! {
        name => AppRoute,
        // The render configuration is injected verbatim into the HTML shell, so it certainly should be present
        render_cfg => get_render_cfg().expect("render configuration invalid or not injected"),
        templates => get_templates_map(),
        locales => get_locales()
    }

    sycamore::render_to(
        || {
            template! {
                Router(RouterProps::new(HistoryIntegration::new(), move |route: StateHandle<AppRoute<DomNode>>| {
                    match &route.get().as_ref().0 {
                        // Perseus' custom routing system is tightly coupled to the template system, and returns exactly what we need for the app shell!
                        RouteVerdict::Found(RouteInfo {
                            path,
                            template,
                            locale
                        }) => app_shell(
                            path.clone(),
                            template.clone(),
                            locale.clone(),
                            // We give the app shell a translations manager and let it get the `Rc<Translator>` itself (because it can do async safely)
                            Rc::clone(&translations_manager),
                            Rc::clone(&error_pages)
                        ),
                        // If the user is using i18n, then they'll want to detect the locale on any paths missing a locale
                        // Those all go to the same system that redirects to the appropriate locale
                        RouteVerdict::LocaleDetection(path) => detect_locale(path.clone(), get_locales()),
                        // We handle the 404 for the user for convenience
                        // To get a translator here, we'd have to go async and dangerously check the URL
                        RouteVerdict::NotFound => get_error_pages().get_template_for_page("", &404, "not found", None),
                    }
                }))
            }
        },
        &root,
    );

    Ok(())
}
