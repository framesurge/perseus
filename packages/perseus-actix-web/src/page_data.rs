use crate::conv_req::convert_req;
use crate::Options;
use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse};
use perseus::{err_to_status_code, get_page, ConfigManager};
use std::collections::HashMap;

/// The handler for calls to `.perseus/page/*`. This will manage returning errors and the like.
pub async fn page_data<C: ConfigManager>(
    req: HttpRequest,
    opts: web::Data<Options>,
    render_cfg: web::Data<HashMap<String, String>>,
    config_manager: web::Data<C>,
) -> HttpResponse {
    let templates = &opts.templates_map;
    let path = req.match_info().query("filename");
    // We need to turn the Actix Web request into one acceptable for Perseus (uses `http` internally)
    let http_req = convert_req(&req);
    let http_req = match http_req {
        Ok(http_req) => http_req,
        // If this fails, the client request is malformed, so it's a 400
        Err(err) => {
            return HttpResponse::build(StatusCode::from_u16(400).unwrap()).body(err.to_string())
        }
    };
    let page_data = get_page(
        path,
        http_req,
        &render_cfg,
        templates,
        config_manager.get_ref(),
    )
    .await;

    match page_data {
        Ok(page_data) => HttpResponse::Ok().body(serde_json::to_string(&page_data).unwrap()),
        // We parse the error to return an appropriate status code
        Err(err) => HttpResponse::build(StatusCode::from_u16(err_to_status_code(&err)).unwrap())
            .body(err.to_string()),
    }
}
