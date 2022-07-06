use crate::initial_load::initial_load_handler;
use crate::page_data::page_handler;
use crate::{
    conv_req::get_http_req,
    page_data::PageDataReq,
    static_content::{serve_file, static_aliases_filter},
    translations::translations_handler,
};
use perseus::internal::serve::{get_render_cfg, ServerProps};
use perseus::{internal::i18n::TranslationsManager, stores::MutableStore};
use std::sync::Arc;
use warp::Filter;

/// The routes for Perseus. These will configure an existing Warp instance to
/// run Perseus, and should be provided after any other routes, as they include
/// a wildcard route.
pub async fn perseus_routes<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    ServerProps {
        opts,
        immutable_store,
        mutable_store,
        translations_manager,
        global_state_creator,
    }: ServerProps<M, T>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let render_cfg = get_render_cfg(&immutable_store)
        .await
        .expect("Couldn't get render configuration!");
    let index_with_render_cfg = opts.html_shell.clone();
    // Generate the global state
    let global_state = global_state_creator
        .get_build_state()
        .await
        .expect("Couldn't generate global state.");

    // Handle static files
    let js_bundle = warp::path!(".perseus" / "bundle.js")
        .and(warp::path::end())
        .and(warp::fs::file(opts.js_bundle.clone()));
    let wasm_bundle = warp::path!(".perseus" / "bundle.wasm")
        .and(warp::path::end())
        .and(warp::fs::file(opts.wasm_bundle.clone()));
    let wasm_js_bundle = warp::path!(".perseus" / "bundle.wasm.js")
        .and(warp::path::end())
        .and(warp::fs::file(opts.wasm_js_bundle.clone()));
    // Handle JS interop snippets (which need to be served as separate files)
    let snippets = warp::path!(".perseus" / "snippets").and(warp::fs::dir(opts.snippets.clone()));
    // Handle static content in the user-set directories (this will all be under
    // `/.perseus/static`) We only set this if the user is using a static
    // content directory
    let static_dir_path = Arc::new(opts.static_dir.clone());
    let static_dir_path_filter = warp::any().map(move || static_dir_path.clone());
    let static_dir = warp::path!(".perseus" / "static" / ..)
        .and(static_dir_path_filter)
        .and_then(|static_dir_path: Arc<Option<String>>| async move {
            if static_dir_path.is_some() {
                Ok(())
            } else {
                Err(warp::reject::not_found())
            }
        })
        .untuple_one() // We need this to avoid a ((), File) (which makes the return type fail)
        // This alternative will never be served, but if we don't have it we'll get a runtime panic
        .and(warp::fs::dir(
            opts.static_dir.clone().unwrap_or_else(|| "".to_string()),
        ));
    // Handle static aliases
    let static_aliases = warp::any()
        .and(static_aliases_filter(opts.static_aliases.clone()))
        .and_then(serve_file);

    // Define some filters to handle all the data we want to pass through
    let opts = Arc::new(opts);
    let opts = warp::any().map(move || opts.clone());
    let immutable_store = Arc::new(immutable_store);
    let immutable_store = warp::any().map(move || immutable_store.clone());
    let mutable_store = Arc::new(mutable_store);
    let mutable_store = warp::any().map(move || mutable_store.clone());
    let translations_manager = Arc::new(translations_manager);
    let translations_manager = warp::any().map(move || translations_manager.clone());
    let html_shell = Arc::new(index_with_render_cfg);
    let html_shell = warp::any().map(move || html_shell.clone());
    let render_cfg = Arc::new(render_cfg);
    let render_cfg = warp::any().map(move || render_cfg.clone());
    let global_state = Arc::new(global_state);
    let global_state = warp::any().map(move || global_state.clone());

    // Handle getting translations
    let translations = warp::path!(".perseus" / "translations" / String)
        .and(opts.clone())
        .and(translations_manager.clone())
        .then(translations_handler);
    // Handle getting the static HTML/JSON of a page (used for subsequent loads)
    let page_data = warp::path!(".perseus" / "page" / String / ..)
        .and(warp::path::tail())
        .and(warp::query::<PageDataReq>())
        .and(get_http_req())
        .and(opts.clone())
        .and(immutable_store.clone())
        .and(mutable_store.clone())
        .and(translations_manager.clone())
        .and(global_state.clone())
        .then(page_handler);
    // Handle initial loads (we use a wildcard for this)
    let initial_loads = warp::any()
        .and(warp::path::full())
        .and(get_http_req())
        .and(opts)
        .and(html_shell)
        .and(render_cfg)
        .and(immutable_store)
        .and(mutable_store)
        .and(translations_manager)
        .and(global_state)
        .then(initial_load_handler);

    // Now put all those routes together in the final thing (the user will add this
    // to an existing Warp server)
    js_bundle
        .or(wasm_bundle)
        .or(wasm_js_bundle)
        .or(snippets)
        .or(static_dir)
        .or(static_aliases)
        .or(translations)
        .or(page_data)
        .or(initial_loads)
}
