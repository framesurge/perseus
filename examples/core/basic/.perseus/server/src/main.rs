use futures::executor::block_on;
use perseus::internal::i18n::TranslationsManager;
use perseus::internal::serve::{ServerOptions, ServerProps};
use perseus::plugins::PluginAction;
use perseus::stores::MutableStore;
use perseus::PerseusApp;
use perseus::SsrNode;
use perseus_engine as app;
use std::env;
use std::fs;

// This server executable can be run in two modes:
//      dev: inside `.perseus/server/src/main.rs`, works with that file structure
//      prod: as a standalone executable with a `dist/` directory as a sibling

// Integration: Actix Web
#[cfg(feature = "integration-actix-web")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("WARNING: The Actix Web integration uses a beta version of Actix Web, and is considered unstable. It is not recommended for production usage.");

    use actix_web::{App, HttpServer};
    use perseus_actix_web::configurer;

    let is_standalone = get_standalone_and_act();
    let (host, port) = get_host_and_port();

    HttpServer::new(move || App::new().configure(block_on(configurer(get_props(is_standalone)))))
        .bind((host, port))?
        .run()
        .await
}

// Integration: Warp
#[cfg(feature = "integration-warp")]
#[tokio::main]
async fn main() {
    use perseus_warp::perseus_routes;
    use std::net::SocketAddr;

    let is_standalone = get_standalone_and_act();
    let props = get_props(is_standalone);
    let (host, port) = get_host_and_port();
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address provided to bind to.");
    let routes = block_on(perseus_routes(props));
    warp::serve(routes).run(addr).await;
}

/// Determines whether or not we're operating in standalone mode, and acts accordingly. This MUST be executed in the parent thread, as it switches the current directory.
fn get_standalone_and_act() -> bool {
    // So we don't have to define a different `FsConfigManager` just for the server, we shift the execution context to the same level as everything else
    // The server has to be a separate crate because otherwise the dependencies don't work with Wasm bundling
    // If we're not running as a standalone binary, assume we're running in dev mode under `.perseus/`
    if !cfg!(feature = "standalone") {
        env::set_current_dir("../").unwrap();
        false
    } else {
        // If we are running as a standalone binary, we have no idea where we're being executed from (#63), so we should set the working directory to be the same as the binary location
        let binary_loc = env::current_exe().unwrap();
        let binary_dir = binary_loc.parent().unwrap(); // It's a file, there's going to be a parent if we're working on anything close to sanity
        env::set_current_dir(binary_dir).unwrap();
        true
    }
}

/// Gets the host and port to serve on.
fn get_host_and_port() -> (String, u16) {
    // We have to use two sets of environment variables until v0.4.0
    // TODO Remove the old environment variables in v0.4.0
    let host_old = env::var("HOST");
    let port_old = env::var("PORT");
    let host = env::var("PERSEUS_HOST");
    let port = env::var("PERSEUS_PORT");

    let host = host.unwrap_or_else(|_| host_old.unwrap_or_else(|_| "127.0.0.1".to_string()));
    let port = port
        .unwrap_or_else(|_| port_old.unwrap_or_else(|_| "8080".to_string()))
        .parse::<u16>()
        .expect("Port must be a number.");

    (host, port)
}

/// Gets the properties to pass to the server.
fn get_props(is_standalone: bool) -> ServerProps<impl MutableStore, impl TranslationsManager> {
    let app = app::main::<SsrNode>();
    let plugins = app.get_plugins();

    plugins
        .functional_actions
        .server_actions
        .before_serve
        .run((), plugins.get_plugin_data());

    // This allows us to operate inside `.perseus/` and as a standalone binary in production
    let static_dir_path = if is_standalone {
        "./static"
    } else {
        "../static"
    };

    let immutable_store = app.get_immutable_store();
    let locales = app.get_locales();
    let app_root = app.get_root();
    let static_aliases = app.get_static_aliases();
    let templates_map = app.get_atomic_templates_map();
    let error_pages = app.get_error_pages();
    let index_view_str = app.get_index_view_str();
    // Generate the global state
    let global_state_creator = app.get_global_state_creator();
    // By the time this binary is being run, the app has already been built be the CLI (hopefully!), so we can depend on access to hte render config
    let index_view = block_on(PerseusApp::get_html_shell(
        index_view_str,
        &app_root,
        &immutable_store,
        &plugins,
    ));

    let opts = ServerOptions {
        // We don't support setting some attributes from `wasm-pack` through plugins/`define_app!` because that would require CLI changes as well (a job for an alternative engine)
        html_shell: index_view,
        js_bundle: "dist/pkg/perseus_engine.js".to_string(),
        // Our crate has the same name, so this will be predictable
        wasm_bundle: "dist/pkg/perseus_engine_bg.wasm".to_string(),
        // This probably won't exist, but on the off chance that the user needs to support older browsers, we'll provide it anyway
        wasm_js_bundle: "dist/pkg/perseus_engine_bg.wasm.js".to_string(),
        templates_map,
        locales,
        root_id: app_root,
        snippets: "dist/pkg/snippets".to_string(),
        error_pages,
        // The CLI supports static content in `../static` by default if it exists
        // This will be available directly at `/.perseus/static`
        static_dir: if fs::metadata(&static_dir_path).is_ok() {
            Some(static_dir_path.to_string())
        } else {
            None
        },
        static_aliases,
    };

    ServerProps {
        opts,
        immutable_store,
        mutable_store: app.get_mutable_store(),
        translations_manager: block_on(app.get_translations_manager()),
        global_state_creator,
    }
}
