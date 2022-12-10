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
#![recursion_limit = "256"] // TODO Do we need this anymore?

/// Utilities for working with the engine-side, particularly with regards to
/// setting up the entrypoint for your app's build/export/server processes.
#[cfg(not(target_arch = "wasm32"))]
pub mod engine;
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
/// Utilities surrounding `ErrorViews` and their management.
pub mod error_views;

#[cfg(all(feature = "client-helpers", target_arch = "wasm32"))]
mod client;
mod init;
mod page_data;
mod translator;
/// Utilities for working with typed paths.
pub mod path;
/// The core of the Perseus state generation system.
#[cfg(not(target_arch = "wasm32"))]
pub mod turbine;
/// The core of the Perseus browser-side system. This is used on the engine-side as well
/// for rendering.
pub mod reactor;

// The rest of this file is devoted to module structuring
// Re-exports
#[cfg(not(target_arch = "wasm32"))]
pub use http;
#[cfg(not(target_arch = "wasm32"))]
pub use http::Request as HttpRequest;
pub use sycamore_futures::spawn_local_scoped;

/// All HTTP requests use empty bodies for simplicity of passing them around.
/// They'll never need payloads (value in path requested).
///
/// **Warning:** on the browser-side, this is defined as `()`.
#[cfg(not(target_arch = "wasm32"))]
pub type Request = HttpRequest<()>;
#[cfg(target_arch = "wasm32")]
pub type Request = ();

#[cfg(feature = "macros")]
pub use perseus_macro::*;
pub use sycamore::prelude::{DomNode, Html, HydrateNode, SsrNode};
pub use sycamore_router::{navigate, navigate_replace};

// Browser-side only
#[cfg(target_arch = "wasm32")]
pub use crate::utils::checkpoint;
#[cfg(all(feature = "client-helpers", target_arch = "wasm32"))]
pub use client::{run_client, ClientReturn};

/// Internal utilities for lower-level work.
#[cfg(not(target_arch = "wasm32"))]
pub mod internal {
    pub use crate::page_data::*;
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
    pub use crate::error_views::ErrorViews;
    pub use crate::errors::{ErrorCause, GenericErrorWithCause};
    pub use crate::init::*;
    pub use crate::state::{RxResult, RxResultRef, SerdeInfallible, BuildPaths, StateGeneratorInfo};
    pub use crate::reactor::Reactor;
    pub use crate::template::{
        RenderFnResult, RenderFnResultWithCause,
        Template, Capsule
    };

    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::utils::{cache_fallible_res, cache_res};
    pub use crate::web_log;
    pub use crate::{
        blame_err, make_blamed_err, Request, spawn_local_scoped
    };
    #[cfg(feature = "macros")]
    pub use crate::{
        browser, browser_main, browser_only_fn, engine, engine_main, engine_only_fn, main,
        main_export, template, template_rx, test, ReactiveState, UnreactiveState,
    };
    #[cfg(any(feature = "translator-fluent", feature = "translator-lightweight"))]
    pub use crate::{link, t};
}

