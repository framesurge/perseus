use actix_web::{App, HttpServer};
use app::{
    get_error_pages, get_immutable_store, get_locales, get_mutable_store, get_static_aliases,
    get_templates_map, get_translations_manager, APP_ROOT,
};
use futures::executor::block_on;
use perseus_actix_web::{configurer, Options};
use std::collections::HashMap;
use std::env;
use std::fs;

// This server executable can be run in two modes:
//      dev: inside `.perseus/server/src/main.rs`, works with that file structure
//      prod: as a standalone executable with a `dist/` directory as a sibling
// The prod mode can be enabled by setting the `PERSEUS_STANDALONE` environment variable

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // So we don't have to define a different `FsConfigManager` just for the server, we shift the execution context to the same level as everything else
    // The server has to be a separate crate because otherwise the dependencies don't work with Wasm bundling
    // If we're not running as a standalone binary, assume we're running in dev mode under `.perseus/`
    if env::var("PERSEUS_STANDALONE").is_err() {
        env::set_current_dir("../").unwrap();
    }

    // This allows us to operate inside `.perseus/` and as a standalone binary in production
    let (html_shell_path, static_dir_path) = if env::var("PERSEUS_STANDALONE").is_ok() {
        ("./index.html", "./static")
    } else {
        ("../index.html", "../static")
    };

    let host = env::var("HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>();
    if let Ok(port) = port {
        HttpServer::new(move || {
            App::new().configure(block_on(configurer(
                Options {
                    index: html_shell_path.to_string(), // The user must define their own `index.html` file
                    js_bundle: "dist/pkg/perseus_cli_builder.js".to_string(),
                    // Our crate has the same name, so this will be predictable
                    wasm_bundle: "dist/pkg/perseus_cli_builder_bg.wasm".to_string(),
                    templates_map: get_templates_map(),
                    locales: get_locales(),
                    root_id: APP_ROOT.to_string(),
                    snippets: "dist/pkg/snippets".to_string(),
                    error_pages: get_error_pages(),
                    // The CLI supports static content in `../static` by default if it exists
                    // This will be available directly at `/.perseus/static`
                    static_dirs: if fs::metadata(static_dir_path).is_ok() {
                        let mut static_dirs = HashMap::new();
                        static_dirs.insert("".to_string(), static_dir_path.to_string());
                        static_dirs
                    } else {
                        HashMap::new()
                    },
                    static_aliases: get_static_aliases(),
                },
                get_immutable_store(),
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
