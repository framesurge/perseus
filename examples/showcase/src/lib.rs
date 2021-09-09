mod error_pages;
mod templates;

use perseus::define_app;

#[derive(perseus::Route)]
pub enum Route {
    #[to("/<locale>")]
    Index { locale: String },
    #[to("/<locale>/about")]
    About { locale: String },
    #[to("/<locale>/post/new")]
    NewPost { locale: String },
    // BUG: Sycamore doesn't support dynamic paths before dynamic segments (https://github.com/sycamore-rs/sycamore/issues/228)
    #[to("/post/<slug..>")]
    Post { slug: Vec<String> },
    #[to("/<locale>/ip")]
    Ip { locale: String },
    #[to("/<locale>/time")]
    TimeRoot { locale: String },
    #[to("/<locale>/timeisr/<slug>")]
    Time { locale: String, slug: String },
    #[to("/<locale>/amalgamation")]
    Amalgamation { locale: String },
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
        ],
        Route::Post { slug } => [
            format!("post/{}", slug.join("/")),
            templates::post::template_fn(),
            "en-US".to_string() // BUG: see above
        ],
        Route::NewPost { locale } => [
            "post/new".to_string(),
            templates::new_post::template_fn(),
            locale
        ],
        Route::Ip { locale } => [
            "ip".to_string(),
            templates::ip::template_fn(),
            locale
        ],
        Route::Time { slug, locale } => [
            format!("timeisr/{}", slug),
            templates::time::template_fn(),
            locale
        ],
        Route::TimeRoot { locale } => [
            "time".to_string(),
            templates::time_root::template_fn(),
            locale
        ],
        Route::Amalgamation { locale } => [
            "amalgamation".to_string(),
            templates::amalgamation::template_fn(),
            locale
        ]
    },
    error_pages: crate::error_pages::get_error_pages(),
    templates: [
        crate::templates::index::get_template::<G>(),
        crate::templates::about::get_template::<G>(),
        crate::templates::post::get_template::<G>(),
        crate::templates::new_post::get_template::<G>(),
        crate::templates::ip::get_template::<G>(),
        crate::templates::time::get_template::<G>(),
        crate::templates::time_root::get_template::<G>(),
        crate::templates::amalgamation::get_template::<G>()
    ],
    locales: {
        default: "en-US",
        other: ["fr-FR", "es-ES"]
    }
}
