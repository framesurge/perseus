#![doc = include_str!("../README.proj.md")]
/*!
## Packages

This is the API documentation for the `perseus-warp` package, which allows Perseus apps to run on Warp. Note that Perseus mostly uses [the book](https://framesurge.sh/perseus/en-US) for
documentation, and this should mostly be used as a secondary reference source. You can also find full usage examples [here](https://github.com/framesurge/perseus/tree/main/examples).
*/

#![cfg(engine)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

// Serving files from a map is *really* convoluted
mod static_content;
use crate::static_content::{serve_file, static_aliases_filter};

use perseus::http;
use perseus::turbine::ApiResponse as PerseusApiResponse;
use perseus::{
    i18n::TranslationsManager,
    path::*,
    server::ServerOptions,
    stores::MutableStore,
    turbine::{SubsequentLoadQueryParams, Turbine},
    Request,
};
use std::{path::PathBuf, sync::Arc};
use warp::{
    path::{FullPath, Tail},
    reply::Response,
    Filter, Rejection, Reply,
};

// ----- Request conversion implementation -----

/// A Warp filter for extracting an HTTP request directly, which is slightly different to how the Actix Web integration handles this. Modified from [here](https://github.com/seanmonstar/warp/issues/139#issuecomment-853153712).
pub fn get_http_req() -> impl Filter<Extract = (http::Request<()>,), Error = Rejection> + Copy {
    warp::any()
        .and(warp::method())
        .and(warp::filters::path::full())
        // Warp doesn't permit empty query strings without this extra config (see https://github.com/seanmonstar/warp/issues/905)
        .and(
            warp::filters::query::raw()
                .or_else(|_| async move { Ok::<_, Rejection>((String::new(),)) }),
        )
        .and(warp::header::headers_cloned())
        .and_then(|method, path: FullPath, query, headers| async move {
            let uri = http::uri::Builder::new()
                .path_and_query(format!("{}?{}", path.as_str(), query))
                .build()
                .unwrap();

            let mut request = http::Request::builder()
                .method(method)
                .uri(uri)
                .body(()) // We don't do anything with the body in Perseus, so this is irrelevant
                .unwrap();

            *request.headers_mut() = headers;

            Ok::<http::Request<()>, Rejection>(request)
        })
}

// ----- Newtype wrapper for response implementation -----

#[derive(Debug)]
struct ApiResponse(PerseusApiResponse);
impl From<PerseusApiResponse> for ApiResponse {
    fn from(val: PerseusApiResponse) -> Self {
        Self(val)
    }
}
impl Reply for ApiResponse {
    fn into_response(self) -> Response {
        let mut response = Response::new(self.0.body.into());
        *response.status_mut() = self.0.status;
        *response.headers_mut() = self.0.headers;
        response
    }
}

// ----- Integration code -----

