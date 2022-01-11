use crate::conv_req::convert_req;
use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse};
use fmterr::fmt_err;
use perseus::{
    errors::err_to_status_code,
    internal::{
        get_path_prefix_server,
        i18n::{TranslationsManager, Translator},
        router::{match_route_atomic, RouteInfoAtomic, RouteVerdictAtomic},
        serve::{
            build_error_page, get_path_slice, render::get_page_for_template, HtmlShell,
            ServerOptions,
        },
    },
    stores::{ImmutableStore, MutableStore},
    ErrorPages, SsrNode,
};
use std::collections::HashMap;
use std::rc::Rc;

/// Builds on the internal Perseus primitives to provide a utility function that returns an `HttpResponse` automatically.
fn return_error_page(
    url: &str,
    status: &u16,
    // This should already have been transformed into a string (with a source chain etc.)
    err: &str,
    translator: Option<Rc<Translator>>,
    error_pages: &ErrorPages<SsrNode>,
    html_shell: &HtmlShell,
    root_id: &str,
) -> HttpResponse {
    let html = build_error_page(
        url,
        status,
        err,
        translator,
        error_pages,
        html_shell,
        root_id,
    );
    HttpResponse::build(StatusCode::from_u16(*status).unwrap())
        .content_type("text/html")
        .body(html)
}

/// The handler for calls to any actual pages (first-time visits), which will render the appropriate HTML and then interpolate it into
/// the app shell.
pub async fn initial_load<M: MutableStore, T: TranslationsManager>(
    req: HttpRequest,
    opts: web::Data<Rc<ServerOptions>>,
    html_shell: web::Data<HtmlShell<'_>>,
    render_cfg: web::Data<HashMap<String, String>>,
    immutable_store: web::Data<ImmutableStore>,
    mutable_store: web::Data<M>,
    translations_manager: web::Data<T>,
) -> HttpResponse {
    let templates = &opts.templates_map;
    let error_pages = &opts.error_pages;
    let path = req.path();
    let path_slice = get_path_slice(path);
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
    let verdict = match_route_atomic(&path_slice, render_cfg.get_ref(), templates, &opts.locales);
    match verdict {
        // If this is the outcome, we know that the locale is supported and the like
        // Given that all this is valid from the client, any errors are 500s
        RouteVerdictAtomic::Found(RouteInfoAtomic {
            path,     // Used for asset fetching, this is what we'd get in `page_data`
            template, // The actual template to use
            locale,
            was_incremental_match,
        }) => {
            // We need to turn the Actix Web request into one acceptable for Perseus (uses `http` internally)
            let http_req = convert_req(&req);
            let http_req = match http_req {
                Ok(http_req) => http_req,
                // If this fails, the client request is malformed, so it's a 400
                Err(err) => {
                    return html_err(400, &fmt_err(&err));
                }
            };
            // Actually render the page as we would if this weren't an initial load
            let page_data = get_page_for_template(
                &path,
                &locale,
                template,
                was_incremental_match,
                http_req,
                (immutable_store.get_ref(), mutable_store.get_ref()),
                translations_manager.get_ref(),
            )
            .await;
            let page_data = match page_data {
                Ok(page_data) => page_data,
                // We parse the error to return an appropriate status code
                Err(err) => {
                    return html_err(err_to_status_code(&err), &fmt_err(&err));
                }
            };

            let final_html = html_shell
                .get_ref()
                .clone()
                .page_data(&page_data, &opts.root_id)
                .to_string();

            let mut http_res = HttpResponse::Ok();
            http_res.content_type("text/html");
            // Generate and add HTTP headers
            for (key, val) in template.get_headers(page_data.state) {
                http_res.insert_header((key.unwrap(), val));
            }

            http_res.body(final_html)
        }
        // For locale detection, we don't know the user's locale, so there's not much we can do except send down the app shell, which will do the rest and fetch from `.perseus/page/...`
        RouteVerdictAtomic::LocaleDetection(path) => {
            // We use a `302 Found` status code to indicate a redirect
            // We 'should' generate a `Location` field for the redirect, but it's not RFC-mandated, so we can use the app shell
            HttpResponse::Found().content_type("text/html").body(
                html_shell
                    .get_ref()
                    .clone()
                    .locale_redirection_fallback(
                        // We'll redirect the user to the default locale
                        &format!(
                            "{}/{}/{}",
                            get_path_prefix_server(),
                            opts.locales.default,
                            path
                        ),
                        &opts.root_id,
                    )
                    .to_string(),
            )
        }
        RouteVerdictAtomic::NotFound => html_err(404, "page not found"),
    }
}
