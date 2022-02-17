#![allow(clippy::unused_unit)] // rustwasm/wasm-bindgen#2774 awaiting next `wasm-bindgen` release

pub mod app;

use crate::app::{get_app_root, get_error_pages, get_locales, get_plugins, get_templates_map};
use perseus::{
    checkpoint, create_app_route,
    internal::{
        router::{PerseusRouter, PerseusRouterProps},
        shell::get_render_cfg,
    },
    plugins::PluginAction,
    templates::TemplateNodeType,
    DomNode,
};
use sycamore::prelude::view;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// The entrypoint into the app itself. This will be compiled to Wasm and actually executed, rendering the rest of the app.
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let plugins = get_plugins::<DomNode>();

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
        .query_selector(&format!("#{}", get_app_root(&plugins)))
        .unwrap()
        .unwrap();

    // Create the route type we'll use for this app, based on the user's app definition
    create_app_route! {
        name => AppRoute,
        // The render configuration is injected verbatim into the HTML shell, so it certainly should be present
        render_cfg => &get_render_cfg().expect("render configuration invalid or not injected"),
        // TODO avoid unnecessary allocation here (major problem!)
        // The `G` parameter is ambient here for `RouteVerdict`
        templates => &get_templates_map::<G>(&get_plugins()),
        locales => &get_locales::<G>(&get_plugins())
    }
    // Create a new version of the router with that
    type PerseusRouterWithAppRoute<G> = PerseusRouter<G, AppRoute<TemplateNodeType>>;

    // Set up the properties we'll pass to the router
    let router_props = PerseusRouterProps {
        locales: get_locales(&plugins),
        error_pages: get_error_pages(&plugins),
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
