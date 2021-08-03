#![allow(missing_docs)]

pub use error_chain::bail;
use error_chain::error_chain;

// The `error_chain` setup for the whole crate
error_chain! {
    // The custom errors for this crate (very broad)
    errors {
        /// For indistinct JavaScript errors.
        JsErr(err: String) {
            description("an error occurred while interfacing with javascript")
            display("the following error occurred while interfacing with javascript: {:?}", err)
        }

        TemplateFeatureNotEnabled(name: String, feature: String) {
            description("a template feature required by a function called was not present")
            display("the template '{}' is missing the feature '{}'", name, feature)
        }
        PageNotFound(path: String) {
            description("the requested page was not found")
            display("the requested page at path '{}' was not found", path)
        }
        NoRenderOpts(template_path: String) {
            description("a template had no rendering options for use at request-time")
            display("the template '{}' had no rendering options for use at request-time", template_path)
        }
        InvalidDatetimeIntervalIndicator(indicator: String) {
            description("invalid indicator in timestring")
            display("invalid indicator '{}' in timestring, must be one of: s, m, h, d, w, M, y", indicator)
        }
        BothStatesDefined {
            description("both build and request states were defined for a template when only one or fewer were expected")
            display("both build and request states were defined for a template when only one or fewer were expected")
        }
    }
    links {
        ConfigManager(crate::config_manager::Error, crate::config_manager::ErrorKind);
    }
    // We work with many external libraries, all of which have their own errors
    foreign_links {
        Io(::std::io::Error);
        Json(::serde_json::Error);
        ChronoParse(::chrono::ParseError);
    }
}
