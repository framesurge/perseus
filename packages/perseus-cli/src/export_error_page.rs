use crate::cmd::run_cmd_directly;
use crate::errors::ExecutionError;
use crate::parse::ExportErrorPageOpts;
use std::env;
use std::path::PathBuf;

/// Exports a single error page for the given HTTP status code to the given location.
pub fn export_error_page(dir: PathBuf, opts: ExportErrorPageOpts) -> Result<i32, ExecutionError> {
    let target = dir.join(".perseus/builder");
    run_cmd_directly(
        format!(
            "{} run --bin perseus-error-page-exporter {} {}",
            env::var("PERSEUS_CARGO_PATH").unwrap_or_else(|_| "cargo".to_string()),
            // These are mandatory
            opts.code,
            opts.output
        ),
        &target,
    )
}
