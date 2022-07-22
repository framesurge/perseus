use crate::cmd::run_cmd_directly;
use crate::errors::ExecutionError;
use crate::install::Tools;
use crate::parse::{ExportErrorPageOpts, Opts};
use std::path::PathBuf;

/// Exports a single error page for the given HTTP status code to the given
/// location.
pub fn export_error_page(
    dir: PathBuf,
    opts: &ExportErrorPageOpts,
    tools: &Tools,
    global_opts: &Opts,
) -> Result<i32, ExecutionError> {
    run_cmd_directly(
        format!(
            "{} run {} -- {} {}",
            tools.cargo_engine,
            global_opts.cargo_engine_args,
            // These are mandatory
            opts.code,
            opts.output,
        ),
        &dir,
        vec![
            ("PERSEUS_ENGINE_OPERATION", "export_error_page"),
            ("CARGO_TARGET_DIR", "dist/target_engine"),
        ],
    )
}
