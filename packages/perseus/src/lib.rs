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
#![deny(missing_debug_implementations)]
#![recursion_limit = "256"] // TODO Do we need this anymore?

/// Utilities for working with the engine-side, particularly with regards to
/// setting up the entrypoint for your app's build/export/server processes.
#[cfg(engine)]
pub mod engine;
/// Utilities surrounding `ErrorViews` and their management.
pub mod error_views;
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
#[cfg(engine)]
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

#[cfg(all(feature = "client-helpers", client))]
mod client;
mod init;
mod page_data;
/// Utilities for working with typed paths.
pub mod path;
/// The core of the Perseus browser-side system. This is used on the engine-side
/// as well for rendering.
pub mod reactor;
mod translator;
/// The core of the Perseus state generation system.
#[cfg(engine)]
pub mod turbine;

// The rest of this file is devoted to module structuring
// Re-exports
#[cfg(engine)]
pub use http;
#[cfg(engine)]
pub use http::Request as HttpRequest;

/// All HTTP requests use empty bodies for simplicity of passing them around.
/// They'll never need payloads (value in path requested).
///
/// **Warning:** on the browser-side, this is defined as `()`.
#[cfg(engine)]
pub type Request = HttpRequest<()>;
/// All HTTP requests use empty bodies for simplicity of passing them around.
/// They'll never need payloads (value in path requested).
///
/// **Warning:** on the browser-side, this is defined as `()`.
#[cfg(client)]
pub type Request = ();

#[cfg(feature = "macros")]
pub use perseus_macro::*;

// Browser-side only
#[cfg(client)]
pub use crate::utils::checkpoint;
#[cfg(all(feature = "client-helpers", client))]
pub use client::{run_client, ClientReturn};

/// Internal utilities for lower-level work.
#[cfg(engine)]
pub mod internal {
    pub use crate::page_data::*;
}
/// Internal utilities for logging. These are just re-exports so that users
/// don't have to have `web_sys` and `wasm_bindgen` to use `web_log!`.
#[cfg(client)]
#[doc(hidden)]
pub mod log {
    pub use wasm_bindgen::JsValue;
    pub use web_sys::console::log_1 as log_js_value;
}

/// An alias for `DomNode`, `HydrateNode`, or `SsrNode`, depending on the
/// `hydrate` feature flag and compilation target.
///
/// You **should not** use this in your return types (e.g.
/// `View<PerseusNodeType>`), there you should use a `G: Html` generic.
/// This is intended for `lazy_static!`s and the like, for capsules. See
/// the book and capsule examples for further details.
#[cfg(engine)]
pub type PerseusNodeType = sycamore::web::SsrNode;
/// An alias for `DomNode`, `HydrateNode`, or `SsrNode`, depending on the
/// `hydrate` feature flag and compilation target.
///
/// You **should not** use this in your return types (e.g.
/// `View<PerseusNodeType>`), there you should use a `G: Html` generic.
/// This is intended for `lazy_static!`s and the like, for capsules. See
/// the book and capsule examples for further details.
#[cfg(all(client, not(feature = "hydrate")))]
pub type PerseusNodeType = sycamore::web::DomNode;
/// An alias for `DomNode`, `HydrateNode`, or `SsrNode`, depending on the
/// `hydrate` feature flag and compilation target.
///
/// You **should not** use this in your return types (e.g.
/// `View<PerseusNodeType>`), there you should use a `G: Html` generic.
/// This is intended for `lazy_static!`s and the like, for capsules. See
/// the book and capsule examples for further details.
#[cfg(all(client, feature = "hydrate"))]
pub type PerseusNodeType = sycamore::web::HydrateNode;

/// A series of imports needed by most Perseus apps, in some form. This should
/// be used in conjunction with the Sycamore prelude.
pub mod prelude {
    pub use crate::error_views::ErrorViews;
    // Target-gating doesn't matter, because the prelude is intended to be used all
    // at once
    #[cfg(engine)]
    pub use crate::errors::{BlamedError, ErrorBlame};
    pub use crate::init::*;
    pub use crate::reactor::Reactor;
    pub use crate::state::{BuildPaths, RxResult, RxResultRx, SerdeInfallible, StateGeneratorInfo};
    pub use crate::template::{Capsule, Template};
    pub use sycamore::web::Html;
    pub use sycamore_router::{navigate, navigate_replace};

    #[cfg(engine)]
    pub use crate::utils::{cache_fallible_res, cache_res};
    pub use crate::web_log;
    #[cfg(any(feature = "translator-fluent", feature = "translator-lightweight"))]
    pub use crate::{link, t};
    pub use crate::{PerseusNodeType, Request};
    #[cfg(feature = "macros")]
    pub use perseus_macro::*;
    pub use sycamore_futures::spawn_local_scoped;
}
