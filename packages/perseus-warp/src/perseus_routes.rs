use crate::page_data::page_handler;
use crate::{
    conv_req::get_http_req,
    page_data::PageDataReq,
    static_content::{static_aliases_filter, static_dirs_filter},
    translations::translations_handler,
};
use perseus::internal::serve::{get_render_cfg, ServerOptions};
use perseus::{
    internal::{get_path_prefix_server, i18n::TranslationsManager, serve::prep_html_shell},
    stores::{ImmutableStore, MutableStore},
};
use std::{
    fs,
    sync::{Arc, Mutex, RwLock},
};
use warp::{
    path::{FullPath, Tail},
    Filter,
};

/// The routes for Perseus. These will configure an existing Warp instance to run Perseus, and should be provided after any other routes, as they include a wildcard
/// route.
pub async fn perseus_routes<M: MutableStore, T: TranslationsManager>(
    opts: ServerOptions,
    immutable_store: ImmutableStore,
    mutable_store: M,
    translations_manager: T,
) {
    let render_cfg = get_render_cfg(&immutable_store)
        .await
        .expect("Couldn't get render configuration!");
    let index_file = fs::read_to_string(&opts.index).expect("Couldn't get HTML index file!");
    let index_with_render_cfg = prep_html_shell(index_file, &render_cfg, &get_path_prefix_server());

    // Handle static files
    let js_bundle = warp::path(".perseus/bundle.js").and(warp::fs::file(opts.js_bundle.clone()));
    let wasm_bundle =
        warp::path(".perseus/bundle.wasm").and(warp::fs::file(opts.wasm_bundle.clone()));
    // Handle JS interop snippets (which need to be served as separate files)
    let snippets = warp::path(".perseus/snippets").and(warp::fs::dir(opts.snippets.clone()));
    // Handle static content in the user-set directories (this will all be under `/.perseus/static`)
    let static_dirs = warp::path(".perseus/static")
        .and(static_dirs_filter(opts.static_dirs.clone()))
        .map(|dir_to_serve| warp::fs::dir(dir_to_serve));
    // Handle static aliases
    let static_aliases = warp::any()
        .and(static_aliases_filter(opts.static_aliases.clone()))
        .map(|file_to_serve| warp::fs::file(file_to_serve));

    // Define some filters to handle all the data we want to pass through
    let opts = Arc::new(Mutex::new(opts));
    // let opts = warp::any().map(|| opts.clone());
    let immutable_store = Arc::new(immutable_store);
    let immutable_store = warp::any().map(|| immutable_store.clone());
    let mutable_store = Arc::new(mutable_store);
    let mutable_store = warp::any().map(|| mutable_store.clone());
    let translations_manager = Arc::new(translations_manager);
    let translations_manager = warp::any().map(|| translations_manager.clone());

    // TODO Handle getting translations (needs the locale, the server options, and the translations manager)
    // let translations = warp::path!(".perseus/translations" / String).then({
    //     |locale| async move {
    //         let opts = opts.lock().unwrap();
    //         format!("{}", opts.index)
    //     }
    // });
    // TODO Handle getting the static HTML/JSON of a page (used for subsequent loads)
    // TODO Handle initial loads (we use a wildcard for this)
}
