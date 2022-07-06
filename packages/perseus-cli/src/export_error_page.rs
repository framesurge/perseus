use crate::cmd::run_cmd_directly;
use crate::errors::ExecutionError;
use crate::parse::ExportErrorPageOpts;
use std::env;
use std::path::PathBuf;

/// Exports a single error page for the given HTTP status code to the given
/// location.
pub fn export_error_page(dir: PathBuf, opts: ExportErrorPageOpts) -> Result<i32, ExecutionError> {
    run_cmd_directly(
        format!(
            "{} run {} -- {} {}",
            env::var("PERSEUS_CARGO_PATH").unwrap_or_else(|_| "cargo".to_string()),
            env::var("PERSEUS_CARGO_ARGS").unwrap_or_else(|_| String::new()),
            // These are mandatory
            opts.code,
            opts.output,
        ),
        &dir,
        vec![("PERSEUS_ENGINE_OPERATION", "export_error_page")],
    )
}
