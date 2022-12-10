use crate::error_views::{ErrorViews, ServerErrorData};
use crate::server::HtmlShell;
use crate::error_pages::{ErrorPageData, ErrorPageLocation, ErrorPages};
use crate::translator::Translator;
use crate::SsrNode;
use std::rc::Rc;

/// Prepares an HTML error page for the client, with injected markers for
/// hydration. In the event of an error, this should be returned to the client
/// (with the appropriate status code) to allow Perseus to hydrate and display
/// the correct error page. Note that this is only for use in initial loads
/// (other systems handle errors in subsequent loads, and the app shell
/// exists then so the server doesn't have to do nearly as much work).
///
/// If a translator is provided, this will inject the translations for that
/// locale. If not, then the default translations will be used. This is done
/// because the translator active here will always match with the URL the client
/// requested (or be default if there's no locale attached).
pub fn build_error_page(
    data: ServerErrorData,
    translator: Option<&Translator>,
    error_views: &ErrorViews<SsrNode>,
    html_shell: &HtmlShell,
) -> String {
    // TODO!

    let error_html = error_views.render_to_string(data,, translator.clone());
    let error_head = error_pages.render_head(location.clone(), status, err, translator);
    // We create a JSON representation of the data necessary to hydrate the error
    // page on the client-side Right now, translators are never included in
    // transmitted error pages
    let error_page_data = ErrorPageData {
        location,
        status,
        err: err.to_string(),
    };

    html_shell
        .clone()
        .error_page(&error_page_data, &error_html, &error_head)
        .to_string()
}
