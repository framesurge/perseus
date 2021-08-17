pub mod pages;

use perseus::shell::{app_shell, ErrorPages};
use sycamore::prelude::*;
use sycamore_router::{BrowserRouter, Route};
use wasm_bindgen::prelude::*;

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
    Post { slug: String },
    #[to("/ip")]
    Ip,
    #[to("/time")]
    TimeRoot,
    #[to("/timeisr/<slug>")]
    Time { slug: String },
    #[not_found]
    NotFound,
}

fn get_error_pages() -> ErrorPages {
    let mut error_pages = ErrorPages::new(Box::new(|_, _, _| {
        template! {
            p { "Another error occurred." }
        }
    }));
    error_pages.add_page(
        404,
        Box::new(|_, _, _| {
            template! {
                p { "Page not found." }
            }
        }),
    );
    error_pages.add_page(
        400,
        Box::new(|_, _, _| {
            template! {
                p { "Client error occurred..." }
            }
        }),
    );

    error_pages
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
        || {
            template! {
                BrowserRouter(|route: AppRoute| {
                    // TODO improve performance rather than naively copying error pages for every template
                    match route {
                        AppRoute::Index => app_shell(
                            "index".to_string(),
                            pages::index::template_fn(),
                            get_error_pages()
                        ),
                        AppRoute::About => app_shell(
                            "about".to_string(),
                            pages::about::template_fn(),
                            get_error_pages()
                        ),
                        AppRoute::Post { slug } => app_shell(
                            format!("post/{}", slug),
                            pages::post::template_fn(),
                            get_error_pages()
                        ),
                        AppRoute::NewPost => app_shell(
                            "post/new".to_string(),
                            pages::new_post::template_fn(),
                            get_error_pages()
                        ),
                        AppRoute::Ip => app_shell(
                            "ip".to_string(),
                            pages::ip::template_fn(),
                            get_error_pages()
                        ),
                        AppRoute::Time { slug } => app_shell(
                            format!("timeisr/{}", slug),
                            pages::time::template_fn(),
                            get_error_pages()
                        ),
                        AppRoute::TimeRoot => app_shell(
                            "time".to_string(),
                            pages::time_root::template_fn(),
                            get_error_pages()
                        ),
                        AppRoute::NotFound => template! {
                            p {"Not Found."}
                        }
                    }
                })
            }
        },
        &root,
    );

    Ok(())
}
