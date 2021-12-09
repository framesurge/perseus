use crate::error_pages::ErrorPages;
use crate::locales::Locales;
use crate::template::TemplateMap;
use crate::SsrNode;
use std::collections::HashMap;

/// The options for setting up all server integrations. This should be literally constructed, as nothing is optional. If integrations need further properties,
/// they should expose their own options in addition to these. These should be accessed through an `Arc`/`Rc` for integration developers.
pub struct ServerOptions {
    /// The location on the filesystem of your JavaScript bundle.
    pub js_bundle: String,
    /// The location on the filesystem of your Wasm bundle.
    pub wasm_bundle: String,
    /// The location on the filesystem of your `index.html` file.
    // TODO Should this actually be a raw string of HTML so plugins can inject efficiently?
    pub index: String,
    /// A `HashMap` of your app's templates by their paths.
    pub templates_map: TemplateMap<SsrNode>,
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
    /// Directories to serve static content from, mapping URL to folder path. Note that the URL provided will be gated behind
    /// `.perseus/static/`, and must have a leading `/`. If you're using a CMS instead, you should set these up outside the Perseus
    /// server (but they might still be on the same machine, you can still add more routes after Perseus is configured).
    pub static_dirs: HashMap<String, String>,
    /// A map of URLs to act as aliases for certain static resources. These are particularly designed for things like a site manifest or
    /// favicons, which should be stored in a static directory, but need to be aliased at a path like `/favicon.ico`.
    pub static_aliases: HashMap<String, String>,
}
