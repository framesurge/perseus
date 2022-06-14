use perseus::{Html, PerseusApp, Template};
use sycamore::view;

#[perseus::main(perseus_warp::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new().template(|| {
        Template::new("index").template(|cx, _| {
            view! { cx,
                p { "Hello World!" }
            }
        })
    })
}
