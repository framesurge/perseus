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

        PageFeatureNotEnabled(name: String, feature: String) {
            description("a page feature required by a function called was not present")
            display("the page '{}' is missing the feature '{}'", name, feature)
        }
    }
    links {
        ConfigManager(crate::config_manager::Error, crate::config_manager::ErrorKind);
    }
    // We work with many external libraries, all of which have their own errors
    foreign_links {
        Io(::std::io::Error);
        Json(::serde_json::Error);
    }
}
