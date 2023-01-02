use crate::reactor::Reactor;
use crate::{checkpoint, plugins::PluginAction, template::BrowserNodeType};
use crate::{i18n::TranslationsManager, init::PerseusAppBase, stores::MutableStore};
use sycamore::prelude::create_scope;
#[cfg(feature = "hydrate")]
use sycamore::utils::hydrate::with_hydration_context;
use wasm_bindgen::JsValue;
use web_sys::{CustomEvent, CustomEventInit};

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
///
/// This function performs all error handling internally, and will do its level
/// best not to fail, including through setting panic handlers.
pub fn run_client<M: MutableStore, T: TranslationsManager>(
    app: impl Fn() -> PerseusAppBase<BrowserNodeType, M, T>,
) {
    let mut app = app();
    // The latter of these is a clone of the handler used for other errors
    let (general_panic_handler, view_panic_handler) = app.take_panic_handlers();

    checkpoint("begin");

    // Handle panics (this works for both unwinds and aborts)
    std::panic::set_hook(Box::new(move |panic_info| {
        // Print to the console in development (details are withheld in production,
        // they'll just get 'unreachable executed')
        #[cfg(debug_assertions)]
        console_error_panic_hook::hook(panic_info);

        // In case anything after this fails (which, since we're calling out to
        // view rendering and user code, is reasonably likely), put out a console
        // message to try to explain things (differentiated for end users)
        #[cfg(debug_assertions)]
        crate::web_log!("[CRITICAL ERROR]: Perseus has panicked! An error message has hopefully been displayed on your screen explaining this; if not, then something has gone terribly wrong, and, unless your code is panicking, you should report this as a bug. (If you're seeing this as an end user, please report it to the website administrator.)");
        #[cfg(not(debug_assertions))]
        crate::web_log!("[CRITICAL ERROR]: Perseus has panicked! An error message has hopefully been displayed on your screen explaining this; if not, then reloading the page might help.");

        // Make it clear that apps compiled with unwinding panics might continue now
        // (for completeness)
        #[cfg(panic = "unwind")]
        crate::web_log!("[WARNING]: The app has been compiled with unwinding panics, and it is possible that the app will now continue normal operation if this panic is handled.");

        // Make sure the load event is submitted so interaction isn't totally suspended
        // forever
        dispatch_loaded(false, true);

        // Run the user's arbitrary panic handler
        if let Some(panic_handler) = &general_panic_handler {
            panic_handler(panic_info);
        }

        // Try to render an error page
        Reactor::handle_panic(panic_info, view_panic_handler.clone());

        // There is **not** a plugin opportunity here because that would require
        // cloning the plugins into here. Any of that can be managed by the
        // arbitrary user-given panic handler. Please appreciate how
        // unreasonably difficult it is to get variables into a panic
        // hook.
    }));

    let plugins = app.plugins.clone();
    let error_views;
    #[cfg(debug_assertions)]
    {
        error_views = app.error_views.clone().unwrap_or_default();
    }
    #[cfg(not(debug_assertions))]
    {
        error_views = app
            .error_views
            .clone()
            .expect("you must provide your own error views in production");
    }

    // This variable acts as a signal to determine whether or not there was a
    // show-stopping failure that should trigger root scope disposal
    // (terminating Perseus and rendering the app inoperable)
    let mut running = true;
    // === IF THIS DISPOSER IS CALLED, PERSEUS WILL TERMINATE! ===
    let app_disposer = create_scope(|cx| {
        let core = move || {
            // Create the reactor
            match Reactor::try_from(app) {
                Ok(reactor) => {
                    // We're away!
                    reactor.add_self_to_cx(cx);
                    let reactor = Reactor::from_cx(cx);
                    reactor.start(cx)
                }
                Err(err) => {
                    // We don't have a reactor, so render a critical popup error, hoping the user
                    // can see something prerendered that makes sense (this
                    // displays and everything)
                    Reactor::handle_critical_error(cx, err, &error_views);
                    // We can't do anything without a reactor
                    false
                }
            }
        };

        // If we're using hydration, everything has to be done inside a hydration
        // context (because of all the custom view handling)
        #[cfg(feature = "hydrate")]
        {
            running = with_hydration_context(|| core());
        }
        #[cfg(not(feature = "hydrate"))]
        {
            running = core();
        }
    });

    dispatch_loaded(running, false);

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

/// Regardless of whether an error or a proper render was triggered, allow the
/// browser to send through click events etc. (these are suspended until we've
/// rendered to improve user experience and apparent responsiveness, but this
/// has no impact on machine-measured metrics on UX). This also allows neat
/// interoperability with other code running outside Perseus.
///
/// This will provide as part of the event whether or not the app is running. If
/// this is `false`, the app will terminate immediately afterward. If it is
/// `null` (thank you dynamic typing!), the app has panicked, and further
/// behavior is unspecified, because the app might have been built with
/// unwinding panics, and there could be a user function for catching panics.
fn dispatch_loaded(running: bool, panic: bool) {
    let document = web_sys::window().unwrap().document().unwrap();
    let mut ev_init = CustomEventInit::new();
    // We provide whether or not the app is actually running to this
    ev_init.detail(&if panic { JsValue::NULL } else { running.into() });
    let ev = CustomEvent::new_with_event_init_dict("__perseus_loaded", &ev_init).unwrap();
    document.dispatch_event(&ev).unwrap();
}
