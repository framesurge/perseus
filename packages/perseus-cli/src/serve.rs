use crate::build::{build_internal, finalize};
use crate::cmd::{cfg_spinner, run_stage};
use crate::errors::*;
use crate::thread::{spawn_thread, ThreadHandle};
use console::{style, Emoji};
use indicatif::{MultiProgress, ProgressBar};
use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

// Emojis for stages
static BUILDING_SERVER: Emoji<'_, '_> = Emoji("📡", "");
static SERVING: Emoji<'_, '_> = Emoji("🛰️ ", "");

/// Returns the exit code if it's non-zero.
macro_rules! handle_exit_code {
    ($code:expr) => {{
        let (stdout, stderr, code) = $code;
        if code != 0 {
            return $crate::errors::Result::Ok(code);
        }
        (stdout, stderr)
    }};
}

/// Builds the server for the app, program arguments having been interpreted. This needs to know if we've built as part of this process
/// so it can show an accurate progress count. This also takes a `MultiProgress` so it can be used truly atomically (which will have
/// build spinners already on it if necessary). This also takes a `Mutex<String>` to inform the caller of the path of the server
/// executable.
fn build_server(
    dir: PathBuf,
    spinners: &MultiProgress,
    did_build: bool,
    exec: Arc<Mutex<String>>,
) -> Result<ThreadHandle<impl FnOnce() -> Result<i32>, Result<i32>>> {
    let num_steps = match did_build {
        true => 4,
        false => 2,
    };
    let target = dir.join(".perseus/server");

    // Server building message
    let sb_msg = format!(
        "{} {} Building server",
        style(format!("[{}/{}]", num_steps - 1, num_steps))
            .bold()
            .dim(),
        BUILDING_SERVER
    );

    // We'll parallelize the building of the server with any build commands that are currently running
    // We deliberately insert the spinner at the end of the list
    let sb_spinner = spinners.insert(num_steps - 1, ProgressBar::new_spinner());
    let sb_spinner = cfg_spinner(sb_spinner, &sb_msg);
    let sb_target = target.clone();
    let sb_thread = spawn_thread(move || {
        let (stdout, _stderr) = handle_exit_code!(run_stage(
            vec![&format!(
                // This sets Cargo to tell us everything, including the executable path to the server
                "{} build --message-format json",
                env::var("PERSEUS_CARGO_PATH").unwrap_or_else(|_| "cargo".to_string())
            )],
            &sb_target,
            &sb_spinner,
            &sb_msg
        )?);

        let msgs: Vec<&str> = stdout.trim().split('\n').collect();
        // If we got to here, the exit code was 0 and everything should've worked
        // The last message will just tell us that the build finished, the second-last one will tell us the executable path
        let msg = msgs.get(msgs.len() - 2);
        let msg = match msg {
            // We'll parse it as a Serde `Value`, we don't need to know everything that's in there
            Some(msg) => serde_json::from_str::<serde_json::Value>(msg)
                .map_err(|err| ErrorKind::GetServerExecutableFailed(err.to_string()))?,
            None => bail!(ErrorKind::GetServerExecutableFailed(
                "expected second-last message, none existed (too few messages)".to_string()
            )),
        };
        let server_exec_path = msg.get("executable");
        let server_exec_path = match server_exec_path {
            // We'll parse it as a Serde `Value`, we don't need to know everything that's in there
            Some(server_exec_path) => match server_exec_path.as_str() {
                Some(server_exec_path) => server_exec_path,
                None => bail!(ErrorKind::GetServerExecutableFailed(
                    "expected 'executable' field to be string".to_string()
                )),
            },
            None => bail!(ErrorKind::GetServerExecutableFailed(
                "expected 'executable' field in JSON map in second-last message, not present"
                    .to_string()
            )),
        };

        // And now the main thread needs to know about this
        let mut exec_val = exec.lock().unwrap();
        *exec_val = server_exec_path.to_string();

        Ok(0)
    });

    Ok(sb_thread)
}

