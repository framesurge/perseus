use super::Turbine;
use crate::{errors::ServerError, i18n::TranslationsManager, stores::MutableStore};

/// A template for JS files that define the render configuration and
/// translations. Until the render configuration is defined, Perseus will not be
/// instantiated.
static JS_CONST_FILE_TEMPLATE: &str = r#"window.__PERSEUS_RENDER_CFG = `%render_cfg%`;
window.__PERSEUS_TRANSLATIONS = `%translations%`;"#;

/// A template for JS files that define the render configuration only. This is
/// used for unlocalized pages only.
static JS_UNLOCALIZED_CONST_FILE_TEMPLATE: &str =
    r#"window.__PERSEUS_RENDER_CFG = `%render_cfg%`;"#;

impl<M: MutableStore, T: TranslationsManager> Turbine<M, T> {
    /// Creates a JavaScript file containing the render configuration and
    /// translations data that will also emit a custom event as soon as it
    /// is ready. This is used by Perseus to avoid sending these data (which
    /// are generic, but which can be very large), as part of the initial
    /// HTML bundle. Instead, this file will be requested as part of the bundle,
    /// and it will inform the Wasm bundle, meaning the user can see a page as
    /// quickly as possible.
    ///
    /// This will assume that the given locale is supported.
    pub(crate) async fn initial_consts_js(&self, locale: &str) -> Result<String, ServerError> {
        // If we have no locale, just send the render config (used by unlocalized pages
        // like some errors and the locale redirection pages)
        let js_file = if locale.is_empty() {
            // This is safe to unwrap, we know it will serialize
            let render_cfg = serde_json::to_string(&self.render_cfg).unwrap();
            JS_UNLOCALIZED_CONST_FILE_TEMPLATE.replace("%render_cfg%", &render_cfg)
        } else {
            let translations = self
                .translations_manager
                .get_translations_str_for_locale(locale.to_string())
                .await;
            let translations = match translations {
                Ok(translations) => translations,
                Err(err) => todo!(),
            };
            // This is safe to unwrap, we know it will serialize
            let render_cfg = serde_json::to_string(&self.render_cfg).unwrap();

            JS_CONST_FILE_TEMPLATE
                .replace("%render_cfg%", &render_cfg)
                .replace("%translations%", &translations)
        };

        Ok(js_file)
    }
}
