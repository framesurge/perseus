#![doc = include_str!("../README.proj.md")]
/*!
## Features

- `translator-fluent` -- enables internationalization using [Fluent](https://projectfluent.org)
- `hydrate` -- enables Sycamore's *experimental* hydration system (if you experience odd issues, try disabling this)
- `preload-wasm-on-redirect` -- *experimentally* preloads the Wasm bundle for locale redirections (this only partially works right now)
- `idb-freezing` -- enables utilities for freezing your app's state to IndexedDB in the browser (see the book)
- `live-reload` (default) -- enables reloading the browser automatically when you make changes to your app
- `hsr` (default) -- enables *hot state reloading*, which reloads the state of your app right before you made code changes in development, allowing you to pick up where you left off

The remaining features are used internally by the other Perseus packages, and enabling them manually in your project will very likely wreak havoc unless you seriously know what you're doing!

- `tinker-plugins` -- makes tinker plugins be registered
- `server-side` -- enables various functions only used on the server-side (minimizes the client-side bundle)
- `standalone` -- makes Perseus able to be run as a standalone binary by changing some minor internal defaults

## Packages

This is the API documentation for the core `perseus` package, which underlies all Perseus apps. Note that Perseus mostly uses [the book](https://arctic-hen7.github.io/perseus/en-US) for
documentation, and this should mostly be used as a secondary reference source. You can also find full usage examples [here](https://github.com/arctic-hen7/perseus/tree/main/examples).
*/

#![deny(missing_docs)]
// #![deny(missing_debug_implementations)] // TODO Pending sycamore-rs/sycamore#412
#![forbid(unsafe_code)]
#![recursion_limit = "256"] // TODO Do we need this anymore?

pub mod errors;
/// Utilities for working with plugins.
pub mod plugins;
/// Utilities for working with Perseus' state platform.
pub mod state;
/// Utilities for working with immutable and mutable stores. You can learn more about these in the book.
pub mod stores;

mod build;
#[cfg(feature = "client-helpers")]
mod client;
#[cfg(feature = "builder")]
mod engine;
mod error_pages;
mod export;
mod i18n;
mod init;
mod macros;
mod router;
mod server;
mod shell;
mod template;
mod translator;
mod utils;

// The rest of this file is devoted to module structuring
// Re-exports
pub use http;
pub use http::Request as HttpRequest;
pub use sycamore_futures::spawn_local_scoped;
/// All HTTP requests use empty bodies for simplicity of passing them around. They'll never need payloads (value in path requested).
pub type Request = HttpRequest<()>;
#[cfg(feature = "client-helpers")]
pub use client::{run_client, ClientReturn};
pub use perseus_macro::{autoserde, head, main, make_rx, template, template_rx, test};
pub use sycamore::prelude::{DomNode, Html, HydrateNode, SsrNode};
pub use sycamore_router::{navigate, navigate_replace, Route}; // TODO Should we be exporting `Route` anymore?

// TODO Restructure everything here (needs to stay the same until v0.4.0 though)

// Items that should be available at the root (this should be nearly everything used in a typical Perseus app)
pub use crate::error_pages::ErrorPages;
pub use crate::errors::{ErrorCause, GenericErrorWithCause};
pub use crate::plugins::{Plugin, PluginAction, Plugins};
pub use crate::shell::checkpoint;
pub use crate::template::{HeadFn, RenderFnResult, RenderFnResultWithCause, States, Template};
pub use crate::utils::{cache_fallible_res, cache_res};
// Everything in the `init.rs` file should be available at the top-level for convenience
pub use crate::init::*;
/// Utilities for developing templates, particularly including return types for various rendering strategies.
pub mod templates {
    pub use crate::errors::{ErrorCause, GenericErrorWithCause};
    pub use crate::router::{RouterLoadState, RouterState};
    pub use crate::template::*;
}
/// Utilities for building an app.
#[cfg(feature = "builder")]
pub mod builder {
    pub use crate::engine::*;
}
/// A series of exports that should be unnecessary for nearly all uses of Perseus. These are used principally in developing alternative
/// engines.
pub mod internal {
    /// Internal utilities for working with internationalization.
    pub mod i18n {
        pub use crate::i18n::*;
        #[doc(hidden)]
        pub use crate::macros::DFLT_TRANSLATIONS_DIR;
        pub use crate::translator::*;
    }
    /// Internal utilities for working with the serving process. These will be useful for building integrations for hosting Perseus
    /// on different platforms.
    pub mod serve {
        pub use crate::server::*;
    }
    /// Internal utilities for working with the Perseus router.
    pub mod router {
        pub use crate::router::*;
    }
    /// Internal utilities for working with error pages.
    pub mod error_pages {
        pub use crate::error_pages::*;
    }
    /// Internal utilities for working with the app shell.
    pub mod shell {
        pub use crate::shell::*;
    }
    /// Internal utilities for building apps at a very low level.
    pub mod build {
        pub use crate::build::*;
    }
    /// Internal utilities for exporting apps at a very low level.
    pub mod export {
        pub use crate::export::*;
    }
    pub use crate::utils::{get_path_prefix_client, get_path_prefix_server};
    /// Internal utilities for logging. These are just re-exports so that users don't have to have `web_sys` and `wasm_bindgen` to use `web_log!`.
    pub mod log {
        pub use wasm_bindgen::JsValue;
        pub use web_sys::console::log_1 as log_js_value;
    }
}
