#![doc = include_str!("../README.proj.md")]
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
pub mod errors;
mod export;
mod export_error_page;
mod init;
mod install;
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
use std::path::PathBuf;
use std::{fs, path::Path};

/// The current version of the CLI, extracted from the crate version.
pub const PERSEUS_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use build::build;
pub use deploy::deploy;
pub use export::export;
pub use export_error_page::export_error_page;
pub use init::{init, new};
pub use install::{get_tools_dir, Tools};
pub use prepare::check_env;
pub use reload_server::{order_reload, run_reload_server};
pub use serve::serve;
pub use serve_exported::serve_exported;
pub use snoop::{snoop_build, snoop_server, snoop_wasm_build};
pub use tinker::tinker;

/// Creates the `dist/` directory in the project root, which is necessary
/// for Cargo to be able to put its build artifacts in there.
pub fn create_dist(dir: &Path) -> Result<(), ExecutionError> {
    let target = dir.join("dist");
    if !target.exists() {
        fs::create_dir(target).map_err(|err| ExecutionError::CreateDistFailed { source: err })?;
    }
    Ok(())
}

/// Deletes the entire `dist/` directory. Notably, this is where we keep
/// several Cargo artifacts, so this means the next build will be much
/// slower.
pub fn delete_dist(dir: PathBuf) -> Result<(), ExecutionError> {
    let target = dir.join("dist");
    if target.exists() {
        if let Err(err) = fs::remove_dir_all(&target) {
            return Err(ExecutionError::RemoveArtifactsFailed {
                target: target.to_str().map(|s| s.to_string()),
                source: err,
            });
        }
    }

    Ok(())
}

/// Deletes build artifacts in `dist/static` or `dist/pkg` and replaces the
/// directory.
pub fn delete_artifacts(dir: PathBuf, dir_to_remove: &str) -> Result<(), ExecutionError> {
    let mut target = dir;
    target.extend(["dist", dir_to_remove]);
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
    // We also create parent directories because that's an issue for some reason in
    // Docker (see #69)
    if let Err(err) = fs::create_dir_all(&target) {
        return Err(ExecutionError::RemoveArtifactsFailed {
            target: target.to_str().map(|s| s.to_string()),
            source: err,
        });
    }

    Ok(())
}

/// Gets the name of the user's crate from their `Cargo.toml` (assumed to be in
/// the root of the given directory).
pub fn get_user_crate_name(dir: &Path) -> Result<String, ExecutionError> {
    let manifest = cargo_toml::Manifest::from_path(dir.join("Cargo.toml"))
        .map_err(|err| ExecutionError::GetManifestFailed { source: err })?;
    let name = manifest
        .package
        .ok_or(ExecutionError::CrateNameNotPresentInManifest)?
        .name;
    Ok(name)
}
