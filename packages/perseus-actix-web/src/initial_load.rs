use crate::conv_req::convert_req;
use crate::Options;
use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse};
use perseus::error_pages::ErrorPageData;
use perseus::router::{match_route, RouteInfo, RouteVerdict};
use perseus::{
    err_to_status_code, serve::get_page_for_template_and_translator, ConfigManager, ErrorPages,
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
            // Create a translator here, we'll use it twice
            let translator_raw = translations_manager
                .get_translator_for_locale(locale.to_string())
                .await;
            let translator_raw = match translator_raw {
                Ok(translator_raw) => translator_raw,
                Err(err) => {
                    return html_err(500, &err.to_string());
                }
            };
            let translator = Rc::new(translator_raw);
            // Actually render the page as we would if this weren't an initial load
            let page_data = get_page_for_template_and_translator(
                &path,
                &locale,
                &template,
                http_req,
                Rc::clone(&translator),
                config_manager.get_ref(),
            )
            .await;
            let page_data = match page_data {
                Ok(page_data) => page_data,
                // We parse the error to return an appropriate status code
                Err(err) => {
                    return html_err(err_to_status_code(&err), &err.to_string());
                }
            };

            // Render the HTML head and interpolate it
            let head_str =
                template.render_head_str(page_data.state.clone(), Rc::clone(&translator));
            let html_with_head = html_shell.replace(
                "<!--PERSEUS_INTERPOLATED_HEAD_BEGINS-->",
                &format!("<!--PERSEUS_INTERPOLATED_HEAD_BEGINS-->{}", head_str),
            );

            // Interpolate a global variable of the state so the app shell doesn't have to make any more trips
            // The app shell will unset this after usage so it doesn't contaminate later non-initial loads
            // Error pages (above) will set this to `error`
            let state_var = format!("<script>window.__PERSEUS_INITIAL_STATE = '{}';</script>", {
                if let Some(state) = &page_data.state {
                    state
                        // If we don't escape quotes, we get runtime syntax errors
                        .replace(r#"'"#, r#"\'"#)
                        .replace(r#"""#, r#"\""#)
                } else {
                    "None".to_string()
                }
            });
            // We put this at the very end of the head (after the delimiter comment) because it doesn't matter if it's expunged on subsequent loads
            let html_with_state =
                html_with_head.replace("</head>", &format!("{}\n</head>", state_var));

            // Figure out exactly what we're interpolating in terms of content
            // The user MUST place have a `<div>` of this exact form (documented explicitly)
            // We permit either double or single quotes
            let html_to_replace_double = format!("<div id=\"{}\">", &opts.root_id);
            let html_to_replace_single = format!("<div id='{}'>", &opts.root_id);
            let html_replacement = format!(
                // We give the content a specific ID so that it can be deleted if an error page needs to be rendered on the client-side
                "{}<div id=\"__perseus_content\">{}</div>",
                &html_to_replace_double,
                &page_data.content
            );
            // Now interpolate that HTML into the HTML shell
            let final_html = html_with_state
                .replace(&html_to_replace_double, &html_replacement)
                .replace(&html_to_replace_single, &html_replacement);

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
