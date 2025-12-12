#![doc = include_str!("../README.proj.md")]
/*!
## Packages

This is the API documentation for the `perseus-actix-web` package, which allows Perseus apps to run on Actix Web. Note that Perseus mostly uses [the book](https://framesurge.sh/perseus/en-US) for
documentation, and this should mostly be used as a secondary reference source. You can also find full usage examples [here](https://github.com/arctic-hen7/framesurge/tree/main/examples).
 */

#![cfg(engine)] // This crate needs to be run with the Perseus CLI
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

use actix_files::{Files, NamedFile};
use actix_web::CustomizeResponder;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use perseus::turbine::ApiResponse as PerseusApiResponse;
use perseus::{
    http::{self, StatusCode},
    i18n::TranslationsManager,
    path::*,
    server::ServerOptions,
    stores::MutableStore,
    turbine::{SubsequentLoadQueryParams, Turbine},
    Request,
};

// ----- HTTP version conversion helpers -----
// Actix-web uses http 0.2.x while Perseus uses http 1.x, so we need conversion functions

/// Convert actix-web's http 0.2.x Method to perseus's http 1.x Method
fn convert_method(actix_method: &actix_web::http::Method) -> http::Method {
    match actix_method.as_str() {
        "GET" => http::Method::GET,
        "POST" => http::Method::POST,
        "PUT" => http::Method::PUT,
        "DELETE" => http::Method::DELETE,
        "HEAD" => http::Method::HEAD,
        "OPTIONS" => http::Method::OPTIONS,
        "CONNECT" => http::Method::CONNECT,
        "PATCH" => http::Method::PATCH,
        "TRACE" => http::Method::TRACE,
        _ => http::Method::GET, // Fallback
    }
}

/// Convert actix-web's http 0.2.x Uri to perseus's http 1.x Uri
fn convert_uri(actix_uri: &actix_web::http::Uri) -> Result<http::Uri, String> {
    actix_uri
        .to_string()
        .parse()
        .map_err(|e| format!("Failed to convert URI: {}", e))
}

/// Convert actix-web's http 0.2.x Version to perseus's http 1.x Version
fn convert_version(actix_version: actix_web::http::Version) -> http::Version {
    match actix_version {
        actix_web::http::Version::HTTP_09 => http::Version::HTTP_09,
        actix_web::http::Version::HTTP_10 => http::Version::HTTP_10,
        actix_web::http::Version::HTTP_11 => http::Version::HTTP_11,
        actix_web::http::Version::HTTP_2 => http::Version::HTTP_2,
        actix_web::http::Version::HTTP_3 => http::Version::HTTP_3,
        _ => http::Version::HTTP_11, // Fallback
    }
}

/// Convert perseus's http 1.x StatusCode to actix-web's http 0.2.x StatusCode
fn convert_status_code(perseus_status: http::StatusCode) -> actix_web::http::StatusCode {
    actix_web::http::StatusCode::from_u16(perseus_status.as_u16())
        .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
}

// ----- Request conversion implementation -----

