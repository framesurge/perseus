#![doc = include_str!("../README.proj.md")]
/*!
## Packages

This is the API documentation for the `perseus-axum` package, which allows Perseus apps to run on Axum. Note that Perseus mostly uses [the book](https://arctic-hen7.github.io/perseus/en-US) for
documentation, and this should mostly be used as a secondary reference source. You can also find full usage examples [here](https://github.com/arctic-hen7/perseus/tree/main/examples).
*/

#![deny(missing_docs)]

// This integration doesn't need to convert request types, because we can get
// them straight out of Axum and then just delete the bodies
#[cfg(feature = "dflt-server")]
mod dflt_server;
mod router;

pub use crate::router::get_router;
#[cfg(feature = "dflt-server")]
pub use dflt_server::dflt_server;
pub use perseus::server::ServerOptions;
