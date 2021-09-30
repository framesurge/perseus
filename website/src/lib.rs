mod components;
mod error_pages;
mod templates;

use perseus::define_app;

define_app! {
    templates: [
        templates::index::get_template::<G>()
    ],
    error_pages: error_pages::get_error_pages(),
    locales: {
        default: "en-US",
        other: []
    }
}
