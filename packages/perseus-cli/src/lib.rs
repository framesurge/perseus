mod help;
mod prepare;
pub mod errors;
mod build;
mod serve;
mod cmd;

mod extraction;

use errors::*;
use std::fs;
use std::path::PathBuf;

pub const PERSEUS_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use help::help;
pub use prepare::{prepare, check_env};
pub use build::build;
pub use serve::serve;

/// Deletes a corrupted '.perseus/' directory. This qwill be called on certain error types that would leave the user with a half-finished
/// product, which is better to delete for safety and sanity.
pub fn delete_bad_dir(dir: PathBuf) -> Result<()> {
    let mut target = dir;
    target.extend([".perseus"]);
    // We'll only delete the directory if it exists, otherwise we're fine
    if target.exists() {
        if let Err(err) = fs::remove_dir_all(&target) {
            bail!(ErrorKind::RemoveBadDirFailed(target.to_str().map(|s| s.to_string()), err.to_string()))
        }
    }
    Ok(())
}
