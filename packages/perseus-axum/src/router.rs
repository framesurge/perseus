use crate::initial_load::initial_load_handler;
use crate::page_data::page_handler;
use crate::translations::translations_handler;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use closure::closure;
use perseus::{i18n::TranslationsManager, stores::MutableStore};
use perseus::{
    server::{get_render_cfg, ServerProps},
    state::get_built_global_state,
};
use std::sync::Arc;
use tower_http::services::{ServeDir, ServeFile};

/// Gets the `Router` needed to configure an existing Axum app for Perseus, and
/// should be provided after any other routes, as they include a wildcard route.
pub async fn get_router<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    ServerProps {
        opts,
        immutable_store,
        mutable_store,
        translations_manager,
        global_state_creator,
    }: ServerProps<M, T>,
) -> Router {
    let render_cfg = get_render_cfg(&immutable_store)
        .await
        .expect("Couldn't get render configuration!");
    let index_with_render_cfg = opts.html_shell.clone();
    // Generate the global state
    let global_state = get_built_global_state(&immutable_store)
        .await
        .expect("couldn't get pre-built global state or placeholder (the app's build artifacts have almost certainly been corrupted)");

    let immutable_store = Arc::new(immutable_store);
    let mutable_store = Arc::new(mutable_store);
    let translations_manager = Arc::new(translations_manager);
    let html_shell = Arc::new(index_with_render_cfg);
    let render_cfg = Arc::new(render_cfg);
    let global_state = Arc::new(global_state);

    let static_dir = opts.static_dir.clone();
    let static_aliases = opts.static_aliases.clone();

    let router = Router::new()
        .route(
            "/.perseus/bundle.js",
            get_service(ServeFile::new(opts.js_bundle.clone())).handle_error(handle_fs_error),
        )
        .route(
            "/.perseus/bundle.wasm",
            get_service(ServeFile::new(opts.wasm_bundle.clone())).handle_error(handle_fs_error),
        )
        .route(
            "/.perseus/bundle.wasm.js",
            get_service(ServeFile::new(opts.wasm_js_bundle.clone())).handle_error(handle_fs_error),
        )
        .route(
            "/.perseus/snippets/*path",
            get_service(ServeDir::new(opts.snippets.clone())).handle_error(handle_fs_error),
        );
    let opts = Arc::new(opts);
    let mut router = router
        .route(
            "/.perseus/translations/:locale",
            get(closure!(clone opts, clone translations_manager, |path| translations_handler::<T>(path, opts, translations_manager))),
        )
        .route("/.perseus/page/:locale/*tail", get(
            closure!(
                clone opts,
                clone immutable_store,
                clone mutable_store,
                clone translations_manager,
                clone global_state,
                clone global_state_creator,
                |path, query, http_req|
                    page_handler::<M, T>(
                        path,
                        query,
                        http_req,
                        opts,
                        immutable_store,
                        mutable_store,
                        translations_manager,
                        global_state,
                        global_state_creator,
                    )
            )
        ));
    // Only add the static content directory route if such a directory is being used
    if let Some(static_dir) = static_dir {
        router = router.nest_service(
            "/.perseus/static",
            get_service(ServeDir::new(static_dir)).handle_error(handle_fs_error),
        )
    }
    // Now add support for serving static aliases
    for (url, static_path) in static_aliases.iter() {
        // Note that `static_path` is already relative to the right place
        // (`.perseus/server/`)
        router = router.route(
            url, // This comes with a leading forward slash!
            get_service(ServeFile::new(static_path)).handle_error(handle_fs_error),
        );
    }
    // And add the fallback for initial loads
    router.fallback_service(get(closure!(
        clone opts,
        clone html_shell,
        clone render_cfg,
        clone immutable_store,
        clone mutable_store,
        clone translations_manager,
        clone global_state,
        clone global_state_creator,
        |http_req|
        initial_load_handler::<M, T>(
            http_req,
            opts,
            html_shell,
            render_cfg,
            immutable_store,
            mutable_store,
            translations_manager,
            global_state,
            global_state_creator,
        )
    )))
}

// TODO Review if there's anything more to do here
async fn handle_fs_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Couldn't serve file.")
}
