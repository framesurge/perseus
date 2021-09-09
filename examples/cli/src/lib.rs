mod error_pages;
mod pages;

use perseus::define_app;

#[derive(perseus::Route)]
pub enum Route {
    #[to("/")]
    Index,
    #[to("/about")]
    About,
    #[not_found]
    NotFound,
}
define_app! {
    root: "#root",
    route: Route,
    router: {
        Route::Index => [
            "index".to_string(),
            pages::index::template_fn(),
            "en-US".to_string()
        ],
        Route::About => [
            "about".to_string(),
            pages::about::template_fn(),
            "en-US".to_string()
        ]
    },
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
