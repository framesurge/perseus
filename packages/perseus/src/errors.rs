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
    #[cfg(not(target_arch = "wasm32"))]
    #[error(transparent)]
    EngineError(#[from] EngineError),
    // Plugin errors could come from literally anywhere, and could have entirely arbitrary data
    #[error(transparent)]
    PluginError(#[from] PluginError),
}

#[derive(Error, Debug)]
#[error("plugin '{name}' returned an error (this is unlikely to be Perseus' fault)")]
pub struct PluginError {
    pub name: String,
    #[source]
    pub source: Box<dyn std::error::Error + Send + Sync>,
}

/// Errors that can occur in the server-side engine system (responsible for
/// building the app).
#[cfg(not(target_arch = "wasm32"))]
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
///
/// **Important:** any changes to this `enum` constitute a breaking change,
/// since users match this in their error pages. Changes in underlying
/// `enum`s are not considered breaking (e.g. introducing a new invariant error).
///
/// **Warning:** in all these cases, except `ClientError::ServerError`, the user can
/// already see the prerendered version of the page, it just isn't interactive. Only
/// in that case will your error page occupy the entire screen, otherwise it will
/// be placed into a `div` with the class `__perseus-error`, a deliberate choice
/// to reinforce the best practice of giving the user as much as possible (it might
/// not be interactive, but they can still use a rudimentary version). See the book
/// for further details.
#[derive(Error, Debug)]
pub enum ClientError {
    #[error(transparent)]
    PluginError(#[from] PluginError),
    #[error(transparent)]
    InvariantError(#[from] ClientInvariantError),
    #[error(transparent)]
    ThawError(#[from] ClientThawError),
    // Not like the `ServerError` in this file!
    #[error("an error with HTTP status code '{status}' was returned by the server: '{message}'")]
    ServerError {
        status: u16,
        // This has to have been serialized unfortunately
        message: String,
    },
    #[error(transparent)]
    FetchError(#[from] FetchError)

    // #[error("locale '{locale}' is not supported")]
    // LocaleNotSupported { locale: String },
    // /// This converts from a `JsValue` or the like.
    // #[error("the following error occurred while interfacing with JavaScript: {0}")]
    // Js(String),
    // #[error(transparent)]
    // FetchError(#[from] FetchError),
    // ,
    // // If the user is using the template macros, this should never be emitted because we can
    // // ensure that the generated state is valid
    // #[error("tried to deserialize invalid state (it was not malformed, but the state was not of the declared type)")]
    // StateInvalid {
    //     #[source]
    //     source: serde_json::Error,
    // },
    // #[error("server informed us that a valid locale was invald (this almost certainly requires a hard reload)")]
    // ValidLocaleNotProvided { locale: String },
    // #[error("the given path for preloading leads to a locale detection page; you probably wanted to wrap the path in `link!(...)`")]
    // PreloadLocaleDetection,
    // #[error("the given path for preloading was not found")]
    // PreloadNotFound,
}

/// Errors that can occur in the browser from certain invariants not being upheld. These
/// should be extremely rare, but, since we don't control what HTML the browser gets, we avoid
/// panicking in these cases.
///
/// Note that some of these invariants may be broken by an app's own code, such as invalid global
/// state downcasting.
#[derive(Debug, Error)]
pub enum ClientInvariantError {
    #[error("the render configuration was not found, or was malformed")]
    RenderCfg,
    #[error("the global state was not found, or was malformed (even apps not using global state should have an empty one injected)")]
    GlobalState,
    // This won't be triggered for HSR
    #[error("attempted to register state on a page/capsule that had been previously declared as having no state")]
    IllegalStateRegistration,
    #[error("attempted to downcast reactive global state to the incorrect type (this is an error)")]
    GlobalStateDowncast,
    // This is technically a typing error, but we do the typing internally, so this should be impossible
    #[error("invalid page/widget state found")]
    InvalidState {
        #[source]
        source: serde_json::Error,
    },
    // Invariant because the user would have had to call something like `.template_with_state()` for this to happen
    #[error("no state was found for a page/widget that expected state (you might have forgotten to write a state generation function, like `get_build_state`)")]
    NoState,
    #[error("the initial state was not found, or was malformed")]
    InitialState,
    #[error("the initial state denoted an error, but this was malformed")]
    InitialStateError {
        #[source]
        source: serde_json::Error,
    },
    #[error("the locale '{locale}', which is supported by this app, was not returned by the server")]
    ValidLocaleNotProvided {
        locale: String,
    },
    // This is just for initial loads (`__PERSEUS_TRANSLATIONS` window variable)
    #[error("the translations were not found, or were malformed (even apps not using i18n have a declaration of their lack of translations)")]
    Translations,
    #[error("we found the current page to be a 404, but the engine disagrees")]
    RouterMismatch
}

/// Errors that can occur in the browser while interfacing with browser functionality. These should never really
/// occur unless you're operating in an extremely alien environment (which probably wouldn't support Wasm, but
/// we try to allow maximal error page control).
#[derive(Debug, Error)]
pub enum ClientBrowserError {
    #[error("failed to get current url for initial load determination")]
    InitialPath
}

/// Errors that can occur in the browser as a result of attempting to thaw provided state.
#[derive(Debug, Error)]
pub enum ClientThawError {
    #[error("invalid frozen page/widget state")]
    InvalidFrozenState {
        #[source]
        source: serde_json::Error,
    },
    #[error("invalid frozen global state")]
    InvalidFrozenGlobalState {
        #[source]
        source: serde_json::Error,
    },
    #[error("this app uses global state, but the provided frozen state declared itself to have no global state")]
    NoFrozenGlobalState
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
    // We should only get a failure to minify if the user has given invalid HTML, or if Sycamore
    // stuffed up somewhere
    #[error("failed to minify html (you can disable the `minify` flag to avoid this; this is very likely a Sycamore bug, unless you've provided invalid custom HTML)")]
    MinifyError {
        #[source]
        source: std::io::Error,
    },
    #[error("failed to decode url provided (probably malformed request)")]
    UrlDecodeFailed {
        #[source]
        source: std::string::FromUtf8Error,
    },
    #[error("the template '{template_name}' had no helper build state written to the immutable store (the store has been tampered with, and the app must be rebuilt)")]
    MissingBuildExtra { template_name: String },
    #[error("the template '{template_name}' had invalid helper build state written to the immutable store (the store has been tampered with, and the app must be rebuilt)")]
    InvalidBuildExtra {
        template_name: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("page state was encountered that could not be deserialized into serde_json::Value (the store has been tampered with, and the app must be rebuilt)")]
    InvalidPageState {
        #[source]
        source: serde_json::Error,
    },
    #[error("attempting to resolve dependency '{widget}' in locale '{locale}' produced a locale redirection verdict (this shouldn't be possible)")]
    ResolveDepLocaleRedirection {
        widget: String,
        locale: String,
    },
    #[error("attempting to resolve dependency '{widget}' in locale '{locale}' produced a not found verdict (did you mistype the widget path?)")]
    ResolveDepNotFound {
        widget: String,
        locale: String,
    },
    #[error("template '{template_name}' cannot be built at build-time due to one or more of its dependencies having state that may change later; to allow this template to be built later, add `.allow_rescheduling()` to your template definition")]
    TemplateCannotBeRescheduled {
        template_name: String,
    },
    // This is a serious error in programming
    #[error("a dependency tree was not resolved, but a function expecting it to have been was called (this is a server-side error)")]
    DepTreeNotResolved,
    #[error("the template name did not prefix the path (this request was severely malformed)")]
    TemplateNameNotInPath,

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
    #[error(transparent)]
    PluginError(#[from] PluginError),
    // This can occur in state acquisition failures during prerendering
    #[error(transparent)]
    ClientError(#[from] ClientError)
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
    #[error("asset of type '{ty}' fetched from '{url}' wasn't a string")]
    NotString {
        url: String,
        ty: AssetType,
    },
    #[error("asset of type '{ty}' fetched from '{url}' returned status code '{status}' (expected 200)")]
    NotOk {
        url: String,
        status: u16,
        // The underlying body of the HTTP error response
        err: String,
        ty: AssetType,
    },
    #[error("asset of type '{ty}' fetched from '{url}' couldn't be serialized")]
    SerFailed {
        url: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
        ty: AssetType,
    },
    // This is not used by the `fetch` function, but it is used by the preloading system
    #[error("preload asset fetched from '{url}' was not found")]
    PreloadNotFound {
        url: String,
        ty: AssetType,
    },
}

/// The type of an asset fetched from the server. This allows distinguishing between errors in
/// fetching, say, pages, vs. translations, which you may wish to handle differently.
#[derive(Debug)]
pub enum AssetType {
    /// A page in the app.
    Page,
    /// A widget in the app.
    Widget,
    /// Translations for a locale.
    Translations,
    /// A page/widget the user asked to have preloaded.
    Preload,
}
impl std::fmt::Display for AssetType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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
        #[source]
        source: serde_json::Error,
    },
}

/// Errors that can occur while exporting an app to static files.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Error, Debug)]
pub enum ExportError {
    #[error("template '{template_name}' can't be exported because it depends on strategies that can't be run at build-time (only build state and build paths can be used in exportable templates)")]
    TemplateNotExportable { template_name: String },
    #[error("template '{template_name}' wasn't found in built artifacts (run `perseus clean --dist` if this persists)")]
    TemplateNotFound { template_name: String },
    #[error("your app can't be exported because its global state depends on strategies that can't be run at build time (only build state can be used in exportable apps)")]
    GlobalStateNotExportable,
    #[error("template '{template_name} can't be exported because one or more of its widget dependencies use state generation strategies that can't be run at build-time")]
    DependenciesNotExportable { template_name: String },
    // This is used in error page exports
    #[error("invalid status code provided for error page export (please provide a valid http status code)")]
    InvalidStatusCode
}

/// Errors that can occur while serving an app. These are integration-agnostic.
#[derive(Error, Debug)]
pub enum ServeError {
    #[error("page/widget at '{path}' not found")]
    PageNotFound { path: String },
    #[error("both build and request states were defined for a template when only one or fewer were expected (should it be able to amalgamate states?)")]
    BothStatesDefined,
    #[cfg(not(target_arch = "wasm32"))]
    #[error("couldn't parse revalidation datetime (try cleaning all assets)")]
    BadRevalidate {
        #[source]
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

/// Creates a new [`GenericErrorWithCause` (the error type behind
/// [`RenderFnResultWithCause`](crate::RenderFnResultWithCause)) efficiently.
/// This allows you to explicitly return errors from any state-generation
/// functions, including both an error and a statement of whether the server or
/// the client is responsible. With this macro, you can use any of the following
/// syntaxes (substituting `"error!"` for any error that can be converted with
/// `.into()` into a `Box<dyn std::error::Error>`):
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

/// This macro is identical to [`blame_err!`], except it will simply return a
/// [`GenericErrorWithCause`], not `return`ing it from the caller function. This
/// is more useful if you're providing a blamed error to something like
/// `.map_err()`.
#[macro_export]
macro_rules! make_blamed_err {
    (client, $err:expr) => {
        ::perseus::GenericErrorWithCause {
            error: $err.into(),
            cause: $crate::ErrorCause::Client(::std::option::Option::None),
        }
    };
    (client, $code:literal, $err:expr) => {
        ::perseus::GenericErrorWithCause {
            error: $err.into(),
            cause: $crate::ErrorCause::Client(::std::option::Option::Some($code)),
        }
    };
    (server, $err:expr) => {
        ::perseus::GenericErrorWithCause {
            error: $err.into(),
            cause: $crate::ErrorCause::Server(::std::option::Option::None),
        }
    };
    (server, $code:literal, $err:expr) => {
        ::perseus::GenericErrorWithCause {
            error: $err.into(),
            cause: $crate::ErrorCause::Server(::std::option::Option::Some($code)),
        }
    };
}
