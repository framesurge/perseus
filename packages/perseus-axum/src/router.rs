use crate::initial_load::initial_load_handler;
use crate::page_data::page_handler;
use crate::translations::translations_handler;
use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use perseus::internal::serve::{get_render_cfg, ServerProps};
use perseus::{internal::i18n::TranslationsManager, stores::MutableStore};
use std::sync::Arc;
use tower::builder::ServiceBuilder;
use tower_http::services::{ServeDir, ServeFile};

/// Gets the `Router` needed to configure an existing Axum app for Perseus, and should be provided after any other routes, as they include a wildcard
/// route.
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
    let global_state = global_state_creator
        .get_build_state()
        .await
        .expect("Couldn't generate global state.");

    let immutable_store = Arc::new(immutable_store);
    let mutable_store = Arc::new(mutable_store);
    let translations_manager = Arc::new(translations_manager);
    let html_shell = Arc::new(index_with_render_cfg);
    let render_cfg = Arc::new(render_cfg);
    let global_state = Arc::new(global_state);

    let static_dir = opts.static_dir.clone();
    let static_aliases = opts.static_aliases.clone();

    let mut router = Router::new()
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
            "/.perseus/snippets/*_",
            get_service(ServeDir::new(opts.snippets.clone())).handle_error(handle_fs_error),
        )
        .route(
            "/.perseus/translations/:locale",
            get(translations_handler::<T>),
        )
        .route("/.perseus/page/:locale/*tail", get(page_handler::<M, T>));
    // Only add the static content directory route if such a directory is being used
    if let Some(static_dir) = static_dir {
        router = router.route(
            "/.perseus/static/*_",
            get_service(ServeDir::new(static_dir)).handle_error(handle_fs_error),
        )
    }
    // Now add support for serving static aliases
    for (url, static_path) in static_aliases.iter() {
        // Note that `static_path` is already relative to the right place (`.perseus/server/`)
        router = router.route(
            &format!("/{}", url),
            get_service(ServeFile::new(static_path)).handle_error(handle_fs_error),
        );
    }
    // And add the fallback for initial loads
    router = router.fallback(get(initial_load_handler::<M, T>));
    // And finally all the shared state
    let shared_state = ServiceBuilder::new()
        .layer(Extension(Arc::new(opts)))
        .layer(Extension(Arc::new(immutable_store)))
        .layer(Extension(Arc::new(mutable_store)))
        .layer(Extension(Arc::new(translations_manager)))
        .layer(Extension(Arc::new(html_shell)))
        .layer(Extension(Arc::new(render_cfg)))
        .layer(Extension(Arc::new(global_state)))
        .into_inner();
    router.layer(shared_state)
}

// TODO Review if there's anything more to do here
async fn handle_fs_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Couldn't serve file.")
}
