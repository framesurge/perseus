use axum::{
    body::Body,
    extract::{Path, Query},
    http::{HeaderMap, StatusCode},
    Extension,
};
use fmterr::fmt_err;
use perseus::{
    errors::err_to_status_code,
    internal::{
        i18n::TranslationsManager,
        serve::{get_page_for_template, GetPageProps, ServerOptions},
    },
    stores::{ImmutableStore, MutableStore},
    Request,
};
use serde::Deserialize;
use std::sync::Arc;

// Note: this is the same as for the Actix Web integration, but other frameworks may handle parsing query parameters differntly, so this shouldn't be integrated into the core library
#[derive(Deserialize)]
pub struct PageDataReq {
    pub template_name: String,
    pub was_incremental_match: bool,
}

#[allow(clippy::too_many_arguments)] // Because of how Axum extractors work, we don't exactly have a choice
pub async fn page_handler<M: MutableStore, T: TranslationsManager>(
    Path(path_parts): Path<Vec<(String, String)>>, // From this, we can extract the locale and the path tail (the page path, which *does* have slashes)
    Query(PageDataReq {
        template_name,
        was_incremental_match,
    }): Query<PageDataReq>,
    // This works without any conversion because Axum allows us to directly get an `http::Request` out!
    // TODO Make sure the type parameter here works
    http_req: perseus::http::Request<Body>,
    Extension(opts): Extension<Arc<ServerOptions>>,
    Extension(immutable_store): Extension<Arc<ImmutableStore>>,
    Extension(mutable_store): Extension<Arc<M>>,
    Extension(translations_manager): Extension<Arc<T>>,
    Extension(global_state): Extension<Arc<Option<String>>>,
) -> (StatusCode, HeaderMap, String) {
    // Separate the locale from the rest of the page name (and we only care about the values, not the names Axum assigns)
    let locale = &path_parts[0].1;
    let path = path_parts[1..]
        .iter()
        .map(|x| x.1.as_str())
        .collect::<Vec<&str>>()
        .join("/");

    let templates = &opts.templates_map;
    // Check if the locale is supported
    if opts.locales.is_supported(locale) {
        // Warp doesn't let us specify that all paths should end in `.json`, so we'll manually strip that
        let path = path.strip_suffix(".json").unwrap();
        // Get the template to use
        let template = templates.get(&template_name);
        let template = match template {
            Some(template) => template,
            None => {
                // We know the template has been pre-routed and should exist, so any failure here is a 500
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    HeaderMap::new(),
                    "template not found".to_string(),
                );
            }
        };
        // Convert the request into one palatable for Perseus (which doesn't have the body attached)
        let http_req = Request::from_parts(http_req.into_parts().0, ());
        let page_data = get_page_for_template(
            GetPageProps::<M, T> {
                raw_path: path,
                locale,
                was_incremental_match,
                req: http_req,
                global_state: &global_state,
                immutable_store: &immutable_store,
                mutable_store: &mutable_store,
                translations_manager: &translations_manager,
            },
            template,
        )
        .await;
        match page_data {
            Ok(page_data) => {
                // http_res.content_type("text/html");
                // Generate and add HTTP headers
                let mut header_map = HeaderMap::new();
                for (key, val) in template.get_headers(page_data.state.clone()) {
                    header_map.insert(key.unwrap(), val);
                }

                let page_data_str = serde_json::to_string(&page_data).unwrap();

                (StatusCode::OK, header_map, page_data_str)
            }
            // We parse the error to return an appropriate status code
            Err(err) => (
                StatusCode::from_u16(err_to_status_code(&err)).unwrap(),
                HeaderMap::new(),
                fmt_err(&err),
            ),
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            HeaderMap::new(),
            "locale not supported".to_string(),
        )
    }
}
