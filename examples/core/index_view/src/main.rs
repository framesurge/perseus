mod error_pages;
mod templates;

use perseus::{Html, PerseusApp, PerseusRoot};

#[perseus::main(perseus_warp::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .template(crate::templates::about::get_template())
        .error_pages(crate::error_pages::get_error_pages())
        .index_view(|cx| {
            sycamore::view! { cx,
                // We don't need a `<!DOCTYPE html>`, that's added automatically by Perseus (though that can be overridden if you really want by using `.index_view_str()`)
                // We need a `<head>` and a `<body>` at the absolute minimum for Perseus to work properly (otherwise certain script injections will fail)
                head {
                    title { "Perseus Index View Example" }
                }
                body {
                    // This creates an element into which our app will be interpolated
                    // This uses a few tricks internally beyond the classic `<div id="root">`, so we use this wrapper for convenience
                    PerseusRoot()
                    // Because this is in the index view, this will be below every single one of our pages
                    // Note that elements in here can't be selectively removed from one page, it's all-or-nothing in the index view (it wraps your whole app)
                    // Note also that this won't be reloaded, even when the user switches pages
                    footer { "This is a footer!" }
                }
            }
        })
}
