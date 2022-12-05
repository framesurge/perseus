use crate::server::HtmlShell;
use crate::error_pages::{ErrorPageData, ErrorPages};
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
/// This doesn't inject translations of any sort, deliberately, since
/// we can't ensure that they would even exist --- this is used for all
/// types of server-side errors.
///
/// Note that this is only ever used for pages, never widgets.
pub fn build_error_page(
    url: &str,
    status: u16,
    // This should already have been transformed into a string (with a source chain etc.)
    err: &str,
    translator: Option<Rc<Translator>>,
    error_pages: &ErrorPages<SsrNode>,
    html_shell: &HtmlShell,
) -> String {
    let error_html = error_pages.render_to_string(url, status, err, translator.clone());
    let error_head = error_pages.render_head(url, status, err, translator);
    // We create a JSON representation of the data necessary to hydrate the error
    // page on the client-side Right now, translators are never included in
    // transmitted error pages
    let error_page_data = ErrorPageData {
        url: url.to_string(),
        status,
        err: err.to_string(),
    };

    html_shell
        .clone()
        .error_page(&error_page_data, &error_html, &error_head)
        .to_string()
}