/// The routes for Perseus. These will configure an existing Warp instance to
/// run Perseus, and should be provided after any other routes, as they include
/// a wildcard route.
pub async fn perseus_routes<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    turbine: &'static Turbine<M, T>,
    opts: ServerOptions,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // --- File handlers ---
    let js_bundle = warp::path!(".perseus" / "bundle.js")
        .and(warp::path::end())
        .and(warp::fs::file(opts.js_bundle.clone()));
    let js_bundle_compressed = warp::path!(".perseus" / "bundle.js")
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}.br", opts.js_bundle.clone())))
        .map(|reply| {
            warp::reply::with_header(
                reply,
                "Content-Type",
                "application/javascript; charset=utf-8",
            )
        })
        .map(|reply| warp::reply::with_header(reply, "Content-Encoding", "br"));

    let wasm_bundle_compressed = warp::path!(".perseus" / "bundle.wasm")
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}.br", opts.wasm_bundle.clone())))
        .map(|reply| warp::reply::with_header(reply, "Content-Type", "application/wasm"))
        .map(|reply| warp::reply::with_header(reply, "Content-Encoding", "br"));
    let wasm_bundle = warp::path!(".perseus" / "bundle.wasm")
        .and(warp::path::end())
        .and(warp::fs::file(opts.wasm_bundle.clone()));

    let wasm_js_bundle = warp::path!(".perseus" / "bundle.wasm.js")
        .and(warp::path::end())
        .and(warp::fs::file(opts.wasm_js_bundle.clone()));
    let wasm_js_bundle_compressed = warp::path!(".perseus" / "bundle.wasm.js")
        .and(warp::path::end())
        .and(warp::fs::file(format!(
            "{}.br",
            opts.wasm_js_bundle.clone()
        )))
        .map(|reply| {
            warp::reply::with_header(
                reply,
                "Content-Type",
                "application/javascript; charset=utf-8",
            )
        })
        .map(|reply| warp::reply::with_header(reply, "Content-Encoding", "br"));

    let snippets = warp::path!(".perseus" / "snippets" / ..).and(warp::fs::dir(opts.snippets));

    // --- Translation and subsequent load handlers ---
    let translations =
        warp::path!(".perseus" / "translations" / String).then(move |locale: String| async move {
            ApiResponse(turbine.get_translations(&locale).await)
        });
    let localized_initial_consts = warp::path!(".perseus" / "initial_consts" / String).then(
        move |locale: String| async move {
            let locale = locale.strip_suffix(".js").unwrap_or(&locale);
            ApiResponse(turbine.get_initial_consts(&locale).await)
        },
    );
    let unlocalized_initial_consts = warp::path!(".perseus" / "initial_consts.js")
        .then(move || async move { ApiResponse(turbine.get_initial_consts("").await) });
    let page_data = warp::path!(".perseus" / "page" / String / ..)
        .and(warp::path::tail())
        .and(warp::query::<SubsequentLoadQueryParams>())
        .and(get_http_req())
        .then(
            move |locale: String,
                  path: Tail, // This is the path after the locale that was sent
                  SubsequentLoadQueryParams {
                      entity_name,
                      was_incremental_match,
                  }: SubsequentLoadQueryParams,
                  http_req: Request| async move {
                ApiResponse(
                    turbine
                        .get_subsequent_load(
                            PathWithoutLocale(path.as_str().to_string()),
                            locale,
                            entity_name,
                            was_incremental_match,
                            http_req,
                        )
                        .await,
                )
            },
        );

    // --- Static directory and alias handlers ---
    let static_dir_path = Arc::new(turbine.static_dir.clone());
    let static_dir_path_filter = warp::any().map(move || static_dir_path.clone());
    let static_dir = warp::path!(".perseus" / "static" / ..)
        .and(static_dir_path_filter)
        .and_then(|static_dir_path: Arc<PathBuf>| async move {
            if static_dir_path.exists() {
                Ok(())
            } else {
                Err(warp::reject::not_found())
            }
        })
        .untuple_one() // We need this to avoid a ((), File) (which makes the return type fail)
        // This alternative will never be served, but if we don't have it we'll get a runtime panic
        .and(warp::fs::dir(turbine.static_dir.clone()));
    let static_aliases = warp::any()
        .and(static_aliases_filter(turbine.static_aliases.clone()))
        .and_then(serve_file);

    // --- Initial load handler ---
    let initial_loads = warp::any()
        .and(warp::path::full())
        .and(get_http_req())
        .then(move |path: FullPath, http_req: Request| async move {
            ApiResponse(
                turbine
                    .get_initial_load(PathMaybeWithLocale(path.as_str().to_string()), http_req)
                    .await,
            )
        });

    // Now put all those routes together in the final thing (the user will add this
    // to an existing Warp server)
    js_bundle_compressed
        .or(js_bundle)
        .or(wasm_bundle_compressed)
        .or(wasm_bundle)
        .or(wasm_js_bundle_compressed)
        .or(wasm_js_bundle)
        .or(snippets)
        .or(static_dir)
        .or(static_aliases)
        .or(translations)
        .or(localized_initial_consts)
        .or(unlocalized_initial_consts)
        .or(page_data)
        .or(initial_loads)
}

// ----- Default server -----

/// Creates and starts the default Perseus server with Warp. This should be run
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
    let routes = perseus_routes(turbine, opts).await;
    warp::serve(routes).run(addr).await;
}

/// Creates and starts the Warp Perseus server with compression enable. This should be run
/// in a `main` function annotated with `#[tokio::main]` (which requires the
/// `macros` and `rt-multi-thread` features on the `tokio` dependency).
#[cfg(feature = "dflt-server-with-compression")]
pub async fn dflt_server_with_compression<
    M: MutableStore + 'static,
    T: TranslationsManager + 'static,
>(
    turbine: &'static Turbine<M, T>,
    opts: ServerOptions,
    (host, port): (String, u16),
) {
    use std::net::SocketAddr;

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address provided to bind to.");
    let routes = perseus_routes(turbine, opts).await;
    warp::serve(routes.with(warp::compression::gzip()))
        .run(addr)
        .await;
}
