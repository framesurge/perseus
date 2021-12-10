use perseus::http;
use warp::hyper::body::Bytes;
use warp::{path::FullPath, Filter, Rejection};

/// A Warp filter for extracting an HTTP request directly, which is slightly different to how the Actix Web integration handles this. Modified from [here](https://github.com/seanmonstar/warp/issues/139#issuecomment-853153712).
pub fn get_http_req() -> impl Filter<Extract = (http::Request<Bytes>,), Error = Rejection> + Copy {
    warp::any()
        .and(warp::method())
        .and(warp::filters::path::full())
        .and(warp::filters::query::raw())
        .and(warp::header::headers_cloned())
        .and(warp::body::bytes())
        .and_then(|method, path: FullPath, query, headers, bytes| async move {
            let uri = http::uri::Builder::new()
                .path_and_query(format!("{}?{}", path.as_str(), query))
                .build()
                .unwrap();

            let mut request = http::Request::builder()
                .method(method)
                .uri(uri)
                .body(bytes)
                .unwrap();

            *request.headers_mut() = headers;

            Ok::<http::Request<Bytes>, Rejection>(request)
        })
}
