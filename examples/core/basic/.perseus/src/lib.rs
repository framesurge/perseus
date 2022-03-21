#![allow(clippy::unused_unit)] // rustwasm/wasm-bindgen#2774 awaiting next `wasm-bindgen` release

// The user should use the `main` macro to create this wrapper
pub use app::__perseus_main as main;

use perseus::{
    checkpoint, create_app_route,
    internal::{
        router::{PerseusRouter, PerseusRouterProps},
        shell::get_render_cfg,
    },
    plugins::PluginAction,
    templates::TemplateNodeType,
};
use sycamore::prelude::view;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// The entrypoint into the app itself. This will be compiled to Wasm and actually executed, rendering the rest of the app.
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let app = main();
    let plugins = app.get_plugins();

    checkpoint("begin");
    // Panics should always go to the console
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    plugins
        .functional_actions
        .client_actions
        .start
        .run((), plugins.get_plugin_data());
    checkpoint("initial_plugins_complete");

    // Get the root we'll be injecting the router into
    let root = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector(&format!("#{}", app.get_root()))
        .unwrap()
        .unwrap();

    // Create the route type we'll use for this app, based on the user's app definition
    create_app_route! {
        name => AppRoute,
        // The render configuration is injected verbatim into the HTML shell, so it certainly should be present
        render_cfg => &get_render_cfg().expect("render configuration invalid or not injected"),
        // TODO avoid unnecessary allocation here (major problem!)
        // The `G` parameter is ambient here for `RouteVerdict`
        templates => &main::<G>().get_templates_map(),
        locales => &main::<G>().get_locales()
    }
    // Create a new version of the router with that
    type PerseusRouterWithAppRoute<G> = PerseusRouter<G, AppRoute<TemplateNodeType>>;

    // Set up the properties we'll pass to the router
    let router_props = PerseusRouterProps {
        locales: app.get_locales(),
        error_pages: app.get_error_pages(),
    };

    sycamore::render_to(
        move || {
            view! {
                // Actually render the router
                // The Perseus router includes our entire app, and is, for all intents and purposes, the app itself
                PerseusRouterWithAppRoute(router_props)
            }
        },
        &root,
    );

    Ok(())
}
