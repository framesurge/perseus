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
    #[to("/post/<slug>")]
    Post {
        slug: String
    },
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
                            Box::new(|props: Option<pages::index::IndexPageProps>| template! {
                                pages::index::IndexPage(props.unwrap())
                            })
                        ),
                        AppRoute::About => app_shell(
                            "about".to_string(),
                            Box::new(|_: Option<()>| template! {
                                pages::about::AboutPage()
                            })
                        ),
                        AppRoute::Post { slug } => app_shell(
                            format!("post/{}", slug),
                            Box::new(|props: Option<pages::post::PostPageProps>| template! {
                                pages::post::PostPage(props.unwrap())
                            })
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
