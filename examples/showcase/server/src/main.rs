use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse, http::StatusCode};
use actix_files::{NamedFile};
use sycamore::prelude::SsrNode;
use std::collections::HashMap;

use perseus::{
    serve::{get_render_cfg, get_page},
    config_manager::FsConfigManager,
    template::TemplateMap,
    errors::ErrorKind as PerseusErr,
    errors::err_to_status_code
};
use perseus_showcase_app::pages;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(
                get_render_cfg().expect("Couldn't get render configuration!")
            )
            .data(
                FsConfigManager::new()
            )
            .data(
                pages::get_templates_map::<SsrNode>()
            )
            // TODO chunk JS and WASM bundles
            // These allow getting the basic app code (not including the static data)
            // This contains everything in the spirit of a pseudo-SPA
            .route("/.perseus/bundle.js", web::get().to(js_bundle))
            .route("/.perseus/bundle.wasm", web::get().to(wasm_bundle))
            // This allows getting the static HTML/JSON of a page
            // We stream both together in a single JSON object so SSR works (otherwise we'd have request IDs and weird caching...)
            .route("/.perseus/page/{filename:.*}", web::get().to(page_data))
            // For everything else, we'll serve the app shell directly
            .default_service(web::route().to(initial))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}

async fn initial() -> std::io::Result<NamedFile> {
    NamedFile::open("../app/index.html")
}
async fn js_bundle() -> std::io::Result<NamedFile> {
    NamedFile::open("../app/pkg/bundle.js")
}
async fn wasm_bundle() -> std::io::Result<NamedFile> {
    NamedFile::open("../app/pkg/perseus_showcase_app_bg.wasm")
}
async fn page_data(
    req: HttpRequest,
    templates: web::Data<TemplateMap<SsrNode>>,
    render_cfg: web::Data<HashMap<String, String>>,
    config_manager: web::Data<FsConfigManager>
) -> HttpResponse {
    let path = req.match_info().query("filename");
    let page_data = get_page(path, &render_cfg, &templates, config_manager.get_ref()).await;
    let http_res = match page_data {
        Ok(page_data) => HttpResponse::Ok().body(
            serde_json::to_string(&page_data).unwrap()
        ),
        Err(err) => HttpResponse::build(StatusCode::from_u16(err_to_status_code(&err)).unwrap()).body(err.to_string()),
    };

    http_res
}
