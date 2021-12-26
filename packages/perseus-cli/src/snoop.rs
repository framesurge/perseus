use crate::errors::*;
use crate::parse::SnoopServeOpts;
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Runs a command directly, piping its output and errors to the streams of this program. This allows the user to investigate the innards of
/// Perseus, or just see their own `dbg!` calls. This will return the exit code of the command, which should be passed through to this program.
fn run_cmd_directly(cmd: String, dir: &Path) -> Result<i32, ExecutionError> {
    // The shell configurations for Windows and Unix
    #[cfg(unix)]
    let shell_exec = "sh";
    #[cfg(windows)]
    let shell_exec = "powershell";
    #[cfg(unix)]
    let shell_param = "-c";
    #[cfg(windows)]
    let shell_param = "-command";

    let output = Command::new(shell_exec)
        .args([shell_param, &cmd])
        .current_dir(dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|err| ExecutionError::CmdExecFailed { cmd, source: err })?;

    let exit_code = match output.status.code() {
        Some(exit_code) => exit_code,         // If we have an exit code, use it
        None if output.status.success() => 0, // If we don't, but we know the command succeeded, return 0 (success code)
        None => 1, // If we don't know an exit code but we know that the command failed, return 1 (general error code)
    };

    Ok(exit_code)
}

/// Runs static generation processes directly so the user can see detailed logs. This is commonly used for allowing users to see `dbg!` and
/// the like in their builder functions.
pub fn snoop_build(dir: PathBuf) -> Result<i32, ExecutionError> {
    let target = dir.join(".perseus/builder");
    run_cmd_directly(
        format!(
            "{} run",
            env::var("PERSEUS_CARGO_PATH").unwrap_or_else(|_| "cargo".to_string())
        ),
        &target,
    )
}

/// Runs the commands to build the user's app to Wasm directly so they can see detailed logs.
pub fn snoop_wasm_build(dir: PathBuf) -> Result<i32, ExecutionError> {
    let target = dir.join(".perseus");
    run_cmd_directly(
        format!(
            "{} build --target web --dev",
            env::var("PERSEUS_WASM_PACK_PATH").unwrap_or_else(|_| "wasm-pack".to_string())
        ),
        &target,
    )
}

/// Runs the commands to run the server directly so the user can see detailed logs.
pub fn snoop_server(dir: PathBuf, opts: SnoopServeOpts) -> Result<i32, ExecutionError> {
    let target = dir.join(".perseus/server");
    run_cmd_directly(
        format!(
            "{} run {}",
            env::var("PERSEUS_CARGO_PATH").unwrap_or_else(|_| "cargo".to_string()),
            // Enable the appropriate feature for a non-default server integration
            format!(
                "--features integration-{} --no-default-features",
                opts.integration.to_string()
            )
        ),
        &target,
    )
}
