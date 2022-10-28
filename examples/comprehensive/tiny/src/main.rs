use perseus::prelude::*;
use sycamore::prelude::*;

#[perseus::main(perseus_warp::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(|| {
            Template::new("index").template(|cx, _| {
                view! { cx,
                    p { "Hello World!" }
                }
            })
        })
        .error_pages(|| ErrorPages::new(
            |cx, url, status, err, _| view! { cx,
                p { (format!("An error with HTTP code {} occurred at '{}': '{}'.", status, url, err)) }
            },
            |cx, _, _, _, _| view! { cx,
                title { "Error" }
            })
        )
}
