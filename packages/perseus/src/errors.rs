#![allow(missing_docs)]

use crate::config_manager::ConfigManagerError;
use crate::translations_manager::TranslationsManagerError;
use thiserror::Error;

/// All errors that can be returned from this crate.
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ClientError(#[from] ClientError),
    #[error(transparent)]
    ServerError(#[from] ServerError),
}

/// Errors that can occur in the browser.
#[derive(Error, Debug)]
pub enum ClientError {
    #[error("locale '{locale}' is not supported")]
    LocaleNotSupported { locale: String },
    /// This converts from a `JsValue` or the like.
    #[error("the following error occurred while interfacing with JavaScript: {0}")]
    Js(String),
    #[error(transparent)]
    FetchError(#[from] FetchError),
}

/// Errors that can occur in the build process or while the server is running.
#[derive(Error, Debug)]
pub enum ServerError {
    #[error("render function '{fn_name}' in template '{template_name}' failed (cause: {cause:?})")]
    RenderFnFailed {
        // This is something like `build_state`
        fn_name: String,
        template_name: String,
        cause: ErrorCause,
        // This will be triggered by the user's custom render functions, which should be able to have any error type
        // TODO figure out custom error types on render functions
        #[source]
        source: Box<dyn std::error::Error>,
    },
    #[error(transparent)]
    ConfigManagerError(#[from] ConfigManagerError),
    #[error(transparent)]
    TranslationsManagerError(#[from] TranslationsManagerError),
    #[error(transparent)]
    BuildError(#[from] BuildError),
    #[error(transparent)]
    ExportError(#[from] ExportError),
    #[error(transparent)]
    ServeError(#[from] ServeError),
}
/// Converts a server error into an HTTP status code.
pub fn err_to_status_code(err: &ServerError) -> u16 {
    match err {
        ServerError::ServeError(ServeError::PageNotFound { .. }) => 404,
        // Ambiguous (user-generated error), we'll rely on the given cause
        ServerError::RenderFnFailed { cause, .. } => match cause {
            ErrorCause::Client(code) => code.unwrap_or(400),
            ErrorCause::Server(code) => code.unwrap_or(500),
        },
        // Any other errors go to a 500, they'll be misconfigurations or internal server errors
        _ => 500,
    }
}

/// Errors that can occur while fetching a resource from the server.
#[derive(Error, Debug)]
pub enum FetchError {
    #[error("asset fetched from '{url}' wasn't a string")]
    NotString { url: String },
    #[error("asset fetched from '{url}' returned status code '{status}' (expected 200)")]
    NotOk {
        url: String,
        status: u16,
        // The underlying body of the HTTP error response
        err: String,
    },
    #[error("asset fetched from '{url}' couldn't be serialized")]
    SerFailed {
        url: String,
        #[source]
        source: Box<dyn std::error::Error>,
    },
}

/// Errors that can occur while building an app.
#[derive(Error, Debug)]
pub enum BuildError {
    #[error("template '{template_name}' is missing feature '{feature_name}' (required due to its properties)")]
    TemplateFeatureNotEnabled {
        template_name: String,
        feature_name: String,
    },
    #[error("html shell couldn't be found at '{path}'")]
    HtmlShellNotFound {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error(
        "invalid indicator '{indicator}' in time string (must be one of: s, m, h, d, w, M, y)"
    )]
    InvalidDatetimeIntervalIndicator { indicator: String },
    #[error("asset 'render_cfg.json' invalid or corrupted (try cleaning all assets)")]
    RenderCfgInvalid {
        #[from]
        source: serde_json::Error,
    },
}

/// Errors that can occur while exporting an app to static files.
#[derive(Error, Debug)]
pub enum ExportError {
    #[error("template '{template_name}' can't be exported because it depends on strategies that can't be run at build-time (only build state and build paths can be use din exportable templates)")]
    TemplateNotExportable { template_name: String },
    #[error("template '{template_name}' wasn't found in built artifacts (run `perseus clean --dist` if this persists)")]
    TemplateNotFound { template_name: String },
}

/// Errors that can occur while serving an app. These are integration-agnostic.
#[derive(Error, Debug)]
pub enum ServeError {
    #[error("page at '{path}' not found")]
    PageNotFound { path: String },
    #[error("both build and request states were defined for a template when only one or fewer were expected (should it be able to amalgamate states?)")]
    BothStatesDefined,
    #[error("couldn't parse revalidation datetime (try cleaning all assets)")]
    BadRevalidate {
        #[from]
        source: chrono::ParseError,
    },
}

/// Defines who caused an ambiguous error message so we can reliably create an HTTP status code. Specific status codes may be provided
/// in either case, or the defaults (400 for client, 500 for server) will be used.
#[derive(Debug)]
pub enum ErrorCause {
    Client(Option<u16>),
    Server(Option<u16>),
}

/// An error that has an attached cause that blames either the client or the server for its occurrence. You can convert any error
/// into this with `.into()` or `?`, which will set the cause to the server by default, resulting in a *500 Internal Server Error*
/// HTTP status code. If this isn't what you want, you'll need to initialize this explicitly.
#[derive(Debug)]
pub struct GenericErrorWithCause {
    /// The underlying error.
    pub error: Box<dyn std::error::Error>,
    /// The cause of the error.
    pub cause: ErrorCause,
}
// We should be able to convert any error into this easily (e.g. with `?`) with the default being to blame the server
impl<E: std::error::Error + 'static> From<E> for GenericErrorWithCause {
    fn from(error: E) -> Self {
        Self {
            error: error.into(),
            cause: ErrorCause::Server(None),
        }
    }
}
