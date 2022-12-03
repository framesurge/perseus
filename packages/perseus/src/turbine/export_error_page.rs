use std::{fs, rc::Rc};
use crate::{PerseusApp, errors::*, i18n::TranslationsManager, plugins::PluginAction, stores::MutableStore};
use super::Turbine;
use super::build_error_page::build_error_page;

impl<M: MutableStore, T: TranslationsManager> Turbine<M, T> {
    /// Exports the error page of the given exit code to the given path.
    pub async fn export_error_page(&self, code: u16, output: &str) -> Result<(), Rc<Error>> {
        // We assume the app has already been built before running this (so the render
        // config must be available). It doesn't matter if the type parameters here
        // are wrong, this function doesn't use them
        let html_shell =
            PerseusApp::get_html_shell(self.index_view_str.to_string(), &self.root_id, &self.render_cfg, &self.immutable_store, &self.plugins)
            .await
            .map_err(|err| Rc::new(err.into()))?;

        self.plugins
            .functional_actions
            .export_error_page_actions
            .before_export_error_page
            .run((code, output.to_string()), self.plugins.get_plugin_data())
            .map_err(|err| Rc::new(err.into()))?;

        // Build that error page as the server does
        let err_page_str = build_error_page("", code, "", None, &self.error_pages, &html_shell);

        // Write that to the given output location (this will be relative to wherever the user executed from)
        match fs::write(&output, err_page_str) {
            Ok(_) => (),
            Err(err) => {
                let err = EngineError::WriteErrorPageError {
                    source: err,
                    dest: output.to_string(),
                };
                let err: Rc<Error> = Rc::new(err.into());
                self.plugins
                    .functional_actions
                    .export_error_page_actions
                    .after_failed_write
                    .run(err.clone(), self.plugins.get_plugin_data())
                    .map_err(|err| Rc::new(err.into()))?;
                return Err(err);
            }
        };

        self.plugins
            .functional_actions
            .export_error_page_actions
            .after_successful_export_error_page
            .run((), self.plugins.get_plugin_data())
            .map_err(|err| Rc::new(err.into()))?;

        Ok(())
    }
}
