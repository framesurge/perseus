use actix_files::NamedFile;
use actix_web::web;
use perseus::{ConfigManager, get_render_cfg, TemplateMap, SsrNode};
use crate::page_data::page_data;

/// The options for setting up the Actix Web integration. This should be literally constructed, as nothing is optional.
#[derive(Clone)]
pub struct Options {
	/// The location on the filesystem of your JavaScript bundle.
	pub js_bundle: String,
	/// The location on the filesystem of your WASM bundle.
	pub wasm_bundle: String,
	/// The location on the filesystem of your `index.html` file that includes the JS bundle.
	pub index: String,
	/// A `HashMap` of your app's templates by their paths.
	pub templates_map: TemplateMap<SsrNode>
}

async fn js_bundle(opts: web::Data<Options>) -> std::io::Result<NamedFile> {
	NamedFile::open(&opts.js_bundle)
}
async fn wasm_bundle(opts: web::Data<Options>) -> std::io::Result<NamedFile> {
	NamedFile::open(&opts.wasm_bundle)
}
async fn index(opts: web::Data<Options>) -> std::io::Result<NamedFile> {
	NamedFile::open(&opts.index)
}

/// Configures an existing Actix Web app for Perseus. This returns a function that does the configuring so it can take arguments.
pub fn configurer<C: ConfigManager + 'static>(opts: Options, config_manager: C) -> impl Fn(&mut web::ServiceConfig) {
	move |cfg: &mut web::ServiceConfig| {
		cfg
			.data(get_render_cfg(&config_manager).expect("Couldn't get render configuration!"))
    	    .data(config_manager.clone())
    	    .data(opts.clone())
    	    // TODO chunk JS and WASM bundles
    	    // These allow getting the basic app code (not including the static data)
    	    // This contains everything in the spirit of a pseudo-SPA
    	    .route("/.perseus/bundle.js", web::get().to(js_bundle))
    	    .route("/.perseus/bundle.wasm", web::get().to(wasm_bundle))
    	    // This allows getting the static HTML/JSON of a page
    	    // We stream both together in a single JSON object so SSR works (otherwise we'd have request IDs and weird caching...)
    	    .route("/.perseus/page/{filename:.*}", web::get().to(page_data::<C>))
    	    // For everything else, we'll serve the app shell directly
			// FIXME
			.route("*", web::get().to(index));
	}
}