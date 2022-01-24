use crate::error_pages::ErrorPages;
use crate::i18n::Locales;
use crate::i18n::TranslationsManager;
use crate::state::GlobalStateCreator;
use crate::stores::{ImmutableStore, MutableStore};
use crate::template::ArcTemplateMap;
use crate::SsrNode;
use std::collections::HashMap;

/// The options for setting up all server integrations. This should be literally constructed, as nothing is optional. If integrations need further properties,
/// they should expose their own options in addition to these. These should be accessed through an `Arc`/`Rc` for integration developers.
#[derive(Debug)]
pub struct ServerOptions {
    /// The location on the filesystem of your JavaScript bundle.
    pub js_bundle: String,
    /// The location on the filesystem of your Wasm bundle.
    pub wasm_bundle: String,
    /// The location on the filesystem of your JS bundle converted from your Wasm bundle. This isn't required, and if you haven't generated this, you should provide a fake path.
    pub wasm_js_bundle: String,
    /// The location on the filesystem of your `index.html` file.
    // TODO Should this actually be a raw string of HTML so plugins can inject efficiently?
    pub index: String,
    /// A `HashMap` of your app's templates by their paths.
    pub templates_map: ArcTemplateMap<SsrNode>,
    /// The locales information for the app.
    pub locales: Locales,
    /// The HTML `id` of the element at which to render Perseus. On the server-side, interpolation will be done here in a highly
    /// efficient manner by not parsing the HTML, so this MUST be of the form `<div id="root_id">` in your markup (double or single
    /// quotes, `root_id` replaced by what this property is set to).
    pub root_id: String,
    /// The location of the JS interop snippets to be served as static files.
    pub snippets: String,
    /// The error pages for the app. These will be server-rendered if an initial load fails.
    pub error_pages: ErrorPages<SsrNode>,
    /// The directory to serve static content from, which will be mapped to `/.perseus/static`in the browser.
    pub static_dir: Option<String>,
    /// A map of URLs to act as aliases for certain static resources. These are particularly designed for things like a site manifest or
    /// favicons, which should be stored in a static directory, but need to be aliased at a path like `/favicon.ico`.
    pub static_aliases: HashMap<String, String>,
}

/// The full set of properties that all server integrations take.
#[derive(Debug)]
pub struct ServerProps<M: MutableStore, T: TranslationsManager> {
    /// The options for setting up the server.
    pub opts: ServerOptions,
    /// An immutable store to use.
    pub immutable_store: ImmutableStore,
    /// A mutable store to use.
    pub mutable_store: M,
    /// A translations manager to use.
    pub translations_manager: T,
    /// The global state creator. This is used to avoid issues with `async` and cloning in Actix Web.
    pub global_state_creator: GlobalStateCreator,
}
