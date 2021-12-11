use perseus::http;
use warp::{path::FullPath, Filter, Rejection};

/// A Warp filter for extracting an HTTP request directly, which is slightly different to how the Actix Web integration handles this. Modified from [here](https://github.com/seanmonstar/warp/issues/139#issuecomment-853153712).
pub fn get_http_req() -> impl Filter<Extract = (http::Request<()>,), Error = Rejection> + Copy {
    warp::any()
        .and(warp::method())
        .and(warp::filters::path::full())
        .and(warp::filters::query::raw())
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
