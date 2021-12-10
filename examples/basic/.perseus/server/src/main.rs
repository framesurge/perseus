use actix_web::{App, HttpServer};
use futures::executor::block_on;
use perseus::plugins::PluginAction;
use perseus::SsrNode;
use perseus_actix_web::{configurer, ServerOptions};
use perseus_engine::app::{
    get_app_root, get_error_pages_contained, get_immutable_store, get_locales, get_mutable_store,
    get_plugins, get_static_aliases, get_templates_map_atomic_contained, get_translations_manager,
};
use std::collections::HashMap;
use std::env;
use std::fs;

// This server executable can be run in two modes:
//      dev: inside `.perseus/server/src/main.rs`, works with that file structure
//      prod: as a standalone executable with a `dist/` directory as a sibling
// The prod mode can be enabled by setting the `PERSEUS_STANDALONE` environment variable

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let plugins = get_plugins::<SsrNode>();

    // So we don't have to define a different `FsConfigManager` just for the server, we shift the execution context to the same level as everything else
    // The server has to be a separate crate because otherwise the dependencies don't work with Wasm bundling
    // If we're not running as a standalone binary, assume we're running in dev mode under `.perseus/`
    let is_standalone;
    if env::var("PERSEUS_STANDALONE").is_err() {
        env::set_current_dir("../").unwrap();
        is_standalone = false;
    } else {
        // If we are running as a standalone binary, we have no idea where we're being executed from (#63), so we should set the working directory to be the same as the binary location
        let binary_loc = env::current_exe().unwrap();
        let binary_dir = binary_loc.parent().unwrap(); // It's a file, there's going to be a parent if we're working on anything close to sanity
        env::set_current_dir(binary_dir).unwrap();
        is_standalone = true;
    }

    plugins
        .functional_actions
        .server_actions
        .before_serve
        .run((), plugins.get_plugin_data());

    // This allows us to operate inside `.perseus/` and as a standalone binary in production
    let (html_shell_path, static_dir_path) = if is_standalone {
        ("./index.html", "./static")
    } else {
        ("../index.html", "../static")
    };

    let host = env::var("HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>();
    if let Ok(port) = port {
        let immutable_store = get_immutable_store(&plugins);
        let locales = get_locales(&plugins);
        let app_root = get_app_root(&plugins);
        let static_aliases = get_static_aliases(&plugins);
        HttpServer::new(move || {
            // TODO find a way to configure the server with plugins without using `actix-web` in the `perseus` crate (it won't compile to Wasm)
            App::new().configure(block_on(configurer(
                ServerOptions {
                    // We don't support setting some attributes from `wasm-pack` through plugins/`define_app!` because that would require CLI changes as well (a job for an alternative engine)
                    index: html_shell_path.to_string(), // The user must define their own `index.html` file
                    js_bundle: "dist/pkg/perseus_engine.js".to_string(),
                    // Our crate has the same name, so this will be predictable
                    wasm_bundle: "dist/pkg/perseus_engine_bg.wasm".to_string(),
                    // It's a nightmare to get the templates map to take plugins, so we use a self-contained version
                    // TODO reduce allocations here
                    templates_map: get_templates_map_atomic_contained(),
                    locales: locales.clone(),
                    root_id: app_root.to_string(),
                    snippets: "dist/pkg/snippets".to_string(),
                    error_pages: get_error_pages_contained(),
                    // The CLI supports static content in `../static` by default if it exists
                    // This will be available directly at `/.perseus/static`
                    static_dirs: if fs::metadata(&static_dir_path).is_ok() {
                        let mut static_dirs = HashMap::new();
                        static_dirs.insert("".to_string(), static_dir_path.to_string());
                        static_dirs
                    } else {
                        HashMap::new()
                    },
                    static_aliases: static_aliases.clone(),
                },
                immutable_store.clone(),
                get_mutable_store(),
                block_on(get_translations_manager()),
            )))
        })
        .bind((host, port))?
        .run()
        .await
    } else {
        eprintln!("Port must be a number.");
        Ok(())
    }
}
