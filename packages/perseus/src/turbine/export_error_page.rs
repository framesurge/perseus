use super::Turbine;
use crate::error_views::ServerErrorData;
use crate::{errors::*, i18n::TranslationsManager, plugins::PluginAction, stores::MutableStore};
use std::fs;
use std::sync::Arc;

impl<M: MutableStore, T: TranslationsManager> Turbine<M, T> {
    /// Exports the error page of the given exit code to the given path.
    pub async fn export_error_page(&self, code: u16, output: &str) -> Result<(), Arc<Error>> {
        self.plugins
            .functional_actions
            .export_error_page_actions
            .before_export_error_page
            .run((code, output.to_string()), self.plugins.get_plugin_data())
            .map_err(|err| Arc::new(err.into()))?;

        // Build that error page as the server does (assuming the app has been
        // built so that the HTML shell is ready)
        let err_page_str = self.build_error_page(
            ServerErrorData {
                status: code,
                // Hopefully, this error will appear in a context that makes sense (e.g. a 404).
                // Exporting a 500 page doesn't make a great deal of sense on most
                // static serving infrastructure (they'll have their own).
                msg: "app was exported, no further details available".to_string(),
            },
            // Localizing exported error pages is not currently supported. However, if a locale is
            // available in the browser, it will be used to override whatever was
            // rendered from this.
            None,
        );

        // Write that to the given output location (this will be relative to wherever
        // the user executed from)
        match fs::write(output, err_page_str) {
            Ok(_) => (),
            Err(err) => {
                let err = EngineError::WriteErrorPageError {
                    source: err,
                    dest: output.to_string(),
                };
                let err: Arc<Error> = Arc::new(err.into());
                self.plugins
                    .functional_actions
                    .export_error_page_actions
                    .after_failed_write
                    .run(err.clone(), self.plugins.get_plugin_data())
                    .map_err(|err| Arc::new(err.into()))?;
                return Err(err);
            }
        };

        self.plugins
            .functional_actions
            .export_error_page_actions
            .after_successful_export_error_page
            .run((), self.plugins.get_plugin_data())
            .map_err(|err| Arc::new(err.into()))?;

        Ok(())
    }
}
