use crate::{
    cmd::{cfg_spinner, run_stage},
    errors::*,
    parse::{CheckOpts, Opts},
    thread::{spawn_thread, ThreadHandle},
    Tools,
};
use console::{style, Emoji};
use indicatif::{MultiProgress, ProgressBar};
use std::path::PathBuf;

// Emoji for stages
static CHECKING_ENGINE: Emoji<'_, '_> = Emoji("🦾", "");
static CHECKING_BROWSER: Emoji<'_, '_> = Emoji("🌐", "");
static GENERATING: Emoji<'_, '_> = Emoji("🔨", "");

/// Returns the exit code if it's non-zero.
macro_rules! handle_exit_code {
    ($code:expr) => {
        let (_, _, code) = $code;
        if code != 0 {
            return ::std::result::Result::Ok(code);
        }
    };
}

/// Checks the user's app by checking their code and building it. This will
/// first run `cargo check`, and then `cargo check --target
/// wasm32-unknown-unknown`, so we can error quickly on compilation errors.
/// If those both succeed, then we'll actually try to generate build artifacts,
/// which is the only other place a Perseus can reasonably fail at build-time.
pub fn check(
    dir: PathBuf,
    check_opts: &CheckOpts,
    tools: &Tools,
    global_opts: &Opts,
) -> Result<i32, ExecutionError> {
    // First, run `cargo check`
    let spinners = MultiProgress::new();
    let num_steps = if check_opts.generate { 3 } else { 2 };

    let (engine_thread, browser_thread) =
        cargo_check(dir.clone(), &spinners, num_steps, tools, global_opts)?;
    let engine_res = engine_thread
        .join()
        .map_err(|_| ExecutionError::ThreadWaitFailed)??;
    if engine_res != 0 {
        return Ok(engine_res);
    }
    let browser_res = browser_thread
        .join()
        .map_err(|_| ExecutionError::ThreadWaitFailed)??;
    if browser_res != 0 {
        return Ok(browser_res);
    }

    // If that worked, generate static artifacts (if we've been told to)
    if check_opts.generate {
        let generation_res =
            run_static_generation(dir, &MultiProgress::new(), num_steps, tools, global_opts)?;
        Ok(generation_res)
    } else {
        Ok(0)
    }
}

/// Runs `cargo check` for both the engine-side and the browser-side
/// simultaneously, returning handles to the underlying threads. This will
/// create progress bars as appropriate.
#[allow(clippy::type_complexity)]
fn cargo_check(
    dir: PathBuf,
    spinners: &MultiProgress,
    num_steps: u8,
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
        cargo_engine_args,
        cargo_browser_args,
        ..
    } = global_opts.clone();

    let engine_msg = format!(
        "{} {} Checking your app's engine-side",
        style(format!("[1/{}]", num_steps)).bold().dim(),
        CHECKING_ENGINE
    );
    let browser_msg = format!(
        "{} {} Checking your app's browser-side",
        style(format!("[2/{}]", num_steps)).bold().dim(),
        CHECKING_BROWSER
    );

    // We parallelize the first two spinners
    let engine_spinner = spinners.insert(0, ProgressBar::new_spinner());
    let engine_spinner = cfg_spinner(engine_spinner, &engine_msg);
    let engine_dir = dir.clone();
    let browser_spinner = spinners.insert(1, ProgressBar::new_spinner());
    let browser_spinner = cfg_spinner(browser_spinner, &browser_msg);
    let browser_dir = dir;
    let cargo_engine_exec = tools.cargo_engine.clone();
    let engine_thread = spawn_thread(
        move || {
            handle_exit_code!(run_stage(
                vec![&format!(
                    "{} check {}",
                    cargo_engine_exec, cargo_engine_args
                )],
                &engine_dir,
                &engine_spinner,
                &engine_msg,
                vec![
                    // We still need this for checking, because otherwise we can't check the engine
                    // and the browser simultaneously (different targets, so no
                    // commonalities gained by one directory)
                    ("CARGO_TARGET_DIR", "dist/target_engine"),
                    ("RUSTFLAGS", "--cfg=engine")
                ]
            )?);

            Ok(0)
        },
        global_opts.sequential,
    );
    let browser_thread = spawn_thread(
        move || {
            handle_exit_code!(run_stage(
                vec![&format!(
                    "{} check --target wasm32-unknown-unknown {}",
                    tools.cargo_browser, cargo_browser_args
                )],
                &browser_dir,
                &browser_spinner,
                &browser_msg,
                vec![
                    ("CARGO_TARGET_DIR", "dist/target_wasm"),
                    ("RUSTFLAGS", "--cfg=client")
                ]
            )?);

            Ok(0)
        },
        global_opts.sequential,
    );

    Ok((engine_thread, browser_thread))
}

#[allow(clippy::type_complexity)]
fn run_static_generation(
    dir: PathBuf,
    spinners: &MultiProgress,
    num_steps: u8,
    tools: &Tools,
    global_opts: &Opts,
) -> Result<i32, ExecutionError> {
    let Opts {
        cargo_engine_args, ..
    } = global_opts.clone();

    let msg = format!(
        "{} {} Checking your app's page generation",
        style(format!("[3/{}]", num_steps)).bold().dim(),
        GENERATING
    );

    // We parallelize the first two spinners
    let spinner = spinners.insert(0, ProgressBar::new_spinner());
    let spinner = cfg_spinner(spinner, &msg);

    handle_exit_code!(run_stage(
        vec![&format!("{} run {}", tools.cargo_engine, cargo_engine_args)],
        &dir,
        &spinner,
        &msg,
        vec![
            ("PERSEUS_ENGINE_OPERATION", "build"),
            ("CARGO_TARGET_DIR", "dist/target_engine"),
            ("RUSTFLAGS", "--cfg=engine")
        ]
    )?);

    Ok(0)
}
