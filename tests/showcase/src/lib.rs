mod error_pages;
mod templates;

use perseus::define_app;

define_app! {
    templates: [
        crate::templates::index::get_template::<G>(),
        crate::templates::about::get_template::<G>(),
        crate::templates::new_post::get_template::<G>(),
        crate::templates::post::get_template::<G>(),
        crate::templates::ip::get_template::<G>(),
        crate::templates::time_root::get_template::<G>(),
        crate::templates::time::get_template::<G>(),
        crate::templates::amalgamation::get_template::<G>(),
        crate::templates::router_state::get_template::<G>()
    ],
    error_pages: crate::error_pages::get_error_pages(),
    locales: {
        default: "en-US",
        other: ["fr-FR", "es-ES"]
    }
}
