#![doc = include_str!("../README.proj.md")]
/*!
## Packages

This is the API documentation for the `perseus-actix-web` package, which allows Perseus apps to run on Actix Web. Note that Perseus mostly uses [the book](https://arctic-hen7.github.io/perseus/en-US) for
documentation, and this should mostly be used as a secondary reference source. You can also find full usage examples [here](https://github.com/arctic-hen7/perseus/tree/main/examples).
*/

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

use actix_files::Files;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use perseus::turbine::ApiResponse as PerseusApiResponse;
use perseus::{
    http::StatusCode,
    i18n::TranslationsManager,
    path::*,
    server::ServerOptions,
    stores::MutableStore,
    turbine::{SubsequentLoadQueryParams, Turbine},
    Request,
};

// ----- Request conversion implementation -----

/// Converts an Actix Web request into an `http::request`.
pub fn convert_req(raw: &actix_web::HttpRequest) -> Result<Request, String> {
    let mut builder = Request::builder();

    for (name, val) in raw.headers() {
        builder = builder.header(name, val);
    }

    builder
        .uri(raw.uri())
        .method(raw.method())
        .version(raw.version())
        // We always use an empty body because, in a Perseus request, only the URI matters
        // Any custom data should therefore be sent in headers (if you're doing that, consider a
        // dedicated API)
        .body(())
        .map_err(|err| err.to_string())
}

// ----- Newtype wrapper for response implementation -----

#[derive(Debug)]
struct ApiResponse(PerseusApiResponse);
impl From<PerseusApiResponse> for ApiResponse {
    fn from(val: PerseusApiResponse) -> Self {
        Self(val)
    }
}
impl Responder for ApiResponse {
    type Body = String;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let mut res = HttpResponse::build(self.0.status);
        for header in self.0.headers {
            // The header name is in an `Option`, but we only ever add them with proper
            // names in `PerseusApiResponse`
            res.insert_header((header.0.unwrap(), header.1));
        }
        // TODO
        res.message_body(self.0.body).unwrap()
    }
}

// ----- Integration code -----

/// Configures an existing Actix Web app for Perseus. This returns a function
/// that does the configuring so it can take arguments. This includes a complete
/// wildcard handler (`*`), and so it should be configured after any other
/// routes on your server.
pub async fn configurer<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    turbine: &'static Turbine<M, T>,
    opts: ServerOptions,
) -> impl FnOnce(&mut actix_web::web::ServiceConfig) {
    move |cfg: &mut web::ServiceConfig| {
        cfg
            // --- File handlers ---
            .service(Files::new("/.perseus/bundle.js", &opts.js_bundle))
            .service(Files::new("/.perseus/bundle.wasm", &opts.wasm_bundle))
            .service(Files::new("/.perseus/bundle.wasm.js", &opts.wasm_js_bundle))
            .service(Files::new("/.perseus/snippets", &opts.snippets))
            // --- Translation and subsequent load handlers
            .route(
                "/.perseus/translations/{locale}",
                web::get().to(move |http_req: HttpRequest| async move {
                    let locale = http_req.match_info().query("locale");
                    ApiResponse(turbine.get_translations(&locale).await)
                }),
            )
            .route(
                "/.perseus/page/{locale}/{filename:.*}.json",
                web::get().to(move |http_req: HttpRequest, web::Query(query_params): web::Query<SubsequentLoadQueryParams>| async move {
                    let raw_path = http_req.match_info().query("filename").to_string();
                    let locale = http_req.match_info().query("locale");
                    let SubsequentLoadQueryParams { entity_name, was_incremental_match } = query_params;
                    let http_req = match convert_req(&http_req) {
                        Ok(req) => req,
                        Err(err) => return ApiResponse(PerseusApiResponse::err(StatusCode::BAD_REQUEST, &err))
                    };

                    ApiResponse(turbine.get_subsequent_load(
                        PathWithoutLocale(raw_path),
                        locale.to_string(),
                        entity_name,
                        was_incremental_match,
                        http_req
                    ).await.into())
                }),
            );
        // --- Static directory and alias handlers
        if turbine.static_dir.exists() {
            cfg.service(Files::new("/.perseus/static", &turbine.static_dir));
        }
        for (url, static_path) in turbine.static_aliases.iter() {
            cfg.service(Files::new(url, static_path));
        }
        // --- Initial load handler ---
        cfg.route(
            "{route:.*}",
            web::get().to(move |http_req: HttpRequest| async move {
                let raw_path = http_req.path().to_string();
                let http_req = match convert_req(&http_req) {
                    Ok(req) => req,
                    Err(err) => {
                        return ApiResponse(PerseusApiResponse::err(StatusCode::BAD_REQUEST, &err))
                    }
                };
                ApiResponse(
                    turbine
                        .get_initial_load(PathMaybeWithLocale(raw_path), http_req)
                        .await,
                )
            }),
        );
    }
}

// // File handlers (these have to be broken out for Actix)
// async fn js_bundle(opts: web::Data<ServerOptions>) ->
// std::io::Result<NamedFile> {     NamedFile::open(&opts.js_bundle)
// }
// async fn wasm_bundle(opts: web::Data<ServerOptions>) ->
// std::io::Result<NamedFile> {     NamedFile::open(&opts.wasm_bundle)
// }
// async fn wasm_js_bundle(opts: web::Data<ServerOptions>) ->
// std::io::Result<NamedFile> {     NamedFile::open(&opts.wasm_js_bundle)
// }
// async fn static_alias<M: MutableStore, T: TranslationsManager>(
//     turbine: &'static Turbine<M, T>,
//     req: HttpRequest,
// ) -> std::io::Result<NamedFile> {
//     let filename = turbine.static_aliases.get(req.path());
//     let filename = match filename {
//         Some(filename) => filename,
//         // If the path doesn't exist, then the alias is not found
//         None => return
// Err(std::io::Error::from(std::io::ErrorKind::NotFound)),     };
//     NamedFile::open(filename)
// }

// ----- Default server -----

/// Creates and starts the default Perseus server using Actix Web. This should
/// be run in a `main()` function annotated with `#[tokio::main]` (which
/// requires the `macros` and `rt-multi-thread` features on the `tokio`
/// dependency).
#[cfg(feature = "dflt-server")]
pub async fn dflt_server<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    turbine: &'static Turbine<M, T>,
    opts: ServerOptions,
    (host, port): (String, u16),
) {
    use actix_web::{App, HttpServer};
    use futures::executor::block_on;
    // TODO Fix issues here
    HttpServer::new(move ||
        App::new()
            .configure(
                block_on(
                    configurer(
                        turbine,
                        opts.clone(),
                    )
                )
            )
    )
        .bind((host, port))
        .expect("Couldn't bind to given address. Maybe something is already running on the selected port?")
        .run()
        .await
        .expect("Server failed.") // TODO Improve error message here
}
