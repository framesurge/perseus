#![allow(missing_docs)]
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("couldn't convert request from actix-web format to perseus format")]
    RequestConversionFailed {
        #[source]
        source: actix_web::client::HttpError,
    },
}
