pub mod errors;
pub mod pages;
mod shell;
pub mod serve;
pub mod render_cfg;
pub mod config_manager;
pub mod page;
pub mod build;

use sycamore::prelude::*;
use sycamore_router::{Route, BrowserRouter};
use wasm_bindgen::prelude::*;

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
                        AppRoute::Index => app_shell!({
                            name => "index",
                            props => pages::index::IndexPageProps,
                            template => |props: Option<pages::index::IndexPageProps>| template! {
                                pages::index::IndexPage(props.unwrap())
                            },
                        }),
                        AppRoute::About => app_shell!({
                            name => "about",
                            template => |_: Option<()>| template! {
                                pages::about::AboutPage()
                            },
                        }),
                        AppRoute::Post { slug } => app_shell!({
                            name => &format!("post/{}", slug),
                            props => pages::post::PostPageProps,
                            template => |props: Option<pages::post::PostPageProps>| template! {
                                pages::post::PostPage(props.unwrap())
                            },
                        }),
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
