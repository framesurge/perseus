mod error_pages;
mod templates;

use perseus::define_app;

#[derive(perseus::Route)]
pub enum Route {
    #[to("/<locale>")]
    Index { locale: String },
    #[to("/<locale>/about")]
    About { locale: String },
    #[not_found]
    NotFound,
}

define_app! {
    root: "#root",
    route: Route,
    router: {
        Route::Index { locale } => [
            "index".to_string(),
            templates::index::template_fn(),
            locale
        ],
        Route::About { locale } => [
            "about".to_string(),
            templates::about::template_fn(),
            locale
        ]
    },
    error_pages: crate::error_pages::get_error_pages(),
    templates: [
        crate::templates::index::get_template::<G>(),
        crate::templates::about::get_template::<G>()
    ],
    locales: {
        default: "en-US",
        other: ["fr-FR", "es-ES"]
    }
}
