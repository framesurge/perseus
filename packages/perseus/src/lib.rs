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

/// TODO
#[cfg(not(target_arch = "wasm32"))]
pub mod engine;
/// TODO
pub mod error_pages;
pub mod errors;
/// TODO
pub mod i18n;
/// Utilities for working with plugins.
pub mod plugins;
/// TODO
pub mod router;
/// TODO
#[cfg(not(target_arch = "wasm32"))]
pub mod server;
/// Utilities for working with Perseus' state platform.
pub mod state;
/// Utilities for working with immutable and mutable stores. You can learn more
/// about these in the book.
pub mod stores;
/// TODO
pub mod template;
/// TODO
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
#[cfg(target_arch = "wasm32")]
mod shell;
mod translator;

// The rest of this file is devoted to module structuring
// Re-exports
#[cfg(not(target_arch = "wasm32"))]
pub use http;
#[cfg(not(target_arch = "wasm32"))]
pub use http::Request as HttpRequest;
pub use sycamore_futures::spawn_local_scoped;
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
// Engine-side only
#[cfg(not(target_arch = "wasm32"))]
pub use crate::template::States;
// Browser-side only
#[cfg(target_arch = "wasm32")]
pub use crate::shell::checkpoint;
#[cfg(all(feature = "client-helpers", target_arch = "wasm32"))]
pub use client::{run_client, ClientReturn};

// Internal utilities for lower-level work
/// TODO
#[cfg(not(target_arch = "wasm32"))]
pub mod internal {
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

// TODO Fix feature flags
