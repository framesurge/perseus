use std::sync::Arc;
use std::{fs, rc::Rc};
use sycamore::web::SsrNode;
use crate::init::PerseusAppBase;
use crate::error_pages::ErrorPageLocation;
use crate::{errors::*, i18n::TranslationsManager, plugins::PluginAction, stores::MutableStore};
use super::Turbine;
use super::build_error_page::build_error_page;

impl<M: MutableStore, T: TranslationsManager> Turbine<M, T> {
    /// Exports the error page of the given exit code to the given path.
    pub async fn export_error_page(&self, code: u16, output: &str) -> Result<(), Arc<Error>> {
        // We assume the app has already been built before running this
        let html_shell = self.html_shell.as_ref().unwrap();

        self.plugins
            .functional_actions
            .export_error_page_actions
            .before_export_error_page
            .run((code, output.to_string()), self.plugins.get_plugin_data())
            .map_err(|err| Arc::new(err.into()))?;

        // Build that error page as the server does
        let err_page_str = build_error_page(ErrorPageLocation::Current, code, "", None, &self.error_pages, &html_shell);

        // Write that to the given output location (this will be relative to wherever the user executed from)
        match fs::write(&output, err_page_str) {
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
