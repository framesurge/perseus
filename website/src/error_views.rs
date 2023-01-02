use perseus::{errors::ClientError, prelude::*};
use sycamore::prelude::*;

// This site will be exported statically, so we only have control over 404 pages
// for broken links in the site itself
pub fn get_error_views<G: Html>() -> ErrorViews<G> {
    ErrorViews::new(|cx, err, _err_info, _err_pos| {
        match err {
            // Errors from the server, like 404s; these are best displayed over the whole
            // page
            ClientError::ServerError {
                status,
                // This is fully formatted with newlines and tabs for the error and its causes
                message: _,
            } => match status {
                // This one is usually handled separately
                404 => (
                    view! { cx,
                        title { "Page not found" }
                    },
                    not_found_page(cx),
                ),
                // If the status is 4xx, it's a client-side problem (which is weird, and might
                // indicate tampering)
                _ if (400..500).contains(&status) => (
                    view! { cx,
                        title { "Error" }
                    },
                    view! { cx,
                        p { "There was something wrong with the last request, please try reloading the page." }
                    },
                ),
                // 5xx is a server error
                _ => (
                    view! { cx,
                        title { "Error" }
                    },
                    view! { cx,
                        p { "Sorry, our server experienced an internal error. Please try reloading the page." }
                    },
                ),
            },
            // A panic (yes, you can handle them here!). After this error is displayed, the entire
            // app will terminate, so buttons or other reactive elements are pointless.
            //
            // The argument here is the formatted panic message.
            ClientError::Panic(_) => (
                view! { cx,
                    title { "Critical error" }
                },
                view! { cx,
                    p { "Sorry, but a critical internal error has occurred. This has been automatically reported to our team, who'll get on it as soon as possible. In the mean time, please try reloading the page." }
                },
            ),
            // Network errors (but these could be caused by unexpected server rejections)
            ClientError::FetchError(_) => (
                view! { cx,
                    title { "Error" }
                },
                view! { cx,
                    p { "A network error occurred, do you have an internet connection? (If you do, try reloading the page.)" }
                },
            ),

            _ => (
                view! { cx,
                    title { "Error" }
                },
                view! { cx,
                    p { (format!("An internal error has occurred: '{}'.", err)) }
                },
            ),
        }
    })
}

fn not_found_page<G: Html>(cx: Scope) -> View<G> {
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
                            a(class = "underline text-indigo-500", href = "") { "Home" }
                        }
                        li {
                            a(class = "underline text-indigo-500", href = "docs") { "Docs" }
                        }
                        li {
                            a(class = "underline text-indigo-500", href = "comparisons") { "Comparisons" }
                        }
                        li {
                            a(class = "underline text-indigo-500", href = "plugins") { "Plugins" }
                        }
                    }
                }
            }
        }
    }
}
