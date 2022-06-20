use perseus::{Html, PerseusApp, Template, ErrorPages};
use sycamore::view;

#[perseus::main]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(|| {
            Template::new("index").template(|cx, _| {
                view! { cx,
                        p { "Hello World!" }
                }
            })
        })
        .error_pages(|| ErrorPages::new(|cx, url, status, err, _| view! { cx,
            p { (format!("An error with HTTP code {} occurred at '{}': '{}'.", status, url, err)) }
        }))
}
