use crate::errors::*;
use console::Emoji;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

// Some useful emojis
pub static SUCCESS: Emoji<'_, '_> = Emoji("✅", "success!");
pub static FAILURE: Emoji<'_, '_> = Emoji("❌", "failed!");

/// Runs the given command conveniently, returning the exit code. Notably, this parses the given command by separating it on spaces.
/// Returns the command's output and the exit code.
pub fn run_cmd(
    cmd: String,
    dir: &Path,
    envs: Vec<(&str, &str)>,
    pre_dump: impl Fn(),
) -> Result<(String, String, i32), ExecutionError> {
    // We run the command in a shell so that NPM/Yarn binaries can be recognized (see #5)
    #[cfg(unix)]
    let shell_exec = "sh";
    #[cfg(windows)]
    let shell_exec = "powershell";
    #[cfg(unix)]
    let shell_param = "-c";
    #[cfg(windows)]
    let shell_param = "-command";

    // This will NOT pipe output/errors to the console
    let output = Command::new(shell_exec)
        .args([shell_param, &cmd])
        .envs(envs)
        .current_dir(dir)
        .output()
        .map_err(|err| ExecutionError::CmdExecFailed { cmd, source: err })?;

    let exit_code = match output.status.code() {
        Some(exit_code) => exit_code,         // If we have an exit code, use it
        None if output.status.success() => 0, // If we don't, but we know the command succeeded, return 0 (success code)
        None => 1, // If we don't know an exit code but we know that the command failed, return 1 (general error code)
    };

    // Print `stderr` and `stdout` only if there's something therein and the exit code is non-zero
    // If we only print `stderr`, we can miss some things (see #74)
    if !output.stderr.is_empty() && exit_code != 0 {
        pre_dump();
        std::io::stderr().write_all(&output.stdout).unwrap();
        std::io::stderr().write_all(&output.stderr).unwrap();
    }

    Ok((
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        exit_code,
    ))
}

/// Creates a new spinner.
pub fn cfg_spinner(spinner: ProgressBar, message: &str) -> ProgressBar {
    spinner.set_style(ProgressStyle::default_spinner().tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "));
    spinner.set_message(format!("{}...", &message));
    // Tick the spinner every 50 milliseconds
    spinner.enable_steady_tick(50);

    spinner
}
/// Instructs the given spinner to show success.
pub fn succeed_spinner(spinner: &ProgressBar, message: &str) {
    spinner.finish_with_message(format!("{}...{}", message, SUCCESS));
}
/// Instructs the given spinner to show failure.
pub fn fail_spinner(spinner: &ProgressBar, message: &str) {
    spinner.finish_with_message(format!("{}...{}", message, FAILURE));
}

/// Runs a series of commands. Returns the last command's output and an appropriate exit code (0 if everything worked, otherwise th
/// exit code of the first one that failed). This also takes a `Spinner` to use and control.
pub fn run_stage(
    cmds: Vec<&str>,
    target: &Path,
    spinner: &ProgressBar,
    message: &str,
    envs: Vec<(&str, &str)>,
) -> Result<(String, String, i32), ExecutionError> {
    let mut last_output = (String::new(), String::new());
    // Run the commands
    for cmd in cmds {
        // We make sure all commands run in the target directory ('.perseus/' itself)
        let (stdout, stderr, exit_code) = run_cmd(cmd.to_string(), target, envs.to_vec(), || {
            // This stage has failed
            fail_spinner(spinner, message);
        })?;
        last_output = (stdout, stderr);
        // If we have a non-zero exit code, we should NOT continue (stderr has been written to the console already)
        if exit_code != 0 {
            return Ok((last_output.0, last_output.1, 1));
        }
    }

    // Everything has worked for this stage
    succeed_spinner(spinner, message);

    Ok((last_output.0, last_output.1, 0))
}

/// Runs a command directly, piping its output and errors to the streams of this program. This allows the user to investigate the innards of
/// Perseus, or just see their own `dbg!` calls. This will return the exit code of the command, which should be passed through to this program.
pub fn run_cmd_directly(
    cmd: String,
    dir: &Path,
    envs: Vec<(&str, &str)>,
) -> Result<i32, ExecutionError> {
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
        .envs(envs)
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
