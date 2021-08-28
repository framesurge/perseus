use std::path::PathBuf;
use crate::errors::*;

/// Serves the user's app. If no arguments are provided, this will build in watch mode and serve. If `-p/--prod` is specified, we'll
/// build for development, and if `--no-build` is specified, we won't build at all (useful for pseudo-production serving).
/// General message though: do NOT use the CLI for production serving!
pub fn serve(dir: PathBuf, prog_args: &[String]) -> Result<()> {
    todo!("serve command")
}
