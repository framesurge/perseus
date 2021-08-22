#![allow(missing_docs)]

pub use error_chain::bail;
use error_chain::error_chain;

// The `error_chain` setup for the whole crate
error_chain! {
    // The custom errors for this crate (very broad)
    errors {
        /// 
        JsErr(err: String) {
            description("an error occurred while interfacing with javascript")
            display("the following error occurred while interfacing with javascript: {:?}", err)
        }
		/// For if converting an HTTP request from Actix Web format to Perseus format failed.
		RequestConversionFailed(err: String) {
			description("converting the request from actix-web format to perseus format failed")
            display("converting the request from actix-web format to perseus format failed: {:?}", err)
		}
    }
    links {
        ConfigManager(::perseus::config_manager::Error, ::perseus::config_manager::ErrorKind);
    }
    // We work with many external libraries, all of which have their own errors
    foreign_links {
        Io(::std::io::Error);
        Json(::serde_json::Error);
    }
}