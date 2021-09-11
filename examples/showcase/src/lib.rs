mod error_pages;
mod templates;

use perseus::define_app;

define_app! {
    root: "#root",
    error_pages: crate::error_pages::get_error_pages(),
    templates: [
        "/" => crate::templates::index::get_template::<G>(),
        "/about" => crate::templates::about::get_template::<G>(),
        "/post/new" => crate::templates::new_post::get_template::<G>(),
        // BUG: Sycamore doesn't support dynamic paths before dynamic segments (https://github.com/sycamore-rs/sycamore/issues/228)
        "/post/<slug..>" => crate::templates::post::get_template::<G>(),
        "/ip" => crate::templates::ip::get_template::<G>(),
        "/time" => crate::templates::time_root::get_template::<G>(),
        "/timeisr/<slug>" => crate::templates::time::get_template::<G>(),
        "/amalgamation" => crate::templates::amalgamation::get_template::<G>()
    ],
    locales: {
        default: "en-US",
        other: ["fr-FR", "es-ES"]
    }
}
