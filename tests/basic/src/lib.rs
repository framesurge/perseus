mod error_pages;
mod templates;

use perseus::define_app;

define_app! {
    templates: [
        crate::templates::index::get_template::<G>(),
        crate::templates::about::get_template::<G>()
    ],
    error_pages: crate::error_pages::get_error_pages(),
    static_aliases: {
        "/test.txt" => "static/test.txt"
    }
}
