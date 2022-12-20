use perseus::prelude::*;
use sycamore::prelude::*;

#[perseus::main(perseus_warp::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(
            Template::build("index")
                .view(|cx| {
                    view! { cx,
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
