mod error_pages;
mod templates;

use perseus::{Html, PerseusApp};

#[perseus::main]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template)
        .error_pages(crate::error_pages::get_error_pages)
}
