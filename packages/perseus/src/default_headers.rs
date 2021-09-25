use http::header::HeaderMap;

/// Creates the default headers used in Perseus. This is the default value for `set_headers` on every `Template<G>`
// TODO
pub(crate) fn default_headers() -> HeaderMap {
    HeaderMap::new()
}
