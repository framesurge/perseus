mod error_pages;
mod templates;

use perseus::{Html, PerseusApp};


/// Replace `<perseus_integration>` with an integration of your choice.
/// Examples of supported integrations:
/// - perseus_warp (recommended)
/// - perseus-actix-web
/// - perseus-axum
#[perseus::main(<perseus_integration>::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template)
        .error_pages(crate::error_pages::get_error_pages)
}
