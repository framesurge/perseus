use axum::{Router, body::Body, extract::{Path, Query}, http::{StatusCode, Request}, response::{Response, IntoResponse}, routing::{get, get_service}};
use perseus::{i18n::TranslationsManager, server::ServerOptions, stores::MutableStore, turbine::Turbine};
use serde::Deserialize;
use tower_http::services::{ServeDir, ServeFile};
use perseus::turbine::ApiResponse as PerseusApiResponse;

struct ApiResponse(PerseusApiResponse);
impl From<PerseusApiResponse> for ApiResponse {
    fn from(val: PerseusApiResponse) -> Self {
        Self(val)
    }
}
// Axum allows anything that implements `IntoResponse` to be returned, so we'll implement that for `ApiResponse`
impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        let headers = self.0.headers.unwrap_or_default();
        // Very convenient!
        (self.0.status, headers, self.0.body).into_response()
    }
}

#[derive(Deserialize)]
struct PageDataReq {
    pub entity_name: String,
    pub was_incremental_match: bool,
}

/// Gets the `Router` needed to configure an existing Axum app for Perseus, and
/// should be provided after any other routes, as they include a wildcard route.
pub async fn get_router<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    turbine: &'static Turbine<M, T>,
    opts: ServerOptions,
) -> Router {
    let router = Router::new()
        .route(
            "/.perseus/bundle.js",
            get_service(ServeFile::new(opts.js_bundle.clone())).handle_error(handle_fs_error),
        )
        .route(
            "/.perseus/bundle.wasm",
            get_service(ServeFile::new(opts.wasm_bundle.clone())).handle_error(handle_fs_error),
        )
        .route(
            "/.perseus/bundle.wasm.js",
            get_service(ServeFile::new(opts.wasm_js_bundle.clone())).handle_error(handle_fs_error),
        )
        .route(
            "/.perseus/snippets/*path",
            get_service(ServeDir::new(opts.snippets.clone())).handle_error(handle_fs_error),
        );
    let mut router = router
        .route(
            "/.perseus/translations/:locale",
            get(move |Path(locale): Path<String>| async move {
                ApiResponse(turbine.get_translations(&locale).await)
            }),
        )
        .route("/.perseus/page/:locale/*tail", get(move |Path(path_parts): Path<Vec<String>>,
                                                   Query(PageDataReq {
                                                       entity_name,
                                                       was_incremental_match,
                                                   }): Query<PageDataReq>, http_req: Request<Body>| async move {
                                                       // Separate the locale from the rest of the page name
                                                       let locale = &path_parts[0];
                                                       let raw_path = path_parts[1..]
                                                           .iter()
                                                           .map(|x| x.as_str())
                                                           .collect::<Vec<&str>>()
                                                           .join("/");
                                                       // Get rid of the body from the request (Perseus only needs the metadata)
                                                       let req = Request::from_parts(http_req.into_parts().0, ());

                                                       ApiResponse(turbine.get_subsequent_load(
                                                           &raw_path,
                                                           locale,
                                                           &entity_name,
                                                           was_incremental_match,
                                                           req
                                                       ).await.into())
                                                   }));
    // Only add the static content directory route if such a directory is being used
    if turbine.static_dir.exists() {
        router = router.nest_service(
            "/.perseus/static",
            get_service(ServeDir::new(&turbine.static_dir)).handle_error(handle_fs_error),
        )
    }
    // Now add support for serving static aliases
    for (url, static_path) in turbine.static_aliases.iter() {
        // Note that `static_path` is already relative to the right place
        // (`.perseus/server/`)
        router = router.route(
            url, // This comes with a leading forward slash!
            get_service(ServeFile::new(static_path)).handle_error(handle_fs_error),
        );
    }
    // And add the fallback for initial loads
    router.fallback_service(get(move |http_req: Request<Body>| async move {
        // Since this is a fallback handler, we have to do everything from the request itself
        let path = http_req.uri().path().to_string();
        let http_req = Request::from_parts(http_req.into_parts().0, ());

        ApiResponse(turbine.get_initial_load(&path, http_req).await)
    }))
}

// TODO Review if there's anything more to do here
async fn handle_fs_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Couldn't serve file.")
}
