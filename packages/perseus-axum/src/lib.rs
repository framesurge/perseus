#![doc = include_str!("../README.proj.md")]
/*!
## Packages

This is the API documentation for the `perseus-axum` package, which allows Perseus apps to run on Axum. Note that Perseus mostly uses [the book](https://framesurge.sh/perseus/en-US) for
documentation, and this should mostly be used as a secondary reference source. You can also find full usage examples [here](https://github.com/framesurge/perseus/tree/main/examples).
 */

#![cfg(engine)] // This crate needs to be run with the Perseus CLI
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

use axum::{
    body::Body,
    extract::{Path, Query},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, get_service},
    Router,
    handler::Handler,
};
use perseus::turbine::ApiResponse as PerseusApiResponse;
use perseus::{
    i18n::TranslationsManager,
    path::*,
    server::ServerOptions,
    stores::MutableStore,
    turbine::{SubsequentLoadQueryParams, Turbine},
};
use tower_http::services::{ServeDir, ServeFile};

// ----- Request conversion implementation -----

// Not needed, since Axum uses `http::Request` under the hood, and we can just
// change the body type to `()`.

// ----- Newtype wrapper for response implementation -----

#[derive(Debug)]
struct ApiResponse(PerseusApiResponse);

impl From<PerseusApiResponse> for ApiResponse {
    fn from(val: PerseusApiResponse) -> Self {
        Self(val)
    }
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        // Very convenient!
        (self.0.status, self.0.headers, self.0.body).into_response()
    }
}

// ----- Integration code -----

/// Gets the `Router` needed to configure an existing Axum app for Perseus, and
/// should be provided after any other routes, as they include a wildcard route.
pub async fn get_router<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    turbine: &'static Turbine<M, T>,
    opts: ServerOptions,
) -> Router {
    let router = Router::new()
        // --- File handlers ---
        .route(
            "/.perseus/bundle.js",
            get_service(ServeFile::new(opts.js_bundle.clone()).precompressed_br()).handle_error(handle_fs_error),
        )
        .route(
            "/.perseus/bundle.wasm",
            get_service(ServeFile::new(opts.wasm_bundle.clone()).precompressed_br()).handle_error(handle_fs_error),
        )
        .route(
            "/.perseus/bundle.wasm.js",
            get_service(ServeFile::new(opts.wasm_js_bundle.clone()).precompressed_br()).handle_error(handle_fs_error),
        )
        .nest_service(
            "/.perseus/snippets",
            get_service(ServeDir::new(opts.snippets).precompressed_br()).handle_error(handle_fs_error),
        );

    // --- Translation and subsequent load handlers ---
    let mut router = router
        .route(
            "/.perseus/translations/:locale",
            get(move |Path(locale): Path<String>| async move {
                ApiResponse(turbine.get_translations(&locale).await)
            }),
        )
        .route(
            "/.perseus/initial_consts.js",
            get(move || async move { ApiResponse(turbine.get_initial_consts("").await) }),
        )
        .route(
            "/.perseus/initial_consts/:locale",
            get(move |Path(locale): Path<String>| async move {
                let locale = match locale.strip_suffix(".js") {
                    Some(locale) => locale,
                    None => {
                        return ApiResponse(PerseusApiResponse::err(
                            StatusCode::BAD_REQUEST,
                            "invalid locale (needs `.js` extension)",
                        ))
                    }
                };
                ApiResponse(turbine.get_initial_consts(&locale).await)
            }),
        )
        .route(
            "/.perseus/page/:locale/*tail",
            get(
                move |Path(path_parts): Path<Vec<String>>,
                      Query(SubsequentLoadQueryParams {
                                entity_name,
                                was_incremental_match,
                            }): Query<SubsequentLoadQueryParams>,
                      http_req: Request<Body>| async move {
                    // Separate the locale from the rest of the page name
                    let locale = &path_parts[0];
                    let raw_path = path_parts[1..]
                        .iter()
                        .map(|x| x.as_str())
                        .collect::<Vec<&str>>()
                        .join("/");
                    // Get rid of the body from the request (Perseus only needs the metadata)
                    let req = Request::from_parts(http_req.into_parts().0, ());

                    ApiResponse(
                        turbine
                            .get_subsequent_load(
                                PathWithoutLocale(raw_path),
                                locale.to_string(),
                                entity_name,
                                was_incremental_match,
                                req,
                            )
                            .await,
                    )
                },
            ),
        );

    // --- Static directory and alias handlers ---
    if turbine.static_dir.exists() {
        router = router.nest_service(
            "/.perseus/static",
            get_service(ServeDir::new(&turbine.static_dir)).handle_error(handle_fs_error),
        )
    }
    for (url, static_path) in turbine.static_aliases.iter() {
        router = router.route(
            url, // This comes with a leading forward slash!
            get_service(ServeFile::new(static_path)).handle_error(handle_fs_error),
        );
    }

    // --- Initial load handler ---
    router.fallback_service(get(move |http_req: Request<Body>| async move {
        // Since this is a fallback handler, we have to do everything from the request
        // itself
        let path = http_req.uri().path().to_string();
        let http_req = Request::from_parts(http_req.into_parts().0, ());

        ApiResponse(
            turbine
                .get_initial_load(PathMaybeWithLocale(path), http_req)
                .await,
        )
    }))
}

// TODO Review if there's anything more to do here
async fn handle_fs_error(_err: std::io::Error) -> impl IntoResponse {
    dbg!("Error!");
    (StatusCode::INTERNAL_SERVER_ERROR, "Couldn't serve file.")
}


// ----- Default server -----

/// Creates and starts the default Perseus server with Axum. This should be run
/// in a `main` function annotated with `#[tokio::main]` (which requires the
/// `macros` and `rt-multi-thread` features on the `tokio` dependency).
#[cfg(feature = "dflt-server")]
pub async fn dflt_server<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    turbine: &'static Turbine<M, T>,
    opts: ServerOptions,
    (host, port): (String, u16),
) {
    use std::net::SocketAddr;

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address provided to bind to.");

    let app = get_router(turbine, opts).await;

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Creates and starts the default Perseus server with compression using Axum. This should be run
/// in a `main` function annotated with `#[tokio::main]` (which requires the
/// `macros` and `rt-multi-thread` features on the `tokio` dependency).
#[cfg(feature = "dflt-server-with-compression")]
pub async fn dflt_server_with_compression<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    turbine: &'static Turbine<M, T>,
    opts: ServerOptions,
    (host, port): (String, u16),
) {
    use std::net::SocketAddr;

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address provided to bind to.");

    let app = get_router(turbine, opts).await.layer(
            tower_http::compression::CompressionLayer::new()
        );

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
