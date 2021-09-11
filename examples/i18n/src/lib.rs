mod error_pages;
mod templates;

use perseus::define_app;

define_app! {
    root: "#root",
    error_pages: crate::error_pages::get_error_pages(),
    templates: [
        "/about" => crate::templates::about::get_template::<G>(),
        // Note that the index page comes last, otherwise locale detection for `/about` matches `about` as a locale
        "/" => crate::templates::index::get_template::<G>()
    ],
    locales: {
        default: "en-US",
        other: ["fr-FR", "es-ES"]
    }
}
