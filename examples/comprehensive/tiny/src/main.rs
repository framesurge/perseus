use perseus::prelude::*;
use sycamore::prelude::*;

fn index_view() -> View {
    view! {
        div {
            h1 { "Hello World!" }
            p { "This is a tiny Perseus example." }
        }
    }
}

#[engine_only_fn]
fn head() -> View {
    view! {
        title { "Tiny Perseus Example" }
    }
}

#[perseus::main(perseus_axum::dflt_server)]
pub fn main() -> PerseusApp {
    PerseusApp::new()
        .template(
            Template::build("index")
                .view(index_view)
                .head(head)
                .build(),
        )
        // This forces Perseus to use the development defaults in production, which just
        // lets you easily deploy this app. In a real app, you should always provide your own
        // error pages!
        .error_views(ErrorViews::unlocalized_development_default())
}