/// Converts an Actix Web request into an `http::request`.
pub fn convert_req(raw: &actix_web::HttpRequest) -> Result<Request, String> {
    let mut builder = Request::builder();

    // Convert headers from actix's http 0.2.x to perseus's http 1.x
    for (name, val) in raw.headers() {
        if let Ok(perseus_name) = http::HeaderName::from_bytes(name.as_str().as_bytes()) {
            if let Ok(perseus_value) = http::HeaderValue::from_bytes(val.as_bytes()) {
                builder = builder.header(perseus_name, perseus_value);
            }
        }
    }

    // Convert URI, Method, and Version
    let perseus_uri = convert_uri(raw.uri())?;
    let perseus_method = convert_method(raw.method());
    let perseus_version = convert_version(raw.version());

    builder
        .uri(perseus_uri)
        .method(perseus_method)
        .version(perseus_version)
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
        // Convert perseus's http 1.x StatusCode to actix's http 0.2.x StatusCode
        let actix_status = convert_status_code(self.0.status);
        let mut res = HttpResponse::build(actix_status);

        // Convert headers from perseus's http 1.x to actix's http 0.2.x
        for (name, value) in &self.0.headers {
            if let Ok(actix_name) =
                actix_web::http::header::HeaderName::from_bytes(name.as_str().as_bytes())
            {
                if let Ok(actix_value) =
                    actix_web::http::header::HeaderValue::from_bytes(value.as_bytes())
                {
                    res.insert_header((actix_name, actix_value));
                }
            }
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
        let snippets_dir = opts.snippets.clone();
        cfg
            .app_data(web::Data::new(opts))
            // --- File handlers ---
            .route("/.perseus/bundle.js", web::get().to(js_bundle))
            .route("/.perseus/bundle.wasm", web::get().to(wasm_bundle))
            .route("/.perseus/bundle.wasm.js", web::get().to(wasm_js_bundle))
            .service(Files::new("/.perseus/snippets", &snippets_dir))
            // --- Translation and subsequent load handlers
            .route(
                "/.perseus/translations/{locale}",
                web::get().to(move |http_req: HttpRequest| async move {
                    let locale = http_req.match_info().query("locale");
                    ApiResponse(turbine.get_translations(locale).await)
                }),
            )
            .route(
                "/.perseus/initial_consts/{locale}.js",
                web::get().to(move |http_req: HttpRequest| async move {
                    let locale = http_req.match_info().query("locale");
                    ApiResponse(turbine.get_initial_consts(locale).await)
                })
            )
            .route(
                "/.perseus/initial_consts.js",
                web::get().to(move || async move {
                    ApiResponse(turbine.get_initial_consts("").await)
                })
            )
            .route(
                // We capture the `.json` ending in the handler
                "/.perseus/page/{locale}/{filename:.*}",
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
                    ).await)
                }),
            );
        // --- Static directory and alias handlers
        if turbine.static_dir.exists() {
            cfg.service(Files::new("/.perseus/static", &turbine.static_dir));
        }
        for url in turbine.static_aliases.keys() {
            cfg.route(
                url,
                web::get().to(|req| async { static_alias(turbine, req).await }),
            );
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

// File handlers (these have to be broken out for Actix)
async fn js_bundle(
    opts: web::Data<ServerOptions>,
) -> std::io::Result<CustomizeResponder<NamedFile>> {
    search_for_pre_compressed_version(
        &opts.js_bundle,
        "application/javascript; charset=utf-8".to_string(),
    )
}

async fn wasm_bundle(
    opts: web::Data<ServerOptions>,
) -> std::io::Result<CustomizeResponder<NamedFile>> {
    search_for_pre_compressed_version(&opts.wasm_bundle, "application/wasm".to_string())
}

async fn wasm_js_bundle(
    opts: web::Data<ServerOptions>,
) -> std::io::Result<CustomizeResponder<NamedFile>> {
    search_for_pre_compressed_version(
        &opts.wasm_js_bundle,
        "application/javascript; charset=utf-8".to_string(),
    )
}

fn search_for_pre_compressed_version(
    path: &str,
    application_type: String,
) -> std::io::Result<CustomizeResponder<NamedFile>> {
    let pre_compressed_path = format!("{}.br", path);
    match NamedFile::open(pre_compressed_path) {
        Ok(file) => Ok(file
            .customize()
            .insert_header(("Content-Encoding".to_string(), "br".to_string()))
            .insert_header(("Content-Type".to_string(), application_type))),
        Err(_) => match NamedFile::open(path) {
            Ok(file) => Ok(file
                .customize()
                .insert_header(("Content-Type".to_string(), application_type))),
            Err(e) => Err(e),
        },
    }
}

async fn static_alias<M: MutableStore, T: TranslationsManager>(
    turbine: &'static Turbine<M, T>,
    req: HttpRequest,
) -> std::io::Result<NamedFile> {
    let filename = turbine.static_aliases.get(req.path());
    let filename = match filename {
        Some(filename) => filename,
        // If the path doesn't exist, then the alias is not found
        None => return Err(std::io::Error::from(std::io::ErrorKind::NotFound)),
    };
    NamedFile::open(filename)
}

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

/// Creates and starts the default Perseus server with GZIP compression enabled
/// using Actix Web. This should be run in a `main()` function annotated with
/// `#[tokio::main]` (which requires the `macros` and `rt-multi-thread` features
/// on the `tokio` dependency).
#[cfg(feature = "dflt-server-with-compression")]
pub async fn dflt_server_with_compression<
    M: MutableStore + 'static,
    T: TranslationsManager + 'static,
>(
    turbine: &'static Turbine<M, T>,
    opts: ServerOptions,
    (host, port): (String, u16),
) {
    use actix_web::{App, HttpServer};
    use futures::executor::block_on;
    // TODO Fix issues here
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Compress::default())
            .configure(block_on(configurer(turbine, opts.clone())))
    })
    .bind((host, port))
    .expect(
        "Couldn't bind to given address. Maybe something is already running on the selected port?",
    )
    .run()
    .await
    .expect("Server failed.") // TODO Improve error message here
}
