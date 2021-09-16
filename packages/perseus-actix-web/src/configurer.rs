use crate::page_data::page_data;
use crate::translations::translations;
use actix_files::NamedFile;
use actix_web::web;
use perseus::{get_render_cfg, ConfigManager, Locales, SsrNode, TemplateMap, TranslationsManager};

/// The options for setting up the Actix Web integration. This should be literally constructed, as nothing is optional.
#[derive(Clone)]
pub struct Options {
    /// The location on the filesystem of your JavaScript bundle.
    pub js_bundle: String,
    /// The locales on the filesystem of the file that will invoke your JavaScript bundle. This should have something like `init()` in
    /// it.
    pub js_init: String,
    /// The location on the filesystem of your Wasm bundle.
    pub wasm_bundle: String,
    /// The location on the filesystem of your `index.html` file that includes the JS bundle.
    pub index: String,
    /// A `HashMap` of your app's templates by their paths.
    pub templates_map: TemplateMap<SsrNode>,
    /// The locales information for the app.
    pub locales: Locales,
}

async fn js_bundle(opts: web::Data<Options>) -> std::io::Result<NamedFile> {
    NamedFile::open(&opts.js_bundle)
}
async fn js_init(opts: web::Data<Options>) -> std::io::Result<NamedFile> {
    NamedFile::open(&opts.js_init)
}
async fn wasm_bundle(opts: web::Data<Options>) -> std::io::Result<NamedFile> {
    NamedFile::open(&opts.wasm_bundle)
}
async fn index(opts: web::Data<Options>) -> std::io::Result<NamedFile> {
    NamedFile::open(&opts.index)
}

/// Configures an existing Actix Web app for Perseus. This returns a function that does the configuring so it can take arguments.
pub async fn configurer<C: ConfigManager + 'static, T: TranslationsManager + 'static>(
    opts: Options,
    config_manager: C,
    translations_manager: T,
) -> impl Fn(&mut web::ServiceConfig) {
    let render_cfg = get_render_cfg(&config_manager)
        .await
        .expect("Couldn't get render configuration!");
    move |cfg: &mut web::ServiceConfig| {
        cfg
            // We implant the render config in the app data for better performance, it's needed on every request
            .data(render_cfg.clone())
            .data(config_manager.clone())
            .data(translations_manager.clone())
            .data(opts.clone())
            // TODO chunk JS and Wasm bundles
            // These allow getting the basic app code (not including the static data)
            // This contains everything in the spirit of a pseudo-SPA
            .route("/.perseus/main.js", web::get().to(js_init))
            .route("/.perseus/bundle.js", web::get().to(js_bundle))
            .route("/.perseus/bundle.wasm", web::get().to(wasm_bundle))
            // This allows getting the static HTML/JSON of a page
            // We stream both together in a single JSON object so SSR works (otherwise we'd have request IDs and weird caching...)
            .route(
                "/.perseus/page/{locale}/{filename:.*}",
                web::get().to(page_data::<C, T>),
            )
            // This allows the app shell to fetch translations for a given page
            .route(
                "/.perseus/translations/{locale}",
                web::get().to(translations::<T>),
            )
            // For everything else, we'll serve the app shell directly
            .route("*", web::get().to(index));
    }
}
