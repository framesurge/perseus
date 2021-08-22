use perseus::config_manager::FsConfigManager;
use perseus_actix_web::{configurer, Options};
use perseus_showcase_app::pages;
use sycamore::SsrNode;
use actix_web::{HttpServer, App};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
        App::new()
        	.configure(
				configurer(
					Options {
						index: "../app/index.html".to_string(),
						js_bundle: "../app/pkg/bundle.js".to_string(),
						wasm_bundle: "../app/pkg/perseus_showcase_app_bg.wasm".to_string(),
						templates_map: pages::get_templates_map::<SsrNode>()
					},
					FsConfigManager::new()
				)
			)
    })
    	.bind(("localhost", 8080))?
    	.run()
    	.await
}