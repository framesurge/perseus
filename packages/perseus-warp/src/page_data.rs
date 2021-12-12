use fmterr::fmt_err;
use perseus::{
    errors::err_to_status_code,
    internal::{
        i18n::TranslationsManager,
        serve::{render::get_page_for_template, ServerOptions},
    },
    stores::{ImmutableStore, MutableStore},
};
use serde::Deserialize;
use std::sync::Arc;
use warp::http::Response;
use warp::path::Tail;

// Note: this is the same as for the Actix Web integration, but other frameworks may handle parsing query parameters differntly, so this shouldn't be integrated into the core library
#[derive(Deserialize)]
pub struct PageDataReq {
    pub template_name: String,
    pub was_incremental_match: bool,
}

#[allow(clippy::too_many_arguments)] // Because of how Warp filters work, we don't exactly have a choice
pub async fn page_handler<M: MutableStore, T: TranslationsManager>(
    locale: String,
    path: Tail, // This is the path after the locale that was sent
    PageDataReq {
        template_name,
        was_incremental_match,
    }: PageDataReq,
    http_req: perseus::http::Request<()>,
    opts: Arc<ServerOptions>,
    immutable_store: Arc<ImmutableStore>,
    mutable_store: Arc<M>,
    translations_manager: Arc<T>,
) -> Response<String> {
    let templates = &opts.templates_map;
    // Check if the locale is supported
    if opts.locales.is_supported(&locale) {
        // Warp doesn't let us specify that all paths should end in `.json`, so we'll manually strip that
        let path = path.as_str().strip_suffix(".json").unwrap();
        // Get the template to use
        let template = templates.get(&template_name);
        let template = match template {
            Some(template) => template,
            None => {
                // We know the template has been pre-routed and should exist, so any failure here is a 500
                return Response::builder()
                    .status(500)
                    .body("template not found".to_string())
                    .unwrap();
            }
        };
        let page_data = get_page_for_template(
            path,
            &locale,
            template,
            was_incremental_match,
            http_req,
            (immutable_store.as_ref(), mutable_store.as_ref()),
            translations_manager.as_ref(),
        )
        .await;
        match page_data {
            Ok(page_data) => {
                let mut http_res = Response::builder().status(200);
                // http_res.content_type("text/html");
                // Generate and add HTTP headers
                for (key, val) in template.get_headers(page_data.state.clone()) {
                    http_res = http_res.header(key.unwrap(), val);
                }

                let page_data_str = serde_json::to_string(&page_data).unwrap();
                http_res.body(page_data_str).unwrap()
            }
            // We parse the error to return an appropriate status code
            Err(err) => Response::builder()
                .status(err_to_status_code(&err))
                .body(fmt_err(&err))
                .unwrap(),
        }
    } else {
        Response::builder()
            .status(404)
            .body("locale not supported".to_string())
            .unwrap()
    }
}
