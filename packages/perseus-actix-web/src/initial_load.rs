use crate::conv_req::convert_req;
use crate::Options;
use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse};
use perseus::error_pages::ErrorPageData;
use perseus::router::{match_route, RouteInfo, RouteVerdict};
use perseus::html_shell::interpolate_page_data;
use perseus::{
    err_to_status_code, serve::get_page_for_template, ConfigManager, ErrorPages,
    SsrNode, TranslationsManager, Translator,
};
use std::collections::HashMap;
use std::rc::Rc;

/// Returns a fully formed error page to the client, with parameters for hydration.
fn return_error_page(
    url: &str,
    status: &u16,
    err: &str,
    translator: Option<Rc<Translator>>,
    error_pages: &ErrorPages<SsrNode>,
    html: &str,
    root_id: &str,
) -> HttpResponse {
    let error_html = error_pages.render_to_string(url, status, err, translator);
    // We create a JSON representation of the data necessary to hydrate the error page on the client-side
    // Right now, translators are never included in transmitted error pages
    let error_page_data = serde_json::to_string(&ErrorPageData {
        url: url.to_string(),
        status: *status,
        err: err.to_string(),
    })
    .unwrap();
    // Add a global variable that defines this as an error
    let state_var = format!(
        "<script>window.__PERSEUS_INITIAL_STATE = 'error-{}';</script>",
        error_page_data.replace(r#"'"#, r#"\'"#) // If we don't escape single quotes, we get runtime syntax errors
    );
    let html_with_declaration = html.replace("</head>", &format!("{}\n</head>", state_var));
    // Interpolate the error page itself
    let html_to_replace_double = format!("<div id=\"{}\">", root_id);
    let html_to_replace_single = format!("<div id='{}'>", root_id);
    let html_replacement = format!(
        // We give the content a specific ID so that it can be hydrated properly
        "{}<div id=\"__perseus_content\">{}</div>",
        &html_to_replace_double,
        &error_html
    );
    // Now interpolate that HTML into the HTML shell
    let final_html = html_with_declaration
        .replace(&html_to_replace_double, &html_replacement)
        .replace(&html_to_replace_single, &html_replacement);

    HttpResponse::build(StatusCode::from_u16(*status).unwrap())
        .content_type("text/html")
        .body(final_html)
}

/// The handler for calls to any actual pages (first-time visits), which will render the appropriate HTML and then interpolate it into
/// the app shell.
pub async fn initial_load<C: ConfigManager, T: TranslationsManager>(
    req: HttpRequest,
    opts: web::Data<Options>,
    html_shell: web::Data<String>,
    render_cfg: web::Data<HashMap<String, String>>,
    config_manager: web::Data<C>,
    translations_manager: web::Data<T>,
) -> HttpResponse {
    let templates = &opts.templates_map;
    let error_pages = &opts.error_pages;
    let path = req.path();
    let path_slice: Vec<&str> = path
        .split('/')
        // Removing empty elements is particularly important, because the path will have a leading `/`
        .filter(|p| !p.is_empty())
        .collect();
    // Create a closure to make returning error pages easier (most have the same data)
    let html_err = |status: u16, err: &str| {
        return return_error_page(
            path,
            &status,
            err,
            None,
            error_pages,
            html_shell.get_ref(),
            &opts.root_id,
        );
    };

    // Run the routing algorithms on the path to figure out which template we need
    let verdict = match_route(&path_slice, render_cfg.get_ref(), templates, &opts.locales);
    match verdict {
        // If this is the outcome, we know that the locale is supported and the like
        // Given that all this is valid from the client, any errors are 500s
        RouteVerdict::Found(RouteInfo {
            path,     // Used for asset fetching, this is what we'd get in `page_data`
            template, // The actual template to use
            locale,
        }) => {
            // We need to turn the Actix Web request into one acceptable for Perseus (uses `http` internally)
            let http_req = convert_req(&req);
            let http_req = match http_req {
                Ok(http_req) => http_req,
                // If this fails, the client request is malformed, so it's a 400
                Err(err) => {
                    return html_err(400, &err.to_string());
                }
            };
            // Actually render the page as we would if this weren't an initial load
            let page_data = get_page_for_template(
                &path,
                &locale,
                &template,
                http_req,
                config_manager.get_ref(),
                translations_manager.get_ref()
            )
            .await;
            let page_data = match page_data {
                Ok(page_data) => page_data,
                // We parse the error to return an appropriate status code
                Err(err) => {
                    return html_err(err_to_status_code(&err), &err.to_string());
                }
            };

            let final_html = interpolate_page_data(&html_shell, page_data, &opts.root_id);

            HttpResponse::Ok()
                .content_type("text/html")
                .body(final_html)
        }
        // For locale detection, we don't know the user's locale, so there's not much we can do except send down the app shell, which will do the rest and fetch from `.perseus/page/...`
        RouteVerdict::LocaleDetection(_) => {
            // We use a `302 Found` status code to indicate a redirect
            // We 'should' generate a `Location` field for the redirect, but it's not RFC-mandated, so we can use the app shell
            HttpResponse::Found()
                .content_type("text/html")
                .body(html_shell.get_ref())
        }
        RouteVerdict::NotFound => html_err(404, "page not found"),
    }
}
