use crate::cmd::run_cmd_directly;
use crate::errors::*;
use crate::parse::{SnoopServeOpts, SnoopWasmOpts};
use std::env;
use std::path::PathBuf;

/// Runs static generation processes directly so the user can see detailed logs.
/// This is commonly used for allowing users to see `dbg!` and the like in their
/// builder functions.
pub fn snoop_build(dir: PathBuf) -> Result<i32, ExecutionError> {
    run_cmd_directly(
        format!(
            "{} run {}",
            env::var("PERSEUS_CARGO_PATH").unwrap_or_else(|_| "cargo".to_string()),
            env::var("PERSEUS_CARGO_ARGS").unwrap_or_else(|_| String::new())
        ),
        &dir,
        vec![("PERSEUS_ENGINE_OPERATION", "build")],
    )
}

/// Runs the commands to build the user's app to Wasm directly so they can see
/// detailed logs.
pub fn snoop_wasm_build(dir: PathBuf, opts: SnoopWasmOpts) -> Result<i32, ExecutionError> {
    run_cmd_directly(
        format!(
            "{} build --out-dir dist/pkg --out-name perseus_engine --target web {} {}",
            env::var("PERSEUS_WASM_PACK_PATH").unwrap_or_else(|_| "wasm-pack".to_string()),
            if opts.profiling {
                "--profiling"
            } else {
                "--dev"
            },
            env::var("PERSEUS_WASM_PACK_ARGS").unwrap_or_else(|_| String::new())
        ),
        &dir,
        vec![],
    )
}

/// Runs the commands to run the server directly so the user can see detailed
/// logs.
pub fn snoop_server(dir: PathBuf, opts: SnoopServeOpts) -> Result<i32, ExecutionError> {
    // Set the environment variables for the host and port
    env::set_var("PERSEUS_HOST", opts.host);
    env::set_var("PERSEUS_PORT", opts.port.to_string());

    run_cmd_directly(
        format!(
            "{} run {}",
            env::var("PERSEUS_CARGO_PATH").unwrap_or_else(|_| "cargo".to_string()),
            env::var("PERSEUS_CARGO_ARGS").unwrap_or_else(|_| String::new())
        ),
        &dir,
        vec![("PERSEUS_ENGINE_OPERATION", "serve")], /* Unlike the `serve` command, we're both
                                                      * building and running here, so we provide
                                                      * the operation */
    )
}
