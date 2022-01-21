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
#![recursion_limit = "256"]

pub mod errors;
/// Utilities for working with plugins.
pub mod plugins;
/// Utilities for working with immutable and mutable stores. You can learn more about these in the book.
pub mod stores;

mod build;
mod cache_res;
mod client_translations_manager;
mod decode_time_str;
mod default_headers;
mod error_pages;
mod export;
mod global_state;
mod html_shell;
mod locale_detector;
mod locales;
mod log;
mod macros;
mod page_data;
mod page_state_store;
mod path_prefix;
mod router;
mod rx_state;
mod server;
mod shell;
mod template;
mod test;
mod translations_manager;
mod translator;

// The rest of this file is devoted to module structuring
// Re-exports
pub use http;
pub use http::Request as HttpRequest;
pub use wasm_bindgen_futures::spawn_local;
/// All HTTP requests use empty bodies for simplicity of passing them around. They'll never need payloads (value in path requested).
pub type Request = HttpRequest<()>;
pub use perseus_macro::{autoserde, head, make_rx, template, template2, test};
pub use sycamore::{generic_node::Html, DomNode, HydrateNode, SsrNode};
pub use sycamore_router::{navigate, Route};

// Items that should be available at the root (this should be nearly everything used in a typical Perseus app)
pub use crate::cache_res::{cache_fallible_res, cache_res};
pub use crate::error_pages::ErrorPages;
pub use crate::errors::{ErrorCause, GenericErrorWithCause};
pub use crate::plugins::{Plugin, PluginAction, Plugins};
pub use crate::shell::checkpoint;
pub use crate::template::{HeadFn, RenderFnResult, RenderFnResultWithCause, States, Template};
/// Utilities for developing templates, particularly including return types for various rendering strategies.
pub mod templates {
    pub use crate::errors::{ErrorCause, GenericErrorWithCause};
    pub use crate::router::{RouterLoadState, RouterState};
    pub use crate::template::*;
    // The engine needs to know whether or not to use hydration, this is how we pass those feature settings through
    #[cfg(not(feature = "hydrate"))]
    #[doc(hidden)]
    pub type TemplateNodeType = sycamore::DomNode;
    #[cfg(feature = "hydrate")]
    #[doc(hidden)]
    pub type TemplateNodeType = sycamore::HydrateNode;
}
// TODO (v0.4.0) Refactor to put several more things inside here (everything to do with generation functions)
/// Utilities for working with state.
pub mod state {
    pub use crate::global_state::GlobalStateCreator;
    pub use crate::page_state_store::PageStateStore;
    pub use crate::rx_state::{MakeRx, MakeUnrx};
}
/// A series of exports that should be unnecessary for nearly all uses of Perseus. These are used principally in developing alternative
/// engines.
pub mod internal {
    /// Internal utilities for working with internationalization.
    pub mod i18n {
        pub use crate::client_translations_manager::*;
        pub use crate::locale_detector::*;
        pub use crate::locales::*;
        #[doc(hidden)]
        pub use crate::macros::DFLT_TRANSLATIONS_DIR;
        pub use crate::translations_manager::*;
        pub use crate::translator::*;
    }
    /// Internal utilities for working with the serving process. These will be useful for building integrations for hosting Perseus
    /// on different platforms.
    pub mod serve {
        pub use crate::html_shell::*;
        pub use crate::page_data::*;
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
    pub use crate::path_prefix::{get_path_prefix_client, get_path_prefix_server};
    /// Internal utilities for logging. These are just re-exports so that users don't have to have `web_sys` and `wasm_bindgen` to use `web_log!`.
    pub mod log {
        pub use wasm_bindgen::JsValue;
        pub use web_sys::console::log_1 as log_js_value;
    }
}
