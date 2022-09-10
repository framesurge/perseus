use crate::cmd::{cfg_spinner, run_stage};
use crate::install::Tools;
use crate::parse::{BuildOpts, Opts};
use crate::thread::{spawn_thread, ThreadHandle};
use crate::{errors::*, get_user_crate_name};
use console::{style, Emoji};
use indicatif::{MultiProgress, ProgressBar};
use std::path::PathBuf;

// Emoji for stages
static GENERATING: Emoji<'_, '_> = Emoji("🔨", "");
static BUILDING: Emoji<'_, '_> = Emoji("🏗️ ", ""); // Yes, there's a space here, for some reason it's needed...

/// Returns the exit code if it's non-zero.
macro_rules! handle_exit_code {
    ($code:expr) => {
        let (_, _, code) = $code;
        if code != 0 {
            return ::std::result::Result::Ok(code);
        }
    };
}

/// Actually builds the user's code, program arguments having been interpreted.
/// This needs to know how many steps there are in total because the serving
/// logic also uses it. This also takes a `MultiProgress` to interact with so it
/// can be used truly atomically. This returns handles for waiting on the
/// component threads so we can use it composably.
#[allow(clippy::type_complexity)]
pub fn build_internal(
    dir: PathBuf,
    spinners: &MultiProgress,
    num_steps: u8,
    is_release: bool,
    tools: &Tools,
    global_opts: &Opts,
) -> Result<
    (
        ThreadHandle<impl FnOnce() -> Result<i32, ExecutionError>, Result<i32, ExecutionError>>,
        ThreadHandle<impl FnOnce() -> Result<i32, ExecutionError>, Result<i32, ExecutionError>>,
    ),
    ExecutionError,
> {
    // We need to own this for the threads
    let tools = tools.clone();
    let Opts {
        wasm_release_rustflags,
        cargo_engine_args,
        cargo_browser_args,
        wasm_bindgen_args,
        wasm_opt_args,
        ..
    } = global_opts.clone();

    let crate_name = get_user_crate_name(&dir)?;
    // Static generation message
    let sg_msg = format!(
        "{} {} Generating your app",
        style(format!("[1/{}]", num_steps)).bold().dim(),
        GENERATING
    );
    // Wasm building message
    let wb_msg = format!(
        "{} {} Building your app to Wasm",
        style(format!("[2/{}]", num_steps)).bold().dim(),
        BUILDING
    );

    // We parallelize the first two spinners (static generation and Wasm building)
    // We make sure to add them at the top (the server spinner may have already been
    // instantiated)
    let sg_spinner = spinners.insert(0, ProgressBar::new_spinner());
    let sg_spinner = cfg_spinner(sg_spinner, &sg_msg);
    let sg_dir = dir.clone();
    let wb_spinner = spinners.insert(1, ProgressBar::new_spinner());
    let wb_spinner = cfg_spinner(wb_spinner, &wb_msg);
    let wb_dir = dir;
    let cargo_engine_exec = tools.cargo_engine.clone();
    let sg_thread = spawn_thread(
        move || {
            handle_exit_code!(run_stage(
                vec![&format!(
                    "{} run {} {}",
                    cargo_engine_exec,
                    if is_release { "--release" } else { "" },
                    cargo_engine_args
                )],
                &sg_dir,
                &sg_spinner,
                &sg_msg,
                vec![
                    ("PERSEUS_ENGINE_OPERATION", "build"),
                    ("CARGO_TARGET_DIR", "dist/target_engine")
                ]
            )?);

            Ok(0)
        },
        global_opts.sequential,
    );
    let wb_thread = spawn_thread(
        move || {
            let mut cmds = vec![
            // Build the Wasm artifact first (and we know where it will end up, since we're setting the target directory)
            format!(
                "{} build --target wasm32-unknown-unknown {} {}",
                tools.cargo_browser,
                if is_release { "--release" } else { "" },
                cargo_browser_args
            ),
            // NOTE The `wasm-bindgen` version has to be *identical* to the dependency version
            format!(
                "{cmd} ./dist/target_wasm/wasm32-unknown-unknown/{profile}/{crate_name}.wasm --out-dir dist/pkg --out-name perseus_engine --target web {args}",
                cmd=tools.wasm_bindgen,
                profile={ if is_release { "release" } else { "debug" } },
                args=wasm_bindgen_args,
                crate_name=crate_name
            )
        ];
            // If we're building for release, then we should run `wasm-opt`
            if is_release {
                cmds.push(format!(
                "{cmd} -Oz ./dist/pkg/perseus_engine_bg.wasm -o ./dist/pkg/perseus_engine_bg.wasm {args}",
                cmd=tools.wasm_opt,
                args=wasm_opt_args
            ));
            }
            let cmds = cmds.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
            handle_exit_code!(run_stage(
                cmds,
                &wb_dir,
                &wb_spinner,
                &wb_msg,
                if is_release {
                    vec![
                        ("CARGO_TARGET_DIR", "dist/target_wasm"),
                        ("RUSTFLAGS", &wasm_release_rustflags),
                    ]
                } else {
                    vec![("CARGO_TARGET_DIR", "dist/target_wasm")]
                }
            )?);

            Ok(0)
        },
        global_opts.sequential,
    );

    Ok((sg_thread, wb_thread))
}

/// Builds the subcrates to get a directory that we can serve. Returns an exit
/// code.
pub fn build(
    dir: PathBuf,
    opts: &BuildOpts,
    tools: &Tools,
    global_opts: &Opts,
) -> Result<i32, ExecutionError> {
    let spinners = MultiProgress::new();

    let (sg_thread, wb_thread) =
        build_internal(dir, &spinners, 2, opts.release, tools, global_opts)?;
    let sg_res = sg_thread
        .join()
        .map_err(|_| ExecutionError::ThreadWaitFailed)??;
    if sg_res != 0 {
        return Ok(sg_res);
    }
    let wb_res = wb_thread
        .join()
        .map_err(|_| ExecutionError::ThreadWaitFailed)??;
    if wb_res != 0 {
        return Ok(wb_res);
    }

    // We've handled errors in the component threads, so the exit code is now zero
    Ok(0)
}
