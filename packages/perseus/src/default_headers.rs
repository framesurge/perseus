use http::header::{self, HeaderMap};

/// Creates the default headers used in Perseus. This is the default value for `set_headers` on every `Template<G>`
pub(crate) fn default_headers() -> HeaderMap {
    let mut map = HeaderMap::new();
    map.insert(header::CACHE_CONTROL, "max-age=300".parse().unwrap());
    map
}
