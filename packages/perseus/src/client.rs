use crate::reactor::Reactor;
use crate::{checkpoint, plugins::PluginAction, template::TemplateNodeType};
use crate::{i18n::TranslationsManager, init::PerseusAppBase, stores::MutableStore};
use sycamore::prelude::create_scope;
use sycamore::utils::hydrate::with_hydration_context;
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
    // TODO New system
    std::panic::set_hook(Box::new(move |panic_info| {
        // Print to the console in development
        #[cfg(debug_assertions)]
        console_error_panic_hook::hook(panic_info);
        // If the user wants a little warning dialogue, create that
        if let Some(panic_handler) = &panic_handler {
            panic_handler(panic_info);
        }
    }));

    let plugins = app.get_plugins();

    // This variable acts as a signal to determine whether or not there was a
    // show-stopping failure that should trigger root scope disposal
    // (terminating Perseus and rendering the app inoperable)
    let mut running = true;
    // === IF THIS DISPOSER IS CALLED, PERSEUS WILL TERMINATE! ===
    let app_disposer = create_scope(|cx| {
        let core = move || {
            // Get the error views, just in case the reactor can't be instantiated
            let error_views = app.get_error_views();

            // Create the reactor
            match Reactor::try_from(app) {
                Ok(reactor) => {
                    // We're away!
                    reactor.add_self_to_cx(cx);
                    let reactor = Reactor::from_cx(cx);
                    running = reactor.start(cx);
                }
                Err(err) => {
                    // We don't have a reactor, so render a critical popup error, hoping the user
                    // can see something prerendered that makes sense (this
                    // displays and everything)
                    Reactor::handle_critical_error(cx, &err, &error_views);
                    // We can't do anything without a reactor
                    running = false;
                }
            };
        };

        // If we're using hydration, everything has to be done inside a hydration
        // context (because of all the custom view handling)
        #[cfg(feature = "hydrate")]
        with_hydration_context(|| core());
        #[cfg(not(feature = "hydrate"))]
        core();
    });

    // If we failed, terminate
    if !running {
        // SAFETY We're outside the app's scope.
        unsafe { app_disposer.dispose() }
        // This is one of the best places in Perseus for crash analytics
        plugins
            .functional_actions
            .client_actions
            .crash
            .run((), plugins.get_plugin_data())
            .expect("plugin action on crash failed");

        // Goodbye, dear friends.
    }
}

/// A convenience type wrapper for the type returned by nearly all client-side
/// entrypoints.
pub type ClientReturn = Result<(), JsValue>;
