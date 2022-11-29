use std::sync::Arc;

use crate::conv_req::convert_req;
use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse};
use fmterr::fmt_err;
use perseus::{
    errors::err_to_status_code,
    i18n::TranslationsManager,
    internal::PageDataPartial,
    server::{get_page_for_template, GetPageProps, ServerOptions},
    state::GlobalStateCreator,
    stores::{ImmutableStore, MutableStore},
    template::TemplateState,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PageDataReq {
    pub template_name: String,
    pub was_incremental_match: bool,
}

/// The handler for calls to `.perseus/page/*`. This will manage returning
/// errors and the like.
#[allow(clippy::too_many_arguments)]
pub async fn page_data<M: MutableStore, T: TranslationsManager>(
    req: HttpRequest,
    opts: web::Data<ServerOptions>,
    immutable_store: web::Data<ImmutableStore>,
    mutable_store: web::Data<M>,
    translations_manager: web::Data<T>,
    global_state: web::Data<TemplateState>,
    gsc: web::Data<Arc<GlobalStateCreator>>,
    web::Query(query_params): web::Query<PageDataReq>,
) -> HttpResponse {
    let templates = &opts.templates_map;
    let locale = req.match_info().query("locale");
    let PageDataReq {
        template_name,
        was_incremental_match,
    } = query_params;
    // Check if the locale is supported
    if opts.locales.is_supported(locale) {
        let path = req.match_info().query("filename");
        // We need to turn the Actix Web request into one acceptable for Perseus (uses
        // `http` internally)
        let http_req = convert_req(&req);
        let http_req = match http_req {
            Ok(http_req) => http_req,
            // If this fails, the client request is malformed, so it's a 400
            Err(err) => {
                return HttpResponse::build(StatusCode::from_u16(400).unwrap()).body(fmt_err(&err))
            }
        };
        // Get the template to use
        let template = templates.get(&template_name);
        let template = match template {
            Some(template) => template,
            None => {
                // We know the template has been pre-routed and should exist, so any failure
                // here is a 500
                return HttpResponse::InternalServerError().body("template not found".to_string());
            }
        };
        let page_data = get_page_for_template(
            GetPageProps {
                raw_path: path,
                locale,
                was_incremental_match,
                req: http_req,
                global_state: &global_state,
                immutable_store: immutable_store.get_ref(),
                mutable_store: mutable_store.get_ref(),
                translations_manager: translations_manager.get_ref(),
                global_state_creator: gsc.get_ref(),
            },
            template,
            false, // For subsequent loads, we don't want to render content (the client can do it)
        )
        .await;
        match page_data {
            Ok((page_data, _)) => {
                let partial_page_data = PageDataPartial {
                    state: page_data.state.clone(),
                    head: page_data.head,
                };
                let mut http_res = HttpResponse::Ok();
                http_res.content_type("text/html");
                // Generate and add HTTP headers
                for (key, val) in template.get_headers(TemplateState::from_value(page_data.state)) {
                    http_res.insert_header((key.unwrap(), val));
                }

                http_res.body(serde_json::to_string(&partial_page_data).unwrap())
            }
            // We parse the error to return an appropriate status code
            Err(err) => {
                HttpResponse::build(StatusCode::from_u16(err_to_status_code(&err)).unwrap())
                    .body(fmt_err(&err))
            }
        }
    } else {
        HttpResponse::NotFound().body("locale not supported".to_string())
    }
}
