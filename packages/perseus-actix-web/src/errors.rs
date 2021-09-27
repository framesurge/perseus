#![allow(missing_docs)]

pub use perseus::errors::format_err;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("couldn't convert request from actix-web format to perseus format")]
    RequestConversionFailed {
        #[source]
        source: actix_web::client::HttpError,
    },
}
