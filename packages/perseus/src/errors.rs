#![allow(missing_docs)]

#[cfg(not(target_arch = "wasm32"))]
use crate::i18n::TranslationsManagerError;
use thiserror::Error;

/// All errors that can be returned from this crate.
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ClientError(#[from] ClientError),
    #[cfg(not(target_arch = "wasm32"))]
    #[error(transparent)]
    ServerError(#[from] ServerError),
    #[cfg(all(feature = "builder", not(target_arch = "wasm32")))]
    #[error(transparent)]
    EngineError(#[from] EngineError),
}

/// Errors that can occur in the server-side engine system (responsible for
/// building the app).
#[cfg(all(feature = "builder", not(target_arch = "wasm32")))]
#[derive(Error, Debug)]
pub enum EngineError {
    // Many of the build/export processes return these more generic errors
    #[error(transparent)]
    ServerError(#[from] ServerError),
    #[error("couldn't copy static directory at '{path}' to '{dest}'")]
    CopyStaticDirError {
        #[source]
        source: fs_extra::error::Error,
        path: String,
        dest: String,
    },
    #[error("couldn't copy static alias file from '{from}' to '{to}'")]
    CopyStaticAliasFileError {
        #[source]
        source: std::io::Error,
        from: String,
        to: String,
    },
    #[error("couldn't copy static alias directory from '{from}' to '{to}'")]
    CopyStaticAliasDirErr {
        #[source]
        source: fs_extra::error::Error,
        from: String,
        to: String,
    },
    #[error("couldn't write the generated error page to '{dest}'")]
    WriteErrorPageError {
        #[source]
        source: std::io::Error,
        dest: String,
    },
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
    #[error("invalid frozen state provided")]
    ThawFailed {
        #[source]
        source: serde_json::Error,
    },
    // If the user is using the template macros, this should never be emitted because we can
    // ensure that the generated state is valid
    #[error("tried to deserialize invalid state")]
    StateInvalid {
        #[source]
        source: serde_json::Error,
    },
}

/// Errors that can occur in the build process or while the server is running.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Error, Debug)]
pub enum ServerError {
    #[error("render function '{fn_name}' in template '{template_name}' failed (cause: {cause:?})")]
    RenderFnFailed {
        // This is something like `build_state`
        fn_name: String,
        template_name: String,
        cause: ErrorCause,
        // This will be triggered by the user's custom render functions, which should be able to
        // have any error type
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    #[error(transparent)]
    GlobalStateError(#[from] GlobalStateError),
    #[error(transparent)]
    StoreError(#[from] StoreError),
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
#[cfg(not(target_arch = "wasm32"))]
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

/// Errors that can occur with regards to global state.
#[derive(Error, Debug)]
pub enum GlobalStateError {
    #[error("couldn't generate global state at build time")]
    BuildGenerationFailed {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

/// Errors that can occur while reading from or writing to a mutable or
/// immutable store.
// We do need this on the client to complete some things
#[derive(Error, Debug)]
pub enum StoreError {
    #[error("asset '{name}' not found in store")]
    NotFound { name: String },
    #[error("asset '{name}' couldn't be read from store")]
    ReadFailed {
        name: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    #[error("asset '{name}' couldn't be written to store")]
    WriteFailed {
        name: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
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
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

/// Errors that can occur while building an app.
#[cfg(not(target_arch = "wasm32"))]
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
#[cfg(not(target_arch = "wasm32"))]
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
    #[cfg(not(target_arch = "wasm32"))]
    #[error("couldn't parse revalidation datetime (try cleaning all assets)")]
    BadRevalidate {
        #[from]
        source: chrono::ParseError,
    },
}

/// Defines who caused an ambiguous error message so we can reliably create an
/// HTTP status code. Specific status codes may be provided in either case, or
/// the defaults (400 for client, 500 for server) will be used.
#[derive(Debug)]
pub enum ErrorCause {
    Client(Option<u16>),
    Server(Option<u16>),
}

/// An error that has an attached cause that blames either the client or the
/// server for its occurrence. You can convert any error into this with
/// `.into()` or `?`, which will set the cause to the server by default,
/// resulting in a *500 Internal Server Error* HTTP status code. If this isn't
/// what you want, you'll need to initialize this explicitly.
#[derive(Debug)]
pub struct GenericErrorWithCause {
    /// The underlying error.
    pub error: Box<dyn std::error::Error + Send + Sync>,
    /// The cause of the error.
    pub cause: ErrorCause,
}
// We should be able to convert any error into this easily (e.g. with `?`) with
// the default being to blame the server
impl<E: std::error::Error + Send + Sync + 'static> From<E> for GenericErrorWithCause {
    fn from(error: E) -> Self {
        Self {
            error: error.into(),
            cause: ErrorCause::Server(None),
        }
    }
}

/// Creates a new [`GenericErrorWithCause` (the error type behind [`RenderFnResultWithCause`]) efficiently. This allows you to
/// explicitly return errors from any state-generation functions, including both an error and a statement of
/// whether the server or the client is responsible. With this macro, you can
/// use any of the following syntaxes (substituting `"error!"` for any error
/// that can be converted with `.into()` into a `Box<dyn std::error::Error>`):
///
/// - `blame_err!(client, "error!")` -- an error that's the client's fault, with
///   the default HTTP status code (400, a generic client error)
/// - `blame_err!(server, "error!")` -- an error that's the server's fault, with
///   the default HTTP status code (500, a generic server error)
/// - `blame_err!(client, 404, "error!")` -- an error that's the client's fault,
///   with a custom HTTP status code (404 in this example)
/// - `blame_err!(server, 501, "error!")` -- an error that's the server's fault,
///   with a custom HTTP status code (501 in this example)
///
/// Note that this macro will automatically `return` the error it creates.
#[macro_export]
macro_rules! blame_err {
    (client, $err:expr) => {
        return ::std::result::Result::Err(::perseus::GenericErrorWithCause {
            error: $err.into(),
            cause: $crate::ErrorCause::Client(::std::option::Option::None),
        })
    };
    (client, $code:literal, $err:expr) => {
        return ::std::result::Result::Err(::perseus::GenericErrorWithCause {
            error: $err.into(),
            cause: $crate::ErrorCause::Client(::std::option::Option::Some($code)),
        })
    };
    (server, $err:expr) => {
        return ::std::result::Result::Err(::perseus::GenericErrorWithCause {
            error: $err.into(),
            cause: $crate::ErrorCause::Server(::std::option::Option::None),
        })
    };
    (server, $code:literal, $err:expr) => {
        return ::std::result::Result::Err(::perseus::GenericErrorWithCause {
            error: $err.into(),
            cause: $crate::ErrorCause::Server(::std::option::Option::Some($code)),
        })
    };
}
