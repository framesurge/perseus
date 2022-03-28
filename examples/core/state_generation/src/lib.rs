mod error_pages;
mod templates;

use perseus::{Html, PerseusApp};

#[perseus::main]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::build_state::get_template)
        .template(crate::templates::build_paths::get_template)
        .template(crate::templates::request_state::get_template)
        .template(crate::templates::incremental_generation::get_template)
        .template(crate::templates::revalidation::get_template)
        .template(crate::templates::revalidation_and_incremental_generation::get_template)
        .template(crate::templates::amalgamation::get_template)
        .error_pages(crate::error_pages::get_error_pages)
}
