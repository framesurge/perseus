use crate::conv_req::convert_req;
use crate::Options;
use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse};
use perseus::{
    err_to_status_code,
    serve::get_page_for_template,
    ConfigManager, TranslationsManager,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PageDataReq {
    pub template_name: String,
}

/// The handler for calls to `.perseus/page/*`. This will manage returning errors and the like.
pub async fn page_data<C: ConfigManager, T: TranslationsManager>(
    req: HttpRequest,
    opts: web::Data<Options>,
    config_manager: web::Data<C>,
    translations_manager: web::Data<T>,
    web::Query(query_params): web::Query<PageDataReq>,
) -> HttpResponse {
    let templates = &opts.templates_map;
    let locale = req.match_info().query("locale");
    let template_name = query_params.template_name;
    // Check if the locale is supported
    if opts.locales.is_supported(locale) {
        let path = req.match_info().query("filename");
        // We need to turn the Actix Web request into one acceptable for Perseus (uses `http` internally)
        let http_req = convert_req(&req);
        let http_req = match http_req {
            Ok(http_req) => http_req,
            // If this fails, the client request is malformed, so it's a 400
            Err(err) => {
                return HttpResponse::build(StatusCode::from_u16(400).unwrap())
                    .body(err.to_string())
            }
        };
        // Get the template to use
        let template = templates.get(&template_name);
        let template = match template {
            Some(template) => template,
            None => {
                // We know the template has been pre-routed and should exist, so any failure here is a 500
                return HttpResponse::InternalServerError().body("template not found".to_string());
            }
        };
        let page_data = get_page_for_template(
            path,
            locale,
            template,
            http_req,
            config_manager.get_ref(),
            translations_manager.get_ref()
        )
        .await;
        match page_data {
            Ok(page_data) => HttpResponse::Ok().body(serde_json::to_string(&page_data).unwrap()),
            // We parse the error to return an appropriate status code
            Err(err) => HttpResponse::build(StatusCode::from_u16(err_to_status_code(&err)).unwrap())
                .body(err.to_string())
        }
    } else {
        HttpResponse::NotFound().body("locale not supported".to_string())
    }
}
