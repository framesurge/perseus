/*!
 * Perseus is a blazingly fast frontend web development framework built in Rust with support for major rendering strategies,
 * reactivity without a virtual DOM, and extreme customizability. It wraps the lower-level capabilities of[Sycamore](https://github.com/sycamore-rs/sycamore)
 * and provides a NextJS-like API!
 *
 * - ✨ Supports static generation (serving only static resources)
 * - ✨ Supports server-side rendering (serving dynamic resources)
 * - ✨ Supports revalidation after time and/or with custom logic (updating rendered pages)
 * - ✨ Supports incremental regeneration (build on demand)
 * - ✨ Open build matrix (use any rendering strategy with anything else, mostly)
 * - ✨ CLI harness that lets you build apps with ease and confidence
 *
 * This is the documentation for the Perseus CLI, but there's also [the core package](https://crates.io/crates/perseus) and [integrations](https://arctic-hen7.github.io/perseus/serving.html)
 * to make serving apps on other platforms easier!
 *
 * # Resources
 *
 * These docs will help you as a reference, but [the book](https://arctic-hen7.github.io/perseus/cli.html) should
 * be your first port of call for learning about how to use Perseus and how it works.
 *
 * - [The Book](https://arctic-hen7.github.io/perseus)
 * - [GitHub repository](https://github.com/arctic-hen7/perseus)
 * - [Crate page](https://crates.io/crates/perseus)
 * - [Gitter chat](https://gitter.im/perseus-framework/community)
 * - [Discord server channel](https://discord.com/channels/820400041332179004/883168134331256892) (for Sycamore-related stuff)
 */

#![deny(missing_docs)]

mod build;
mod cmd;
mod eject;
pub mod errors;
mod help;
mod prepare;
mod serve;
mod thread;

mod extraction;

use errors::*;
use std::fs;
use std::path::PathBuf;

/// The current version of the CLI, extracted from the crate version.
pub const PERSEUS_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use build::build;
pub use eject::{eject, has_ejected};
pub use help::help;
pub use prepare::{check_env, prepare};
pub use serve::serve;

/// Deletes a corrupted '.perseus/' directory. This will be called on certain error types that would leave the user with a half-finished
/// product, which is better to delete for safety and sanity.
pub fn delete_bad_dir(dir: PathBuf) -> Result<()> {
    let mut target = dir;
    target.extend([".perseus"]);
    // We'll only delete the directory if it exists, otherwise we're fine
    if target.exists() {
        if let Err(err) = fs::remove_dir_all(&target) {
            bail!(ErrorKind::RemoveBadDirFailed(
                target.to_str().map(|s| s.to_string()),
                err.to_string()
            ))
        }
    }
    Ok(())
}

/// Deletes build artifacts in `.perseus/dist/static` or `.perseus/dist/pkg` and replaces the directory.
pub fn delete_artifacts(dir: PathBuf, dir_to_remove: &str) -> Result<()> {
    let mut target = dir;
    target.extend([".perseus", "dist", dir_to_remove]);
    // We'll only delete the directory if it exists, otherwise we're fine
    if target.exists() {
        if let Err(err) = fs::remove_dir_all(&target) {
            bail!(ErrorKind::RemoveArtifactsFailed(
                target.to_str().map(|s| s.to_string()),
                err.to_string()
            ))
        }
    }
    // No matter what, it's gone now, so recreate it
    if let Err(err) = fs::create_dir(&target) {
        bail!(ErrorKind::RemoveArtifactsFailed(
            target.to_str().map(|s| s.to_string()),
            err.to_string()
        ))
    }

    Ok(())
}
