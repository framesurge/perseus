use super::Turbine;
use crate::error_views::ServerErrorData;
use crate::i18n::TranslationsManager;
use crate::stores::MutableStore;
use crate::translator::Translator;

impl<M: MutableStore, T: TranslationsManager> Turbine<M, T> {
    /// Prepares an HTML error page for the client, with injected markers for
    /// hydration. In the event of an error, this should be returned to the
    /// client (with the appropriate status code) to allow Perseus to
    /// hydrate and display the correct error page. Note that this is only
    /// for use in initial loads (other systems handle errors in subsequent
    /// loads, and the app shell exists then so the server doesn't have to
    /// do nearly as much work).
    ///
    /// If a translator and translations string is provided, it will be assumed
    /// to be of the correct locale, and will be injected into the page. A
    /// best effort should be made to provide translations here.
    ///
    /// # Pitfalls
    /// If a translations string is provided here that does not match with the
    /// locale actually being returned (i.e. that which the client will
    /// infer), there will be a mismatch between the translations string and
    /// the locale, which can only be rectified by the user manually
    /// switching to another locale and back again. Please ensure the
    /// correct translations string is provided here!
    pub(crate) fn build_error_page(
        &self,
        data: ServerErrorData,
        translator: Option<&Translator>,
    ) -> String {
        let (translator, locale) = if let Some(translator) = translator {
            (Some(translator), Some(translator.get_locale()))
        } else {
            (None, None)
        };

        let (head, body) = self.error_views.render_to_string(data.clone(), translator);

        self.html_shell
            .as_ref()
            .unwrap()
            .clone()
            // This will inject the translations string if it's available
            .error_page(&data, &body, &head, locale)
            .to_string()
    }
}
