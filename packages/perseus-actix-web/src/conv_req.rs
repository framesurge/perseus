use crate::errors::*;
use perseus::{HttpRequest, Request};

/// Converts an Actix Web request into an `http::request`.
pub fn convert_req(raw: &actix_web::HttpRequest) -> Result<Request, Error> {
    let mut builder = HttpRequest::builder();

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
        .map_err(|err| Error::RequestConversionFailed { source: err })
}
