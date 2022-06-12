mod error_pages;
mod templates;

use perseus::{Html, PerseusApp};

pub fn get_app<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template)
        .template(crate::templates::about::get_template)
        .error_pages(crate::error_pages::get_error_pages)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn main() -> perseus::ClientReturn {
    use perseus::run_client;

    run_client(get_app())
}
