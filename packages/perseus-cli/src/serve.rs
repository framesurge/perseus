use crate::build::build_internal;
use crate::cmd::run_stage;
use crate::errors::*;
use console::{style, Emoji};
use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

// Emojis for stages
static BUILDING_SERVER: Emoji<'_, '_> = Emoji("📡", "");
static SERVING: Emoji<'_, '_> = Emoji("🛰️ ", "");

/// Returns the exit code if it's non-zero.
macro_rules! handle_exit_code {
    ($code:expr) => {{
        let (stdout, stderr, code) = $code;
        if code != 0 {
            return Ok(code);
        }
        (stdout, stderr)
    }};
}

/// Actually serves the user's app, program arguments having been interpreted. This needs to know if we've built as part of this process
/// so it can show an accurate progress count.
fn serve_internal(dir: PathBuf, did_build: bool) -> Result<i32> {
    let num_steps = match did_build {
        true => 5,
        false => 2,
    };
    let mut target = dir;
    // All the serving work can be done in the `server` subcrate after building is finished
    target.extend([".perseus", "server"]);

    // Build the server runner
    // We use the JSON message format so we can get extra info about the generated executable
    let (stdout, _stderr) = handle_exit_code!(run_stage(
        vec![
            &format!("{} build --message-format json", env::var("PERSEUS_CARGO_PATH").unwrap_or_else(|_| "cargo".to_string()))
        ],
        &target,
        format!(
            "{} {} Building server",
            style(format!("[{}/{}]", num_steps - 1, num_steps))
                .bold()
                .dim(),
            BUILDING_SERVER
        )
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

    // Manually run the generated binary (invoking in the right directory context for good measure if it ever needs it in future)
    let child = Command::new(server_exec_path)
        .current_dir(target)
        // We should be able to access outputs in case there's an error
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| ErrorKind::CmdExecFailed(server_exec_path.to_string(), err.to_string()))?;
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

/// Builds the subcrates to get a directory that we can serve. Returns an exit code.
pub fn serve(dir: PathBuf, prog_args: &[String]) -> Result<i32> {
    // TODO support watching files
    let mut did_build = false;
    // Only build if the user hasn't set `--no-build`, handling non-zero exit codes
    if !prog_args.contains(&"--no-build".to_string()) {
        did_build = true;
        let build_exit_code = build_internal(dir.clone(), 4)?;
        if build_exit_code != 0 {
            return Ok(build_exit_code);
        }
    }
    // Now actually serve the user's data
    let exit_code = serve_internal(dir.clone(), did_build)?;

    Ok(exit_code)
}
