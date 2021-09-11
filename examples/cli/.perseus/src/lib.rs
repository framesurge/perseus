use app::{get_error_pages, get_locales, get_routes, APP_ROUTE};
use perseus::router::{RouteInfo, RouteVerdict};
use perseus::{app_shell, ClientTranslationsManager, DomNode};
use std::cell::RefCell;
use std::rc::Rc;
use sycamore::prelude::template;
use sycamore::rx::{ContextProvider, ContextProviderProps};
use sycamore_router::BrowserRouter;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// The entrypoint into the app itself. This will be compiled to WASM and actually executed, rendering the rest of the app.
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // Panics should always go to the console
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    // Get the root (for the router) we'll be injecting page content into
    let root = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector(APP_ROUTE)
        .unwrap()
        .unwrap();

    // Create a mutable translations manager to control caching
    let translations_manager =
        Rc::new(RefCell::new(ClientTranslationsManager::new(&get_locales())));
    // Get the error pages in an `Rc` so we aren't creating hundreds of them
    let error_pages = Rc::new(get_error_pages());
    // Get the routes in an `Rc` as well
    let routes = Rc::new(get_routes::<DomNode>());

    sycamore::render_to(
        || {
            template! {
                // We provide the routes in context (can't provide them directly because of Sycamore trait constraints)
                // BUG: context doesn't exist when link clicked first time, works second time...
                ContextProvider(ContextProviderProps {
                    value: Rc::clone(&routes),
                    children: || template! {
                        BrowserRouter(move |route: RouteVerdict<DomNode>| {
                            match route {
                                // Perseus' custom routing system is tightly coupled to the template system, and returns exactly what we need for the app shell!
                                RouteVerdict::Found(RouteInfo {
                                    path,
                                    template_fn,
                                    locale
                                }) => app_shell(
                                    path,
                                    template_fn,
                                    locale,
                                    // We give the app shell a translations manager and let it get the `Rc<Translator>` (because it can do async safely)
                                    Rc::clone(&translations_manager),
                                    Rc::clone(&error_pages)
                                ),
                                // If the user is using i18n, then they'll want to detect the locale on any paths missing a locale
                                // Those all go to the same system that redirects to the appropriate locale
                                // TODO locale detection
                                RouteVerdict::LocaleDetection(_) => get_error_pages().get_template_for_page("", &400, "locale detection not yet supported", None),
                                // We handle the 404 for the user for convenience
                                // To get a translator here, we'd have to go async and dangerously check the URL
                                RouteVerdict::NotFound => get_error_pages().get_template_for_page("", &404, "not found", None),
                            }
                        })
                    }
                })
            }
        },
        &root,
    );

    Ok(())
}
