use crate::error_pages::{ErrorPageData, ErrorPages};
use crate::translator::Translator;
use crate::SsrNode;
use std::rc::Rc;

/// Prepares an HTMl error page for the client, with injected markers for hydration. In the event of an error, this should be returned to the client (with the appropriate status code) to allow Perseus
/// to hydrate and display the correct error page. Note that this is only for use in initial loads (other systems handle errors in subsequent loads, and the app shell
/// exists then so the server doesn't have to do nearly as much work).
pub fn build_error_page(
    url: &str,
    status: u16,
    // This should already have been transformed into a string (with a source chain etc.)
    err: &str,
    translator: Option<Rc<Translator>>,
    error_pages: &ErrorPages<SsrNode>,
    html: &str,
    root_id: &str,
) -> String {
    let error_html = error_pages.render_to_string(url, status, err, translator);
    // We create a JSON representation of the data necessary to hydrate the error page on the client-side
    // Right now, translators are never included in transmitted error pages
    let error_page_data = serde_json::to_string(&ErrorPageData {
        url: url.to_string(),
        status,
        err: err.to_string(),
    })
    .unwrap();
    // Add a global variable that defines this as an error
    let state_var = format!(
        "<script>window.__PERSEUS_INITIAL_STATE = `error-{}`;</script>",
        error_page_data
            // We escape any backslashes to prevent their interfering with JSON delimiters
            .replace(r#"\"#, r#"\\"#)
            // We escape any backticks, which would interfere with JS's raw strings system
            .replace(r#"`"#, r#"\`"#)
            // We escape any interpolations into JS's raw string system
            .replace(r#"${"#, r#"\${"#)
    );
    let html_with_declaration = html.replace("</head>", &format!("{}\n</head>", state_var));
    // Interpolate the error page itself
    let html_to_replace_double = format!("<div id=\"{}\">", root_id);
    let html_to_replace_single = format!("<div id='{}'>", root_id);
    let html_replacement = format!(
        // We give the content a specific ID so that it can be hydrated properly
        "{}<div id=\"__perseus_content_initial\" class=\"__perseus_content\">{}</div>",
        &html_to_replace_double,
        &error_html
    );
    // Now interpolate that HTML into the HTML shell
    html_with_declaration
        .replace(&html_to_replace_double, &html_replacement)
        .replace(&html_to_replace_single, &html_replacement)
}
