use actix_web::{App, HttpServer};
use futures::executor::block_on;
use perseus::{FsConfigManager, SsrNode};
use perseus_actix_web::{configurer, Options};
use perseus_showcase_app::pages;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(block_on(configurer(
            Options {
                index: "../app/index.html".to_string(),
                js_bundle: "../app/pkg/bundle.js".to_string(),
                wasm_bundle: "../app/pkg/perseus_showcase_app_bg.wasm".to_string(),
                templates_map: pages::get_templates_map::<SsrNode>(),
            },
            FsConfigManager::new("../app/dist".to_string()),
        )))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
