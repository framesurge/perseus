use perseus::{ErrorPages, Html, i18n::Translator};
use sycamore::prelude::*;
use std::rc::Rc;

// This site will be exported statically, so we only have control over 404 pages
// for broken links in the site itself
pub fn get_error_pages<G: Html>() -> ErrorPages<G> {
    let mut error_pages = ErrorPages::new(
        |cx, url, status, err, _| {
            view! { cx,
                p { (format!("An error with HTTP code {} occurred at '{}': '{}'.", status, url, err)) }
            }
        },
        |cx, _, _, _, _| {
            view! { cx,
                title { "Error" }
            }
        },
    );
    error_pages.add_page(
        404,
        not_found_page,
        |cx, _, _, _, _| {
            view! { cx,
                title { "Not Found" }
            }
        },
    );

    error_pages
}

fn not_found_page<G: Html>(cx: Scope, _url: String, _status: u16, _err: String, _translator: Option<Rc<Translator>>) -> View<G> {
    view! { cx,
        div(class = "flex flex-col justify-center items-center h-screen") {
            main(class = "flex flex-col border border-black rounded-lg max-w-xl m-4") {
                h3(class = "text-2xl font-bold w-full pb-4 border-b border-black my-4") {
                    span(class = "pl-4") { "Page not found!" }
                }
                div(class = "p-4 pt-0 my-4") {
                    span { "That page doesn't seem to exist. If you came here from a link elsewhere on the site, you should report this as a bug " }
                    a(class = "underline text-indigo-500", href = "https://github.com/framesurge/perseus/issues/new/choose") { "here" }
                    span { ". If you came here another website, or a search engine, this page probably existed once, but has since been moved. Here are some pages you might like to try instead:" }
                    ul(class = "pl-6 mt-1 w-full") {
                        li {
                            a(class = "underline text-indigo-500", href = "/") { "Home" }
                        }
                        li {
                            a(class = "underline text-indigo-500", href = "/docs") { "Docs" }
                        }
                        li {
                            a(class = "underline text-indigo-500", href = "/comparisons") { "Comparisons" }
                        }
                        li {
                            a(class = "underline text-indigo-500", href = "/plugins") { "Plugins" }
                        }
                    }
                }
            }
        }
    }
}