/// Runs the server at the given path, handling any errors therewith. This will likely be a black hole until the user manually terminates
/// the process.
fn run_server(exec: Arc<Mutex<String>>, dir: PathBuf, did_build: bool) -> Result<i32> {
    let target = dir.join(".perseus/server");
    let num_steps = match did_build {
        true => 4,
        false => 2,
    };

    // First off, handle any issues with the executable path
    let exec_val = exec.lock().unwrap();
    if exec_val.is_empty() {
        bail!(ErrorKind::GetServerExecutableFailed(
            "mutex value empty, implies uncaught thread termination (please report this as a bug)"
                .to_string()
        ))
    }
    let server_exec_path = (*exec_val).to_string();

    // Manually run the generated binary (invoking in the right directory context for good measure if it ever needs it in future)
    let child = Command::new(&server_exec_path)
        .current_dir(target)
        // We should be able to access outputs in case there's an error
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| ErrorKind::CmdExecFailed(server_exec_path, err.to_string()))?;
    // Figure out what host/port the app will be live on
    let host = env::var("HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .map_err(|err| ErrorKind::PortNotNumber(err.to_string()))?;
    // Give the user a nice informational message
    println!(
        "  {} {} Your app is now live on http://{host}:{port}! To change this, re-run this command with different settings of the HOST/PORT environment variables.",
        style(format!("[{}/{}]", num_steps, num_steps)).bold().dim(),
        SERVING,
        host=host,
        port=port
    );

    // Wait on the child process to finish (which it shouldn't unless there's an error), then perform error handling
    let output = child.wait_with_output().unwrap();
    let exit_code = match output.status.code() {
        Some(exit_code) => exit_code,         // If we have an exit code, use it
        None if output.status.success() => 0, // If we don't, but we know the command succeeded, return 0 (success code)
        None => 1, // If we don't know an exit code but we know that the command failed, return 1 (general error code)
    };
    // Print `stderr` only if there's something therein and the exit code is non-zero
    if !output.stderr.is_empty() && exit_code != 0 {
        // We don't print any failure message other than the actual error right now (see if people want something else?)
        std::io::stderr().write_all(&output.stderr).unwrap();
        return Ok(1);
    }

    Ok(0)
}

/// Builds the subcrates to get a directory that we can serve and then serves it.
pub fn serve(dir: PathBuf, prog_args: &[String]) -> Result<i32> {
    let spinners = MultiProgress::new();
    // TODO support watching files
    let did_build = !prog_args.contains(&"--no-build".to_string());
    let should_run = !prog_args.contains(&"--no-run".to_string());
    // We need to have a way of knowing what the executable path to the server is
    let exec = Arc::new(Mutex::new(String::new()));
    // We can begin building the server in a thread without having to deal with the rest of the build stage yet
    let sb_thread = build_server(dir.clone(), &spinners, did_build, Arc::clone(&exec))?;
    // Only build if the user hasn't set `--no-build`, handling non-zero exit codes
    if did_build {
        let (sg_thread, wb_thread) = build_internal(dir.clone(), &spinners, 4)?;
        let sg_res = sg_thread
            .join()
            .map_err(|_| ErrorKind::ThreadWaitFailed)??;
        let wb_res = wb_thread
            .join()
            .map_err(|_| ErrorKind::ThreadWaitFailed)??;
        if sg_res != 0 {
            return Ok(sg_res);
        } else if wb_res != 0 {
            return Ok(wb_res);
        }
    }
    // Handle errors from the server building
    let sb_res = sb_thread
        .join()
        .map_err(|_| ErrorKind::ThreadWaitFailed)??;
    if sb_res != 0 {
        return Ok(sb_res);
    }

    // And now we can run the finalization stage (only if `--no-build` wasn't specified)
    if did_build {
        finalize(&dir.join(".perseus"))?;
    }

    // Now actually run that executable path if we should
    if should_run {
        let exit_code = run_server(Arc::clone(&exec), dir.clone(), did_build)?;
        Ok(exit_code)
    } else {
        // The user doesn't want to run the server, so we'll give them the executable path instead
        let exec_str: String = (*exec.lock().unwrap()).to_string();
        println!("Not running server because `--no-run` was provided. You can run it manually by running the following executable in `.perseus/server/`.\n{}", exec_str);
        Ok(0)
    }
}
