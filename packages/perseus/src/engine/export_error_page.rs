use crate::{
    errors::EngineError, i18n::TranslationsManager, internal::serve::build_error_page,
    stores::MutableStore, PerseusApp, PerseusAppBase, PluginAction, SsrNode,
};
use std::{fs, rc::Rc};

/// Exports a single error page for the given HTTP status code to the given
/// output location. If the status code doesn't exist or isn't handled, then the
/// fallback page will be exported.
///
/// This expects to run in the root of the project.
///
/// This can only return IO errors from failures to write to the given output
/// location. (Wrapped in an `Rc` so they can be sent to plugins as well.)
pub async fn export_error_page(
    app: PerseusAppBase<SsrNode, impl MutableStore, impl TranslationsManager>,
    code: u16,
    output: &str,
) -> Result<(), Rc<EngineError>> {
    let plugins = app.get_plugins();

    let error_pages = app.get_error_pages();
    // Prepare the HTML shell
    let index_view_str = app.get_index_view_str();
    let root_id = app.get_root();
    let immutable_store = app.get_immutable_store();
    // We assume the app has already been built before running this (so the render
    // config must be available) It doesn't matter if the type parameters here
    // are wrong, this function doesn't use them
    let html_shell =
        PerseusApp::get_html_shell(index_view_str, &root_id, &immutable_store, &plugins).await;

    plugins
        .functional_actions
        .export_error_page_actions
        .before_export_error_page
        .run((code, output.to_string()), plugins.get_plugin_data());

    // Build that error page as the server does
    let err_page_str = build_error_page("", code, "", None, &error_pages, &html_shell);

    // Write that to the given output location
    match fs::write(&output, err_page_str) {
        Ok(_) => (),
        Err(err) => {
            let err = EngineError::WriteErrorPageError {
                source: err,
                dest: output.to_string(),
            };
            let err = Rc::new(err);
            plugins
                .functional_actions
                .export_error_page_actions
                .after_failed_write
                .run(err.clone(), plugins.get_plugin_data());
            return Err(err);
        }
    };

    plugins
        .functional_actions
        .export_error_page_actions
        .after_successful_export_error_page
        .run((), plugins.get_plugin_data());

    Ok(())
}
