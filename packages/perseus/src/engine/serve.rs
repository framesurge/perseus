use crate::i18n::TranslationsManager;
use crate::plugins::PluginAction;
use crate::server::{ServerOptions, ServerProps};
use crate::stores::MutableStore;
use crate::PerseusAppBase;
use futures::executor::block_on;
use std::env;
use std::fs;
use sycamore::web::SsrNode;

/// Gets the host and port to serve on based on environment variables, which are
/// universally used for configuration regardless of engine.
pub(crate) fn get_host_and_port() -> (String, u16) {
    // We have to use two sets of environment variables until v0.4.0
    let host = env::var("PERSEUS_HOST");
    let port = env::var("PERSEUS_PORT");

    let host = host.unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = port
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("Port must be a number.");

    (host, port)
}

/// Gets the properties to pass to the server, invoking plugin opportunities as
/// necessary. This is entirely engine-agnostic.
///
/// WARNING: in production, this will automatically set the working directory
/// to be the parent of the actual binary! This means that disabling
/// debug assertions in development will lead to utterly incomprehensible
/// errors! You have been warned!
pub(crate) fn get_props<M: MutableStore, T: TranslationsManager>(
    app: PerseusAppBase<SsrNode, M, T>,
) -> ServerProps<M, T> {
    if !cfg!(debug_assertions) {
        let binary_loc = env::current_exe().unwrap();
        let binary_dir = binary_loc.parent().unwrap(); // It's a file, there's going to be a parent if we're working on anything close
                                                       // to sanity
        env::set_current_dir(binary_dir).unwrap();
    }

    let plugins = app.get_plugins();

    plugins
        .functional_actions
        .server_actions
        .before_serve
        .run((), plugins.get_plugin_data());

    let static_dir_path = app.get_static_dir();

    let app_root = app.get_root();
    let immutable_store = app.get_immutable_store();
    let index_view_str = app.get_index_view_str();
    // By the time this binary is being run, the app has already been built be the
    // CLI (hopefully!), so we can depend on access to the render config
    let index_view = block_on(PerseusAppBase::<SsrNode, M, T>::get_html_shell(
        index_view_str,
        &app_root,
        &immutable_store,
        &plugins,
    ));

    let opts = ServerOptions {
        // We don't support setting some attributes from `wasm-pack` through plugins/`PerseusApp`
        // because that would require CLI changes as well (a job for an alternative engine)
        html_shell: index_view,
        js_bundle: "dist/pkg/perseus_engine.js".to_string(),
        // Our crate has the same name, so this will be predictable
        wasm_bundle: "dist/pkg/perseus_engine_bg.wasm".to_string(),
        // This probably won't exist, but on the off chance that the user needs to support older
        // browsers, we'll provide it anyway
        wasm_js_bundle: "dist/pkg/perseus_engine_bg.wasm.js".to_string(),
        templates_map: app.get_atomic_templates_map(),
        locales: app.get_locales(),
        root_id: app_root,
        snippets: "dist/pkg/snippets".to_string(),
        error_pages: app.get_atomic_error_pages(),
        // This will be available directly at `/.perseus/static`
        static_dir: if fs::metadata(&static_dir_path).is_ok() {
            Some(static_dir_path)
        } else {
            None
        },
        static_aliases: app.get_static_aliases(),
    };

    ServerProps {
        opts,
        immutable_store,
        mutable_store: app.get_mutable_store(),
        global_state_creator: app.get_global_state_creator(),
        translations_manager: block_on(app.get_translations_manager()),
    }
}
