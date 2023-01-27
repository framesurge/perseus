use crate::errors::*;
use minify_html_onepass::{with_friendly_error, Cfg};

/// Minifies the given HTML document, including any CSS and JS. This is
/// *extremely* fast, and can be reasonably run before returning a request.
///
/// If the second argument is set to `false`, CSS and JS will not be minified,
/// and the performance will be improved.
pub(crate) fn minify(code: &str, minify_extras: bool) -> Result<String, ServerError> {
    // In case the user is using invalid HTML (very tricky error to track down), we
    // let them disable this feature
    if cfg!(feature = "minify") {
        let cfg = Cfg {
            minify_js: minify_extras && cfg!(feature = "minify-js"),
            minify_css: minify_extras && cfg!(feature = "minify-css"),
        };
        let mut bytes = code.as_bytes().to_vec();

        match with_friendly_error(&mut bytes, &cfg) {
            Ok(min_len) => Ok(std::str::from_utf8(&bytes[..min_len]).unwrap().to_string()),
            Err(err) => Err(ServerError::MinifyError {
                // We have to wrap this because the error types are non-`StdError`
                // We want the error to be nice for the user (and it's not a security risk, since
                // the HTML is being sent to the client anyway, if this is being run on the server)
                source: std::io::Error::new(std::io::ErrorKind::NotFound, format!("{:#?}", err)),
            }),
        }
    } else {
        Ok(code.to_string())
    }
}
