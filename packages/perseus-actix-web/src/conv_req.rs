use crate::errors::*;
use perseus::{HttpRequest, Request};

/// Converts an Actix Web request into an `http::request`.
pub fn convert_req(raw: &actix_web::HttpRequest) -> Result<Request, Error> {
    let mut builder = HttpRequest::builder();
    // Add headers one by one
    for (name, val) in raw.headers() {
        // Each method call consumes and returns `self`, so we re-self-assign
        builder = builder.header(name, val);
    }
    // The URI to which the request was sent
    builder = builder.uri(raw.uri());
    // The method (e.g. GET, POST, etc.)
    builder = builder.method(raw.method());
    // The HTTP version used
    builder = builder.version(raw.version());

    builder
        // We always use an empty body because, in a Perseus request, only the URI matters
        // Any custom data should therefore be sent in headers (if you're doing that, consider a dedicated API)
        .body(())
        .map_err(|err| Error::RequestConversionFailed { source: err })
}
