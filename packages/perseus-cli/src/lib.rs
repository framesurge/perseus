#![doc = include_str!("../../../README.md")]
/*!
## Packages

This is the API documentation for the `perseus-cli` package, which acts as a frontend for abstracting away a lot of Perseus' internal complexity. Note that Perseus mostly uses
[the book](https://arctic-hen7.github.io/perseus/en-US) for documentation, and this should mostly be used as a secondary reference source. You can also find full usage examples
[here](https://github.com/arctic-hen7/perseus/tree/main/examples).

## Why is this here?

Usually, binary packages wouldn't have API documentation like this, but the Perseus CLI uses a hybrid structure of a library and a binary, which allows it to be used as a library in applications
that build on Perseus. Note that this area of using Perseus is currently almost entirely undocumented, and there may be major hiccups! If you'd like to help us out, please [open a PR](https://github.com/arctic-hen7/pulls/new) for
the documentation you'd like to see on this front!
*/

#![deny(missing_docs)]

mod build;
mod cmd;
mod deploy;
mod eject;
pub mod errors;
mod export;
mod export_error_page;
mod extraction;
/// Parsing utilities for arguments.
pub mod parse;
mod prepare;
mod reload_server;
mod serve;
mod serve_exported;
mod snoop;
mod thread;
mod tinker;

use errors::*;
use std::fs;
use std::path::PathBuf;

/// The current version of the CLI, extracted from the crate version.
pub const PERSEUS_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use build::build;
pub use deploy::deploy;
pub use eject::{eject, has_ejected};
pub use export::export;
pub use export_error_page::export_error_page;
pub use prepare::{check_env, prepare};
pub use reload_server::{order_reload, run_reload_server};
pub use serve::serve;
pub use serve_exported::serve_exported;
pub use snoop::{snoop_build, snoop_server, snoop_wasm_build};
pub use tinker::tinker;

/// Deletes a corrupted '.perseus/' directory. This will be called on certain error types that would leave the user with a half-finished
/// product, which is better to delete for safety and sanity.
pub fn delete_bad_dir(dir: PathBuf) -> Result<(), PrepError> {
    let mut target = dir;
    target.extend([".perseus"]);
    // We'll only delete the directory if it exists, otherwise we're fine
    if target.exists() {
        if let Err(err) = fs::remove_dir_all(&target) {
            return Err(PrepError::RemoveBadDirFailed { source: err });
        }
    }
    Ok(())
}

/// Deletes build artifacts in `.perseus/dist/static` or `.perseus/dist/pkg` and replaces the directory.
pub fn delete_artifacts(dir: PathBuf, dir_to_remove: &str) -> Result<(), ExecutionError> {
    let mut target = dir;
    target.extend([".perseus", "dist", dir_to_remove]);
    // We'll only delete the directory if it exists, otherwise we're fine
    if target.exists() {
        if let Err(err) = fs::remove_dir_all(&target) {
            return Err(ExecutionError::RemoveArtifactsFailed {
                target: target.to_str().map(|s| s.to_string()),
                source: err,
            });
        }
    }
    // No matter what, it's gone now, so recreate it
    // We also create parent directories because that's an issue for some reason in Docker (see #69)
    if let Err(err) = fs::create_dir_all(&target) {
        return Err(ExecutionError::RemoveArtifactsFailed {
            target: target.to_str().map(|s| s.to_string()),
            source: err,
        });
    }

    Ok(())
}
