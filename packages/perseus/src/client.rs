use crate::errors::PluginError;
use crate::reactor::Reactor;
use crate::{
    checkpoint,
    plugins::PluginAction,
    router::{perseus_router, PerseusRouterProps},
    template::TemplateNodeType,
};
use crate::{i18n::TranslationsManager, stores::MutableStore, PerseusAppBase};
use fmterr::fmt_err;
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
///
/// Note that, by the time this, or any of our code, is executing, the user can
/// already see something due to engine-side rendering.
pub fn run_client<M: MutableStore, T: TranslationsManager>(
    app: impl Fn() -> PerseusAppBase<TemplateNodeType, M, T>,
) {
    let mut app = app();
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

    // Get the root we'll be injecting the router into
    let root = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector(&format!("#{}", app.get_root()?))
        .unwrap()
        .unwrap();

    // Get the error pages, just in case the reactor can't be instantiated
    let error_pages = app.get_error_pages();

    // Create the reactor
    match Reactor::try_from(app) {
        Ok(reactor) => {
            // We can use the reactor to render the whole app properly
            #[cfg(feature = "hydrate")]
                sycamore::hydrate_to(move |cx| perseus_router(cx, reactor), &root);
            #[cfg(not(feature = "hydrate"))]
            {
                // We have to delete the existing content before we can render the new stuff
                // (which should be the same)
                root.set_inner_html("");
                sycamore::render_to(move |cx| perseus_router(cx, reactor), &root);
            }
        },
        Err(err) => {
            // We don't have a reactor, but we can recover to an error page. A reactor failure
            // can't occur on the engine-side, which means, even if we should be using hydration,
            // it's useless. We're rendering a fresh error.
            root.set_inner_html("");
            sycamore::render_to(move |cx| Reactor::handle_critical_error(cx, err, error_pages), &root);
        }
    };
}

/// A convenience type wrapper for the type returned by nearly all client-side
/// entrypoints.
pub type ClientReturn = Result<(), JsValue>;
