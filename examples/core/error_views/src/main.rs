mod error_views;
mod templates;

use perseus::prelude::*;

#[perseus::main(perseus_warp::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        // The same convention of a function to return the needed `struct` is
        // used for both templates and error views
        .error_views(crate::error_views::get_error_views())
}
