mod error_pages;
mod global_state;
mod templates;

use perseus::{Html, PerseusApp};

#[perseus::main(perseus_warp::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .template(crate::templates::about::get_template())
        .error_pages(crate::error_pages::get_error_pages())
        .global_state_creator(crate::global_state::get_global_state_creator())
}
