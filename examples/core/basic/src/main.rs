mod error_views;
mod templates;

use perseus::prelude::*;

#[perseus::main(perseus_integration::dflt_server)]
pub fn main() -> PerseusApp {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .template(crate::templates::about::get_template())
        .error_views(crate::error_views::get_error_views())
}
