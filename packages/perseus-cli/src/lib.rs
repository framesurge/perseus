mod build;
mod cmd;
pub mod errors;
mod help;
mod prepare;
mod serve;

mod extraction;

use errors::*;
use std::fs;
use std::path::PathBuf;

pub const PERSEUS_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use build::build;
pub use help::help;
pub use prepare::{check_env, prepare};
pub use serve::serve;

/// Deletes a corrupted '.perseus/' directory. This qwill be called on certain error types that would leave the user with a half-finished
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
