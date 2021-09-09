#![allow(missing_docs)]

pub use error_chain::bail;
use error_chain::error_chain;

/// Defines who caused an ambiguous error message so we can reliably create an HTTP status code. Specific status codes may be provided
/// in either case, or the defaults (400 for client, 500 for server) will be used.
#[derive(Debug)]
pub enum ErrorCause {
    Client(Option<u16>),
    Server(Option<u16>),
}

// The `error_chain` setup for the whole crate
error_chain! {
    // The custom errors for this crate (very broad)
    errors {
        /// For indistinct JavaScript errors (potentially sensitive, but only generated on the client-side).
        JsErr(err: String) {
            description("an error occurred while interfacing with javascript")
            display("the following error occurred while interfacing with javascript: {:?}", err)
        }
        /// For when a fetched URL didn't return a string, which it must.
        AssetNotString(url: String) {
            description("the fetched asset wasn't a string")
            display("the fetched asset at '{}' wasn't a string", url)
        }
        /// For when the server returned a non-200 error code (not including 404, that's handled separately).
        AssetNotOk(url: String, status: u16, err: String) {
            description("the asset couldn't be fecthed with a 200 OK")
            display("the asset at '{}' returned status code '{}' with payload '{}'", url, status, err)
        }
        /// For when the server returned an asset that was 200 but couldn't be serialized properly. This is the server's fault, and
        /// should generate a 500 status code at presentation.
        AssetSerFailed(url: String, err: String) {
            description("the asset couldn't be properly serialized")
            display("the asset at '{}' was successfully fetched, but couldn't be serialized with error '{}'", url, err)
        }
        /// For when the user requested an unsupported locale. This should generate a 404 at presentation.
        LocaleNotSupported(locale: String) {
            description("the given locale is not supported")
            display("the locale '{}' is not supported", locale)
        }

        /// For when a necessary template feautre was expected but not present. This just pertains to rendering strategies, and shouldn't
        /// ever be sensitive.
        TemplateFeatureNotEnabled(name: String, feature: String) {
            description("a template feature required by a function called was not present")
            display("the template '{}' is missing the feature '{}'", name, feature)
        }
        /// For when the given path wasn't found, a 404 should never be sensitive.
        PageNotFound(path: String) {
            description("the requested page was not found")
            display("the requested page at path '{}' was not found", path)
        }
        /// For when the user misconfigured their revalidation length, which should be caught at build time, and hence shouldn't be
        /// sensitive.
        InvalidDatetimeIntervalIndicator(indicator: String) {
            description("invalid indicator in timestring")
            display("invalid indicator '{}' in timestring, must be one of: s, m, h, d, w, M, y", indicator)
        }
        /// For when a template defined both build and request states when it can't amalgamate them sensibly, which indicates a misconfiguration.
        /// Revealing the rendering strategies of a template in this way should never be sensitive. Due to the execution context, this
        /// doesn't disclose the offending template.
        BothStatesDefined {
            description("both build and request states were defined for a template when only one or fewer were expected")
            display("both build and request states were defined for a template when only one or fewer were expected")
        }
        /// For when a render function failed. Only request-time functions can generate errors that will be transmitted over the network,
        /// so **render functions must not disclose sensitive information in errors**. Other information shouldn't be sensitive.
        RenderFnFailed(fn_name: String, template: String, cause: ErrorCause, err_str: String) {
            description("error while calling render function")
            display("an error caused by '{:?}' occurred while calling render function '{}' on template '{}': '{}'", cause, fn_name, template, err_str)
        }
    }
    links {
        ConfigManager(crate::config_manager::Error, crate::config_manager::ErrorKind);
        TranslationsManager(crate::translations_manager::Error, crate::translations_manager::ErrorKind);
        Translator(crate::translator::Error, crate::translator::ErrorKind);
    }
    // We work with many external libraries, all of which have their own errors
    foreign_links {
        Io(::std::io::Error);
        Json(::serde_json::Error);
        ChronoParse(::chrono::ParseError);
    }
}

pub fn err_to_status_code(err: &Error) -> u16 {
    match err.kind() {
        // Misconfiguration
        ErrorKind::TemplateFeatureNotEnabled(_, _) => 500,
        // Bad request
        ErrorKind::PageNotFound(_) => 404,
        // Misconfiguration
        ErrorKind::InvalidDatetimeIntervalIndicator(_) => 500,
        // Misconfiguration
        ErrorKind::BothStatesDefined => 500,
        // Ambiguous, we'll rely on the given cause
        ErrorKind::RenderFnFailed(_, _, cause, _) => match cause {
            ErrorCause::Client(code) => code.unwrap_or(400),
            ErrorCause::Server(code) => code.unwrap_or(500),
        },
        // We shouldn't be generating JS errors on the server...
        ErrorKind::JsErr(_) => {
            panic!("function 'err_to_status_code' is only intended for server-side usage")
        }
        // These are nearly always server-induced
        ErrorKind::ConfigManager(_) => 500,
        ErrorKind::Io(_) => 500,
        ErrorKind::ChronoParse(_) => 500,
        // JSON errors can be caused by the client, but we don't have enough information
        ErrorKind::Json(_) => 500,
        // Any other errors go to a 500
        _ => 500,
    }
}
