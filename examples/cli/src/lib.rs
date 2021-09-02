mod pages;
mod error_pages;

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
define_app!{
    root: "#root",
    route: Route,
    router: {
        Route::Index => [
            "index".to_string(),
            pages::index::template_fn()
        ],
        Route::About => [
            "about".to_string(),
            pages::about::template_fn()
        ]
    },
    error_pages: crate::error_pages::get_error_pages(),
    templates: [
        crate::pages::index::get_page::<G>(),
        crate::pages::about::get_page::<G>()
    ]
    // config_manager: perseus::FsConfigManager::new()
}
