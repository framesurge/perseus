use crate::{
    checkpoint,
    internal::router::{perseus_router, PerseusRouterProps},
    plugins::PluginAction,
    shell::get_render_cfg,
    templates::TemplateNodeType
};
use wasm_bindgen::JsValue;

use crate::{i18n::TranslationsManager, stores::MutableStore, PerseusAppBase};

/// The entrypoint into the app itself. This will be compiled to Wasm and actually executed, rendering the rest of the app.
/// Runs the app in the browser on the client-side. This is designed to be executed in a function annotated with `#[wasm_bindgen]`.
///
/// This is entirely engine-agnostic, using only the properties from the given `PerseusApp`.
pub fn run_client<M: MutableStore, T: TranslationsManager>(
    app: PerseusAppBase<TemplateNodeType, M, T>,
) -> Result<(), JsValue> {
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

    // Set up the properties we'll pass to the router
    let router_props = PerseusRouterProps {
        locales: app.get_locales(),
        error_pages: app.get_error_pages(),
        templates: app.get_templates_map(),
        render_cfg: get_render_cfg().expect("render configuration invalid or not injected")
    };

    // This top-level context is what we use for everything, allowing page state to be registered and stored for the lifetime of the app
    sycamore::render_to(move |cx| perseus_router(cx, router_props), &root);

    Ok(())
}

/// A convenience type wrapper for the type returned by nearly all client-side entrypoints.
pub type ClientReturn = Result<(), JsValue>;
