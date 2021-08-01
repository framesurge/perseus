pub mod pages;

use sycamore::prelude::*;
use sycamore_router::{Route, BrowserRouter};
use wasm_bindgen::prelude::*;
use perseus::shell::app_shell;

// Define our routes
#[derive(Route)]
enum AppRoute {
    #[to("/")]
    Index,
    #[to("/about")]
    About,
    #[to("/post/new")]
    NewPost,
    #[to("/post/<slug>")]
    Post {
        slug: String
    },
    #[to("/ip")]
    Ip,
    #[not_found]
    NotFound
}

// This is deliberately purely client-side rendered
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    // Get the root (for the router) we'll be injecting page content into
	let root = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("#_perseus_root")
        .unwrap()
        .unwrap();

	sycamore::render_to(
        ||
            template! {
                BrowserRouter(|route: AppRoute| {
                    match route {
                        AppRoute::Index => app_shell(
                            "index".to_string(),
                            pages::index::template_fn()
                        ),
                        AppRoute::About => app_shell(
                            "about".to_string(),
                            pages::about::template_fn()
                        ),
                        AppRoute::Post { slug } => app_shell(
                            format!("post/{}", slug),
                            pages::post::template_fn()
                        ),
                        AppRoute::NewPost => app_shell(
                            "post/new".to_string(),
                            pages::new_post::template_fn()
                        ),
                        AppRoute::Ip => app_shell(
                            "ip".to_string(),
                            pages::ip::template_fn()
                        ),
                        AppRoute::NotFound => template! {
                            p {"Not Found."}
                        }
                    }
                })
            },
        &root
    );

	Ok(())
}
