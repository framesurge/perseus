#![doc = include_str!("../README.proj.md")]
/*!
## Features

- `translator-fluent` --- enables internationalization using [Fluent](https://projectfluent.org)
- `macros` (default) --- adds support for macros that will make your life much easier
- `dflt_engine` (default) --- adds support for the default engine-side mechanics (you would only not want this in extremely niche use-cases)
- `client_helpers` (default) --- adds useful helpers for managing the browser-side
- `hydrate` --- enables Sycamore's *experimental* hydration system (if you experience odd issues, try disabling this)
- `preload-wasm-on-redirect` --- *experimentally* preloads the Wasm bundle for locale redirections (this only partially works right now)
- `idb-freezing` --- enables utilities for freezing your app's state to IndexedDB in the browser (see the book)
- `live-reload` (default) --- enables reloading the browser automatically when you make changes to your app
- `hsr` (default) --- enables *hot state reloading*, which reloads the state of your app right before you made code changes in development, allowing you to pick up where you left off

## Packages

This is the API documentation for the core `perseus` package, which underlies all Perseus apps. Note that Perseus mostly uses [the book](https://arctic-hen7.github.io/perseus/en-US) for
documentation, and this should mostly be used as a secondary reference source. You can also find full usage examples [here](https://github.com/arctic-hen7/perseus/tree/main/examples).
*/

#![deny(missing_docs)]
// #![deny(missing_debug_implementations)] // TODO Pending sycamore-rs/sycamore#412
#![forbid(unsafe_code)]
#![recursion_limit = "256"] // TODO Do we need this anymore?

/// Utilities for working with the engine-side, particularly with regards to
/// setting up the entrypoint for your app's build/export/server processes.
#[cfg(not(target_arch = "wasm32"))]
pub mod engine;
/// Utilities surrounding [`ErrorPages`] and their management.
pub mod error_pages;
pub mod errors;
/// Utilities for internationalization, the process of making your app available
/// in multiple languages.
pub mod i18n;
/// Utilities for working with plugins.
pub mod plugins;
/// Utilities for working with the router. Note that you should only have to use
/// these when waiting for a page transition in normal use-cases.
pub mod router;
/// Utilities for working with the server. These are fairly low-level, and
/// are intended for use by those developing new server integrations.
#[cfg(not(target_arch = "wasm32"))]
pub mod server;
/// Utilities for working with Perseus' state platform.
pub mod state;
/// Utilities for working with immutable and mutable stores. See
/// [`ImmutableStore`] and [`MutableStore`] for details.
pub mod stores;
/// Utilities for working with templates and state generation. This is by far
/// the module you'll probably access the most.
pub mod template;
/// General utilities that may be useful while building Perseus apps.
pub mod utils;

#[cfg(not(target_arch = "wasm32"))]
mod build;
#[cfg(all(feature = "client-helpers", target_arch = "wasm32"))]
mod client;
#[cfg(not(target_arch = "wasm32"))]
mod export;
mod init;
mod macros;
mod page_data;
mod translator;

// The rest of this file is devoted to module structuring
// Re-exports
#[cfg(not(target_arch = "wasm32"))]
pub use http;
#[cfg(not(target_arch = "wasm32"))]
pub use http::Request as HttpRequest;
pub use sycamore_futures::spawn_local_scoped;
#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen_futures::spawn_local;
/// All HTTP requests use empty bodies for simplicity of passing them around.
/// They'll never need payloads (value in path requested).
#[cfg(not(target_arch = "wasm32"))]
pub type Request = HttpRequest<()>;
#[cfg(feature = "macros")]
pub use perseus_macro::{
    amalgamate_states, browser, browser_main, build_paths, build_state, engine, engine_main,
    global_build_state, head, main, main_export, make_rx, request_state, set_headers,
    should_revalidate, template, template_rx, test,
};
pub use sycamore::prelude::{DomNode, Html, HydrateNode, SsrNode};
pub use sycamore_router::{navigate, navigate_replace};

// All the items that should be available at the top-level for convenience
pub use crate::{
    error_pages::ErrorPages,
    errors::{ErrorCause, GenericErrorWithCause},
    init::*,
    template::{RenderFnResult, RenderFnResultWithCause, Template},
};
// Browser-side only
#[cfg(target_arch = "wasm32")]
pub use crate::utils::checkpoint;
#[cfg(all(feature = "client-helpers", target_arch = "wasm32"))]
pub use client::{run_client, ClientReturn};

/// Internal utilities for lower-level work.
#[cfg(not(target_arch = "wasm32"))]
pub mod internal {
    pub use crate::page_data::*;
    pub use crate::{build::*, export::*};
}
/// Internal utilities for logging. These are just re-exports so that users
/// don't have to have `web_sys` and `wasm_bindgen` to use `web_log!`.
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
pub mod log {
    pub use wasm_bindgen::JsValue;
    pub use web_sys::console::log_1 as log_js_value;
}

/// A series of imports needed by most Perseus apps, in some form. This should
/// be used in conjunction with the Sycamore prelude.
pub mod prelude {
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::utils::{cache_fallible_res, cache_res};
    pub use crate::web_log;
    #[cfg(feature = "macros")]
    pub use crate::{
        amalgamate_states, browser, browser_main, build_paths, build_state, engine, engine_main,
        global_build_state, head, main, main_export, make_rx, request_state, set_headers,
        should_revalidate, template, template_rx, test,
    };
    #[cfg(feature = "i18n")]
    pub use crate::{link, t};
    pub use crate::{
        ErrorCause, ErrorPages, GenericErrorWithCause, PerseusApp, PerseusRoot, RenderFnResult,
        RenderFnResultWithCause, Template,
    };
}
