use crate::cmd::run_cmd_directly;
use crate::parse::SnoopServeOpts;
use crate::{errors::*, get_user_crate_name};
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
            env::var("PERSEUS_CARGO_ENGINE_ARGS").unwrap_or_else(|_| String::new())
        ),
        &dir,
        vec![
            ("PERSEUS_ENGINE_OPERATION", "build"),
            ("CARGO_TARGET_DIR", "target_engine"),
        ],
    )
}

/// Runs the commands to build the user's app to Wasm directly so they can see
/// detailed logs.
pub fn snoop_wasm_build(dir: PathBuf) -> Result<i32, ExecutionError> {
    let crate_name = get_user_crate_name(&dir)?;

    println!("[NOTE]: You should expect unused code warnings here! Don't worry about them, they're just a product of the target-gating.");
    let exit_code = run_cmd_directly(
        format!(
            "{} build --target wasm32-unknown-unknown {}",
            env::var("PERSEUS_CARGO_PATH").unwrap_or_else(|_| "cargo".to_string()),
            env::var("PERSEUS_CARGO_BROWSER_ARGS").unwrap_or_else(|_| String::new())
        ),
        &dir,
        vec![("CARGO_TARGET_DIR", "target_wasm")],
    )?;
    if exit_code != 0 {
        return Ok(exit_code);
    }
    run_cmd_directly(
        format!(
            "{cmd} ./target_wasm/wasm32-unknown-unknown/debug/{crate_name}.wasm --out-dir dist/pkg --out-name perseus_engine --target web {args}",
            cmd=env::var("PERSEUS_WASM_BINDGEN_PATH").unwrap_or_else(|_| "wasm-bindgen".to_string()),
            args=env::var("PERSEUS_WASM_BINDGEN_ARGS").unwrap_or_else(|_| String::new()),
            crate_name=crate_name
        ),
        &dir,
        vec![("CARGO_TARGET_DIR", "target_wasm")],
    )
}

/// Runs the commands to run the server directly so the user can see detailed
/// logs.
pub fn snoop_server(dir: PathBuf, opts: SnoopServeOpts) -> Result<i32, ExecutionError> {
    run_cmd_directly(
        format!(
            "{} run {}",
            env::var("PERSEUS_CARGO_PATH").unwrap_or_else(|_| "cargo".to_string()),
            env::var("PERSEUS_CARGO_ENGINE_ARGS").unwrap_or_else(|_| String::new())
        ),
        &dir,
        vec![
            ("PERSEUS_ENGINE_OPERATION", "serve"),
            ("CARGO_TARGET_DIR", "target_engine"),
            ("PERSEUS_HOST", &opts.host),
            ("PERSEUS_PORT", &opts.port.to_string()),
        ], /* Unlike the `serve` command, we're both
            * building and running here, so we provide
            * the operation */
    )
}
