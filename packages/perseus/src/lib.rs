/*!
 * Perseus is a blazingly fast frontend web development framework built in Rust with support for major rendering strategies,
 * reactivity without a virtual DOM, and extreme customizability. It wraps the lower-level capabilities of [Sycamore](https://github.com/sycamore-rs/sycamore)
 * and provides a NextJS-like API!
 *
 * - ✨ Supports static generation (serving only static resources)
 * - ✨ Supports server-side rendering (serving dynamic resources)
 * - ✨ Supports revalidation after time and/or with custom logic (updating rendered pages)
 * - ✨ Supports incremental regeneration (build on demand)
 * - ✨ Open build matrix (use any rendering strategy with anything else, mostly)
 * - ✨ CLI harness that lets you build apps with ease and confidence
 *
 * This is the documentation for the core Perseus crate, but there's also [a CLI](https://arctic-hen7.github.io/perseus/cli.html) and
 * [integrations](https://arctic-hen7.github.io/perseus/serving.html) to make serving apps easier!
 *
 * # Resources
 *
 * These docs will help you as a reference, but [the book](https://arctic-hen7.github.io/perseus) should be your first port of call for
 * learning about how to use Perseus and how it works.
 *
 * - [The Book](https://arctic-hen7.github.io/perseus)
 * - [GitHub repository](https://github.com/arctic-hen7/perseus)
 * - [Crate page](https://crates.io/crates/perseus)
 * - [Gitter chat](https://gitter.im/perseus-framework/community)
 * - [Discord server channel](https://discord.com/channels/820400041332179004/883168134331256892) (for Sycamore-related stuff)
 *
 * # Features
 *
 * Perseus performs internationalization using translators, each of which utilizes some translation engine, like [Fluent](https://projectfluent.org).
 * Each of the available translations are feature-gated, and can be enabled with the `translator-[engine-name]` feature. You can set
 * the default translator by setting the `translator-dflt-[engine-name]` (you of course can't have more than one default translator).
 * You can read more about this system [here](https://arctic-hen7.github.io/perseus/i18n.html).
 */

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
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
mod error_pages;
mod export;
mod i18n;
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
pub use wasm_bindgen_futures::spawn_local;
/// All HTTP requests use empty bodies for simplicity of passing them around. They'll never need payloads (value in path requested).
pub type Request = HttpRequest<()>;
pub use perseus_macro::{autoserde, head, make_rx, template, template_rx, test};
pub use sycamore::{generic_node::Html, DomNode, HydrateNode, SsrNode};
pub use sycamore_router::{navigate, navigate_replace, Route}; // TODO Should we be exporting `Route` anymore?

// TODO Restructure everything here (needs to stay the same until v0.4.0 though)

// Items that should be available at the root (this should be nearly everything used in a typical Perseus app)
pub use crate::error_pages::ErrorPages;
pub use crate::errors::{ErrorCause, GenericErrorWithCause};
pub use crate::plugins::{Plugin, PluginAction, Plugins};
pub use crate::shell::checkpoint;
pub use crate::template::{HeadFn, RenderFnResult, RenderFnResultWithCause, States, Template};
pub use crate::utils::{cache_fallible_res, cache_res};
/// Utilities for developing templates, particularly including return types for various rendering strategies.
pub mod templates {
    pub use crate::errors::{ErrorCause, GenericErrorWithCause};
    pub use crate::router::{RouterLoadState, RouterState};
    pub use crate::template::*;
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
    /// Internal utilities for building.
    pub mod build {
        pub use crate::build::*;
    }
    /// Internal utilities for exporting.
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
