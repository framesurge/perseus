use crate::cmd::{cfg_spinner, run_stage};
use crate::errors::*;
use crate::thread::{spawn_thread, ThreadHandle};
use console::{style, Emoji};
use indicatif::{MultiProgress, ProgressBar};
use std::env;
use std::path::PathBuf;

// Emojis for stages
static TINKERING: Emoji<'_, '_> = Emoji("🔧", ""); // TODO

/// Returns the exit code if it's non-zero.
macro_rules! handle_exit_code {
    ($code:expr) => {
        let (_, _, code) = $code;
        if code != 0 {
            return ::std::result::Result::Ok(code);
        }
    };
}

/// Actually tinkers the engione, program arguments having been interpreted. This needs to know how many steps there are in total
/// and takes a `MultiProgress` to interact with so it can be used truly atomically. This returns handles for waiting on the component
/// threads so we can use it composably.
#[allow(clippy::type_complexity)]
pub fn tinker_internal(
    dir: PathBuf,
    spinners: &MultiProgress,
    num_steps: u8,
) -> Result<
    ThreadHandle<impl FnOnce() -> Result<i32, ExecutionError>, Result<i32, ExecutionError>>,
    Error,
> {
    let target = dir.join(".perseus");

    // Tinkering message
    let tk_msg = format!(
        "{} {} Running plugin tinkers",
        style(format!("[1/{}]", num_steps)).bold().dim(),
        TINKERING
    );

    // We make sure to add them at the top (other spinners may have already been instantiated)
    let tk_spinner = spinners.insert(0, ProgressBar::new_spinner());
    let tk_spinner = cfg_spinner(tk_spinner, &tk_msg);
    let tk_target = target.clone();
    let tk_thread = spawn_thread(move || {
        handle_exit_code!(run_stage(
            vec![&format!(
                "{} run --bin perseus-tinker",
                env::var("PERSEUS_CARGO_PATH").unwrap_or_else(|_| "cargo".to_string()),
            )],
            &tk_target,
            &tk_spinner,
            &tk_msg
        )?);

        Ok(0)
    });

    Ok(tk_thread)
}

/// Runs plugin tinkers on the engine and returns an exit code. This doesn't have a release mode because tinkers should be applied in
/// development to work in both development and production.
pub fn tinker(dir: PathBuf) -> Result<i32, Error> {
    let spinners = MultiProgress::new();

    let tk_thread = tinker_internal(dir.clone(), &spinners, 1)?;
    let tk_res = tk_thread
        .join()
        .map_err(|_| ExecutionError::ThreadWaitFailed)??;
    if tk_res != 0 {
        return Ok(tk_res);
    }

    // We've handled errors in the component threads, so the exit code is now zero
    Ok(0)
}
