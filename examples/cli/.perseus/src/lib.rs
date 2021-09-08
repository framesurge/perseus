use app::{get_error_pages, get_locales, match_route, AppRoute, APP_ROUTE};
use perseus::{app_shell, ClientTranslationsManager};
use std::cell::RefCell;
use std::rc::Rc;
use sycamore::prelude::template;
use sycamore_router::BrowserRouter;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// The entrypoint into the app itself. This will be compiled to WASM and actually executed, rendering the rest of the app.
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // If we're in development, panics should go to the console
    if cfg!(debug_assertions) {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }
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

    sycamore::render_to(
        || {
            template! {
                BrowserRouter(move |route: AppRoute| {
                    match route {
                        // We handle the 404 for the user for convenience
                        AppRoute::NotFound => get_error_pages().get_template_for_page("", &404, "not found"),
                        // All other routes are based on the user's given statements
                        _ => {
                            let (name, render_fn, locale) = match_route(route);

                            app_shell(
                                name,
                                render_fn,
                                locale,
                                // We give the app shell a translations manager and let it get the `Rc<Translator>` (because it can do async safely)
                                Rc::clone(&translations_manager),
                                Rc::clone(&error_pages)
                            )
                        }
                    }
                })
            }
        },
        &root,
    );

    Ok(())
}
