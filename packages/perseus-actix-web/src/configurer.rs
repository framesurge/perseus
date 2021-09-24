use crate::initial_load::initial_load;
use crate::page_data::page_data;
use crate::translations::translations;
use actix_files::{Files, NamedFile};
use actix_web::{web, HttpRequest};
use perseus::{
    get_render_cfg, html_shell::prep_html_shell, ConfigManager, ErrorPages, Locales, SsrNode,
    TemplateMap, TranslationsManager,
};
use std::collections::HashMap;
use std::fs;

/// The options for setting up the Actix Web integration. This should be literally constructed, as nothing is optional.
#[derive(Clone)]
pub struct Options {
    /// The location on the filesystem of your JavaScript bundle.
    pub js_bundle: String,
    /// The location on the filesystem of your Wasm bundle.
    pub wasm_bundle: String,
    /// The location on the filesystem of your `index.html` file that includes the JS bundle.
    pub index: String,
    /// A `HashMap` of your app's templates by their paths.
    pub templates_map: TemplateMap<SsrNode>,
    /// The locales information for the app.
    pub locales: Locales,
    /// The HTML `id` of the element at which to render Perseus. On the server-side, interpolation will be done here in a highly
    /// efficient manner by not parsing the HTML, so this MUST be of the form `<div id="root_id">` in your markup (double or single
    /// quotes, `root_id` replaced by what this property is set to).
    pub root_id: String,
    /// The location of the JS interop snippets to be served as static files.
    pub snippets: String,
    /// The error pages for the app. These will be server-rendered if an initial load fails.
    pub error_pages: ErrorPages<SsrNode>,
    /// Directories to serve static content from, mapping URL to folder path. Note that the URL provided will be gated behind
    /// `.perseus/static/`, and must have a leading `/`. If you're using a CMS instead, you should set these up outside the Perseus
    /// server (but they might still be on the same machine, you can still add more routes after Perseus is configured).
    pub static_dirs: HashMap<String, String>,
    /// A map of URLs to act as aliases for certain static resources. These are particularly designed for things like a site manifest or
    /// favicons, which should be stored in a static directory, but need to be aliased at a path like `/favicon.ico`.
    pub static_aliases: HashMap<String, String>,
}

async fn render_conf(
    render_conf: web::Data<HashMap<String, String>>,
) -> web::Json<HashMap<String, String>> {
    web::Json(render_conf.get_ref().clone())
}
async fn js_bundle(opts: web::Data<Options>) -> std::io::Result<NamedFile> {
    NamedFile::open(&opts.js_bundle)
}
async fn wasm_bundle(opts: web::Data<Options>) -> std::io::Result<NamedFile> {
    NamedFile::open(&opts.wasm_bundle)
}
async fn static_alias(opts: web::Data<Options>, req: HttpRequest) -> std::io::Result<NamedFile> {
    let filename = opts.static_aliases.get(req.path());
    let filename = match filename {
        Some(filename) => filename,
        // If the path doesn't exist, then the alias is not found
        None => return Err(std::io::Error::from(std::io::ErrorKind::NotFound)),
    };
    NamedFile::open(filename)
}

/// Configures an existing Actix Web app for Perseus. This returns a function that does the configuring so it can take arguments. This
/// includes a complete wildcard handler (`*`), and so it should be configured after any other routes on your server.
pub async fn configurer<C: ConfigManager + 'static, T: TranslationsManager + 'static>(
    opts: Options,
    config_manager: C,
    translations_manager: T,
) -> impl Fn(&mut web::ServiceConfig) {
    let render_cfg = get_render_cfg(&config_manager)
        .await
        .expect("Couldn't get render configuration!");
    // Get the index file and inject the render configuration into ahead of time
    // Anything done here will affect any status code and all loads
    let index_file = fs::read_to_string(&opts.index).expect("Couldn't get HTML index file!");
    let index_with_render_cfg = prep_html_shell(index_file, &render_cfg);

    move |cfg: &mut web::ServiceConfig| {
        cfg
            // We implant the render config in the app data for better performance, it's needed on every request
            .data(render_cfg.clone())
            .data(config_manager.clone())
            .data(translations_manager.clone())
            .data(opts.clone())
            .data(index_with_render_cfg.clone())
            // TODO chunk JS and Wasm bundles
            // These allow getting the basic app code (not including the static data)
            // This contains everything in the spirit of a pseudo-SPA
            .route("/.perseus/bundle.js", web::get().to(js_bundle))
            .route("/.perseus/bundle.wasm", web::get().to(wasm_bundle))
            .route("/.perseus/render_conf.json", web::get().to(render_conf))
            // This allows getting the static HTML/JSON of a page
            // We stream both together in a single JSON object so SSR works (otherwise we'd have request IDs and weird caching...)
            // A request to this should also provide the template name (routing should only be done once on the client) as a query parameter
            .route(
                "/.perseus/page/{locale}/{filename:.*}",
                web::get().to(page_data::<C, T>),
            )
            // This allows the app shell to fetch translations for a given page
            .route(
                "/.perseus/translations/{locale}",
                web::get().to(translations::<T>),
            )
            // This allows gettting JS interop snippets (including ones that are supposedly 'inlined')
            // These won't change, so they can be set as a filesystem dependency safely
            .service(Files::new("/.perseus/snippets", &opts.snippets));
        // Now we add support for any static content the user wants to provide
        for (url, static_dir) in opts.static_dirs.iter() {
            cfg.service(Files::new(&format!("/.perseus/static{}", url), static_dir));
        }
        // And finally add in aliases for static content as necessary
        for (url, _static_path) in opts.static_aliases.iter() {
            // This handler indexes the path of the request in `opts.static_aliases` to figure out what to serve
            cfg.route(url, web::get().to(static_alias));
        }
        // For everything else, we'll serve the app shell directly
        // This has to be done AFTER everything else, because it will match anything that's left
        cfg.route("*", web::get().to(initial_load::<C, T>));
    }
}
