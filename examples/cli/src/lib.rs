mod error_pages;
mod pages;

use perseus::define_app;

define_app! {
    root: "#root",
    error_pages: crate::error_pages::get_error_pages(),
    templates: [
        crate::pages::index::get_page::<G>(),
        crate::pages::about::get_page::<G>()
    ],
    locales: {
        default: "en-US",
        other: [],
        no_i18n: true
    }
}
