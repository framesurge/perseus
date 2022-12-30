/// The options for setting up all server integrations. This should be literally
/// constructed, as nothing is optional. If integrations need further
/// properties, they should expose their own options in addition to these.
#[derive(Debug, Clone)]
pub struct ServerOptions {
    /// The location on the filesystem of your JavaScript bundle.
    pub js_bundle: String,
    /// The location on the filesystem of your Wasm bundle.
    pub wasm_bundle: String,
    /// The location on the filesystem of your JS bundle converted from your
    /// Wasm bundle. This isn't required, and if you haven't generated this, you
    /// should provide a fake path.
    pub wasm_js_bundle: String,
    /// The location of the JS interop snippets to be served as static files.
    pub snippets: String,
}
#[cfg(feature = "dflt-engine")]
impl Default for ServerOptions {
    fn default() -> Self {
        Self {
            js_bundle: "dist/pkg/perseus_engine.js".to_string(),
            // Our crate has the same name, so this will be predictable
            wasm_bundle: "dist/pkg/perseus_engine_bg.wasm".to_string(),
            // This probably won't exist, but on the off chance that the user needs to support older
            // browsers, we'll provide it anyway
            wasm_js_bundle: "dist/pkg/perseus_engine_bg.wasm.js".to_string(),
            snippets: "dist/pkg/snippets".to_string(),
        }
    }
}
