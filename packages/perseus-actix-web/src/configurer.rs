use crate::initial_load::initial_load;
use crate::page_data::page_data;
use crate::translations::translations;
use actix_files::{Files, NamedFile};
use actix_web::{web, HttpRequest};
use perseus::{
    i18n::TranslationsManager,
    server::{get_render_cfg, ServerOptions, ServerProps},
    stores::MutableStore,
};

async fn js_bundle(opts: web::Data<ServerOptions>) -> std::io::Result<NamedFile> {
    NamedFile::open(&opts.js_bundle)
}
async fn wasm_bundle(opts: web::Data<ServerOptions>) -> std::io::Result<NamedFile> {
    NamedFile::open(&opts.wasm_bundle)
}
async fn wasm_js_bundle(opts: web::Data<ServerOptions>) -> std::io::Result<NamedFile> {
    NamedFile::open(&opts.wasm_js_bundle)
}
async fn static_alias(
    opts: web::Data<ServerOptions>,
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

/// Configures an existing Actix Web app for Perseus. This returns a function
/// that does the configuring so it can take arguments. This includes a complete
/// wildcard handler (`*`), and so it should be configured after any other
/// routes on your server.
pub async fn configurer<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    ServerProps {
        opts,
        immutable_store,
        mutable_store,
        translations_manager,
        global_state_creator,
    }: ServerProps<M, T>,
) -> impl FnOnce(&mut actix_web::web::ServiceConfig) {
    let render_cfg = get_render_cfg(&immutable_store)
        .await
        .expect("Couldn't get render configuration!");
    let index_with_render_cfg = opts.html_shell.clone();
    // Generate the global state
    // The user will get a more detailed error message in the build process
    let global_state = global_state_creator
        .get_build_state()
        .await
        .expect("Couldn't generate global state.");

    move |cfg: &mut web::ServiceConfig| {
        cfg
            // We implant the render config in the app data for better performance, it's needed on
            // every request
            .app_data(web::Data::new(render_cfg.clone()))
            .app_data(web::Data::new(immutable_store.clone()))
            .app_data(web::Data::new(mutable_store.clone()))
            .app_data(web::Data::new(translations_manager.clone()))
            .app_data(web::Data::new(opts.clone()))
            .app_data(web::Data::new(index_with_render_cfg.clone()))
            .app_data(web::Data::new(global_state.clone()))
            // TODO chunk JS and Wasm bundles
            // These allow getting the basic app code (not including the static data)
            // This contains everything in the spirit of a pseudo-SPA
            .route("/.perseus/bundle.js", web::get().to(js_bundle))
            .route("/.perseus/bundle.wasm", web::get().to(wasm_bundle))
            .route("/.perseus/bundle.wasm.js", web::get().to(wasm_js_bundle))
            // This allows getting the static HTML/JSON of a page
            // We stream both together in a single JSON object so SSR works (otherwise we'd have
            // request IDs and weird caching...) A request to this should also provide
            // the template name (routing should only be done once on the client) as a query
            // parameter
            .route(
                "/.perseus/page/{locale}/{filename:.*}.json",
                web::get().to(page_data::<M, T>),
            )
            // This allows the app shell to fetch translations for a given page
            .route(
                "/.perseus/translations/{locale}",
                web::get().to(translations::<T>),
            )
            // This allows gettting JS interop snippets (including ones that are supposedly
            // 'inlined') These won't change, so they can be set as a filesystem
            // dependency safely
            .service(Files::new("/.perseus/snippets", &opts.snippets));
        // Now we add support for any static content the user wants to provide
        if let Some(static_dir) = &opts.static_dir {
            cfg.service(Files::new("/.perseus/static", static_dir));
        }
        // And finally add in aliases for static content as necessary
        for (url, _static_path) in opts.static_aliases.iter() {
            // This handler indexes the path of the request in `opts.static_aliases` to
            // figure out what to serve
            cfg.route(url, web::get().to(static_alias));
        }
        // For everything else, we'll serve the app shell directly
        // This has to be done AFTER everything else, because it will match anything
        // that's left
        cfg.route("{route:.*}", web::get().to(initial_load::<M, T>));
    }
}
