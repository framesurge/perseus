#![doc = include_str!("../README.proj.md")]
/*!
## Packages

This is the API documentation for the `perseus-actix-web` package, which allows Perseus apps to run on Actix Web. Note that Perseus mostly uses [the book](https://arctic-hen7.github.io/perseus/en-US) for
documentation, and this should mostly be used as a secondary reference source. You can also find full usage examples [here](https://github.com/arctic-hen7/perseus/tree/main/examples).
*/

#![deny(missing_docs)]

mod configurer;
mod conv_req;
#[cfg(feature = "dflt-server")]
mod dflt_server;
pub mod errors;
mod initial_load;
mod page_data;
mod translations;

pub use crate::configurer::configurer;
#[cfg(feature = "dflt-server")]
pub use dflt_server::dflt_server;
pub use perseus::internal::serve::ServerOptions;
