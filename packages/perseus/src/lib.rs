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

/// Utilities for building your app.
pub mod build;
mod client_translations_manager;
/// Utilities for creating custom config managers, as well as the default `FsConfigManager`.
pub mod config_manager;
mod decode_time_str;
pub mod errors;
mod locales;
mod log;
mod macros;
/// Utilities regarding routing.
pub mod router;
/// Utilities for serving your app. These are platform-agnostic, and you probably want an integration like [perseus-actix-web](https://crates.io/crates/perseus-actix-web).
pub mod serve;
/// Utilities to do with the app shell. You probably don't want to delve into here.
pub mod shell;
/// Utilities to do with templating. This is where the bulk of designing apps lies.
pub mod template;
/// Utilities for creating custom translations managers, as well as the default `FsTranslationsManager`.
pub mod translations_manager;
/// Utilities regarding translators, including the default `FluentTranslator`.
pub mod translator;

pub use http;
pub use http::Request as HttpRequest;
/// All HTTP requests use empty bodies for simplicity of passing them around. They'll never need payloads (value in path requested).
pub type Request = HttpRequest<()>;
pub use sycamore::{generic_node::GenericNode, DomNode, SsrNode};
pub use sycamore_router::Route;

pub use crate::build::{build_app, build_template, build_templates_for_locale};
pub use crate::client_translations_manager::ClientTranslationsManager;
pub use crate::config_manager::{ConfigManager, FsConfigManager};
pub use crate::errors::{err_to_status_code, ErrorCause};
pub use crate::locales::Locales;
pub use crate::serve::{get_page, get_render_cfg};
pub use crate::shell::{app_shell, ErrorPages};
pub use crate::template::{States, StringResult, StringResultWithCause, Template, TemplateMap};
pub use crate::translations_manager::{FsTranslationsManager, TranslationsManager};
pub use crate::translator::{Translator, TRANSLATOR_FILE_EXT};
