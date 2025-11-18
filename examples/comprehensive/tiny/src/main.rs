use perseus::prelude::*;
use sycamore::prelude::*;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main() -> PerseusApp {
    PerseusApp::new()
        .template(
            Template::build("index")
                .view(|| {
                    view! {
                        p { "Hello World!" }
                    }
                })
                .build(),
        )
        // This forces Perseus to use the development defaults in production, which just
        // lets you easily deploy this app. In a real app, you should always provide your own
        // error pages!
        .error_views(ErrorViews::unlocalized_development_default())
}
