use app::{get_error_pages, match_route, AppRoute, APP_ROUTE};
use perseus::app_shell;
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

    sycamore::render_to(
        || {
            template! {
                BrowserRouter(|route: AppRoute| {
                    // TODO improve performance rather than naively copying error pages for every template
                    match route {
                        // We handle the 404 for the user for convenience
                        AppRoute::NotFound => get_error_pages().get_template_for_page("", &404, "not found"),
                        // All other routes are based on the user's given statements
                        _ => {
                            let (name, render_fn) = match_route(route);
                            app_shell(
                                name,
                                render_fn,
                                get_error_pages()
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
