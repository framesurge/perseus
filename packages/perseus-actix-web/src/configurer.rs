use crate::initial_load::initial_load;
use crate::page_data::page_data;
use crate::translations::translations;
use actix_files::{Files, NamedFile};
use actix_web::{web, HttpRequest};
use perseus::{
    internal::{
        get_path_prefix_server,
        i18n::TranslationsManager,
        serve::{get_render_cfg, prep_html_shell, ServerOptions},
    },
    stores::{ImmutableStore, MutableStore},
};
use std::fs;
use std::rc::Rc;

async fn js_bundle(opts: web::Data<Rc<ServerOptions>>) -> std::io::Result<NamedFile> {
    NamedFile::open(&opts.js_bundle)
}
async fn wasm_bundle(opts: web::Data<Rc<ServerOptions>>) -> std::io::Result<NamedFile> {
    NamedFile::open(&opts.wasm_bundle)
}
async fn static_alias(
    opts: web::Data<Rc<ServerOptions>>,
    req: HttpRequest,
) -> std::io::Result<NamedFile> {
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
pub async fn configurer<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    opts: ServerOptions,
    immutable_store: ImmutableStore,
    mutable_store: M,
    translations_manager: T,
) -> impl Fn(&mut web::ServiceConfig) {
    let opts = Rc::new(opts); // TODO Find a more efficient way of doing this
    let render_cfg = get_render_cfg(&immutable_store)
        .await
        .expect("Couldn't get render configuration!");
    // Get the index file and inject the render configuration into ahead of time
    // Anything done here will affect any status code and all loads
    let index_file = fs::read_to_string(&opts.index).expect("Couldn't get HTML index file!");
    let index_with_render_cfg = prep_html_shell(index_file, &render_cfg, &get_path_prefix_server());

    move |cfg: &mut web::ServiceConfig| {
        cfg
            // We implant the render config in the app data for better performance, it's needed on every request
            .data(render_cfg.clone())
            .data(immutable_store.clone())
            .data(mutable_store.clone())
            .data(translations_manager.clone())
            .data(opts.clone())
            .data(index_with_render_cfg.clone())
            // TODO chunk JS and Wasm bundles
            // These allow getting the basic app code (not including the static data)
            // This contains everything in the spirit of a pseudo-SPA
            .route("/.perseus/bundle.js", web::get().to(js_bundle))
            .route("/.perseus/bundle.wasm", web::get().to(wasm_bundle))
            // This allows getting the static HTML/JSON of a page
            // We stream both together in a single JSON object so SSR works (otherwise we'd have request IDs and weird caching...)
            // A request to this should also provide the template name (routing should only be done once on the client) as a query parameter
            .route(
                "/.perseus/page/{locale}/{filename:.*}.json",
                web::get().to(page_data::<M, T>),
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
        cfg.route("*", web::get().to(initial_load::<M, T>));
    }
}
