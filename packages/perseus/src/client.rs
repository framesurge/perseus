use crate::{
    checkpoint,
    plugins::PluginAction,
    router::{perseus_router, PerseusRouterProps},
    template::TemplateNodeType,
};
use crate::{i18n::TranslationsManager, stores::MutableStore, PerseusAppBase};
use std::collections::HashMap;
use wasm_bindgen::JsValue;

/// The entrypoint into the app itself. This will be compiled to Wasm and
/// actually executed, rendering the rest of the app. Runs the app in the
/// browser on the client-side. This is designed to be executed in a function
/// annotated with `#[wasm_bindgen]`.
///
/// This is entirely engine-agnostic, using only the properties from the given
/// `PerseusApp`.
///
/// For consistency with `run_dflt_engine`, this takes a function that returns
/// the `PerseusApp`.
pub fn run_client<M: MutableStore, T: TranslationsManager>(
    app: impl Fn() -> PerseusAppBase<TemplateNodeType, M, T>,
) -> Result<(), JsValue> {
    let mut app = app();
    let plugins = app.get_plugins();
    let panic_handler = app.take_panic_handler();

    checkpoint("begin");

    // Handle panics (this works for unwinds and aborts)
    std::panic::set_hook(Box::new(move |panic_info| {
        // Print to the console in development
        #[cfg(debug_assertions)]
        console_error_panic_hook::hook(panic_info);
        // If the user wants a little warning dialogue, create that
        if let Some(panic_handler) = &panic_handler {
            panic_handler(panic_info);
        }
    }));

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
        render_cfg: get_render_cfg().expect("render configuration invalid or not injected"),
        pss_max_size: app.get_pss_max_size(),
    };

    // At this point, the user can already see something from the server-side
    // rendering, so we now have time to figure out exactly what to render.
    // Having done that, we can render/hydrate, depending on the feature flags.
    // All that work is done inside the router.

    // This top-level context is what we use for everything, allowing page state to
    // be registered and stored for the lifetime of the app
    #[cfg(feature = "hydrate")]
    sycamore::hydrate_to(move |cx| perseus_router(cx, router_props), &root);
    #[cfg(not(feature = "hydrate"))]
    {
        // We have to delete the existing content before we can render the new stuff
        // (which should be the same)
        root.set_inner_html("");
        sycamore::render_to(move |cx| perseus_router(cx, router_props), &root);
    }

    Ok(())
}

/// A convenience type wrapper for the type returned by nearly all client-side
/// entrypoints.
pub type ClientReturn = Result<(), JsValue>;

/// Gets the render configuration from the JS global variable
/// `__PERSEUS_RENDER_CFG`, which should be inlined by the server. This will
/// return `None` on any error (not found, serialization failed, etc.), which
/// should reasonably lead to a `panic!` in the caller.
fn get_render_cfg() -> Option<HashMap<String, String>> {
    let val_opt = web_sys::window().unwrap().get("__PERSEUS_RENDER_CFG");
    let js_obj = match val_opt {
        Some(js_obj) => js_obj,
        None => return None,
    };
    // The object should only actually contain the string value that was injected
    let cfg_str = match js_obj.as_string() {
        Some(cfg_str) => cfg_str,
        None => return None,
    };
    let render_cfg = match serde_json::from_str::<HashMap<String, String>>(&cfg_str) {
        Ok(render_cfg) => render_cfg,
        Err(_) => return None,
    };

    Some(render_cfg)
}
