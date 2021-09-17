mod error_pages;
mod templates;

use perseus::define_app;

define_app! {
    root: "#root",
    error_pages: crate::error_pages::get_error_pages(),
    templates: [
        crate::templates::about::get_template::<G>(),
        crate::templates::index::get_template::<G>()
    ],
    locales: {
        default: "en-US",
        other: ["fr-FR", "es-ES"]
    }
}
