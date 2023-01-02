use crate::cmd::run_cmd_directly;
use crate::install::Tools;
use crate::parse::{Opts, SnoopServeOpts};
use crate::{errors::*, get_user_crate_name};
use std::path::PathBuf;

/// Runs static generation processes directly so the user can see detailed logs.
/// This is commonly used for allowing users to see `dbg!` and the like in their
/// builder functions.
pub fn snoop_build(dir: PathBuf, tools: &Tools, global_opts: &Opts) -> Result<i32, ExecutionError> {
    run_cmd_directly(
        format!(
            "{} run {}",
            tools.cargo_engine, global_opts.cargo_engine_args
        ),
        &dir,
        vec![
            ("PERSEUS_ENGINE_OPERATION", "build"),
            ("CARGO_TARGET_DIR", "dist/target_engine"),
            ("RUSTFLAGS", "--cfg=engine"),
        ],
    )
}

/// Runs the commands to build the user's app to Wasm directly so they can see
/// detailed logs. This can't be used for release builds, so we don't have to
/// worry about `wasm-opt`.
pub fn snoop_wasm_build(
    dir: PathBuf,
    tools: &Tools,
    global_opts: &Opts,
) -> Result<i32, ExecutionError> {
    let crate_name = get_user_crate_name(&dir)?;

    println!("[NOTE]: You should expect unused code warnings here! Don't worry about them, they're just a product of the target-gating.");
    let exit_code = run_cmd_directly(
        format!(
            "{} build --target wasm32-unknown-unknown {}",
            tools.cargo_browser, global_opts.cargo_browser_args
        ),
        &dir,
        vec![
            ("CARGO_TARGET_DIR", "dist/target_wasm"),
            ("RUSTFLAGS", "--cfg=client"),
        ],
    )?;
    if exit_code != 0 {
        return Ok(exit_code);
    }
    run_cmd_directly(
        format!(
            "{cmd} ./dist/target_wasm/wasm32-unknown-unknown/debug/{crate_name}.wasm --out-dir dist/pkg --out-name perseus_engine --target web {args}",
            cmd=tools.wasm_bindgen,
            args=global_opts.wasm_bindgen_args,
            crate_name=crate_name
        ),
        &dir,
        vec![("CARGO_TARGET_DIR", "dist/target_wasm")],
    )
}

/// Runs the commands to run the server directly so the user can see detailed
/// logs.
pub fn snoop_server(
    dir: PathBuf,
    opts: &SnoopServeOpts,
    tools: &Tools,
    global_opts: &Opts,
) -> Result<i32, ExecutionError> {
    run_cmd_directly(
        format!(
            "{} run {}",
            tools.cargo_engine, global_opts.cargo_engine_args
        ),
        &dir,
        vec![
            ("PERSEUS_ENGINE_OPERATION", "serve"),
            ("CARGO_TARGET_DIR", "dist/target_engine"),
            ("PERSEUS_HOST", &opts.host),
            ("PERSEUS_PORT", &opts.port.to_string()),
            ("RUSTFLAGS", "--cfg=engine"),
        ], /* Unlike the `serve` command, we're both
            * building and running here, so we provide
            * the operation */
    )
}
