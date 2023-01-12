use crate::cmd::{cfg_spinner, run_stage};
use crate::install::Tools;
use crate::parse::{Opts, ServeOpts, TestOpts};
use crate::thread::spawn_thread;
use crate::{errors::*, serve};
use console::{style, Emoji};
use indicatif::{MultiProgress, ProgressBar};
use std::path::PathBuf;
use std::process::{Command, Stdio};

// Emoji for stages
static TESTING: Emoji<'_, '_> = Emoji("🧪", "");

/// Returns the exit code if it's non-zero.
macro_rules! handle_exit_code {
    ($code:expr) => {
        let (_, _, code) = $code;
        if code != 0 {
            return ::std::result::Result::Ok(code);
        }
    };
}

/// Tests the user's app by creating a testing server and running `cargo test`
/// against it, which will presumably use a WebDriver of some kind.
pub fn test(
    dir: PathBuf,
    test_opts: &TestOpts,
    tools: &Tools,
    global_opts: &Opts,
) -> Result<i32, ExecutionError> {
    // We need to own this for the threads
    let tools = tools.clone();
    let Opts {
        cargo_engine_path,
        cargo_engine_args,
        ..
    } = global_opts.clone();

    let serve_opts = ServeOpts {
        // We want to run the binary while we run `cargo test` at the same time
        no_run: true,
        no_build: test_opts.no_build,
        release: false,
        standalone: false,
        watch: test_opts.watch,
        custom_watch: test_opts.custom_watch.clone(),
        host: test_opts.host.clone(),
        port: test_opts.port,
    };
    let num_steps: u8 = if test_opts.no_build { 2 } else { 4 };
    // This will do all sorts of things with spinners etc., but we've told it we're
    // testing, so things will be neater
    let spinners = MultiProgress::new();
    let (exit_code, server_path) = serve(
        dir.clone(),
        &serve_opts,
        &tools,
        global_opts,
        &spinners,
        true,
    )?;
    if exit_code != 0 {
        return Ok(exit_code);
    }
    if let Some(server_path) = server_path {
        // Building is complete and we have a path to run the server, so we'll now do
        // that with a child process (doesn't need to be in a separate thread, since
        // it's long-running)
        let mut server = Command::new(&server_path)
            .envs([
                ("PERSEUS_ENGINE_OPERATION", "serve"),
                ("PERSEUS_TESTING", "true"),
            ])
            .current_dir(&dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| ExecutionError::CmdExecFailed {
                cmd: server_path,
                source: err,
            })?;

        // Now run the Cargo tests against that
        let test_msg = format!(
            "{} {} Running tests",
            style(format!("[{}/{}]", num_steps, num_steps)).bold().dim(),
            TESTING,
        );
        let test_spinner = spinners.insert(num_steps.into(), ProgressBar::new_spinner());
        let test_spinner = cfg_spinner(test_spinner, &test_msg);
        let test_dir = dir;
        let headless = !test_opts.show_browser;
        let test_thread = spawn_thread(
            move || {
                handle_exit_code!(run_stage(
                    vec![&format!(
                        // We use single-threaded testing, because most webdrivers don't support
                        // multithreaded testing yet
                        "{} test {} -- --test-threads 1",
                        cargo_engine_path, cargo_engine_args
                    )],
                    &test_dir,
                    &test_spinner,
                    &test_msg,
                    if headless {
                        vec![
                            ("CARGO_TARGET_DIR", "dist/target_engine"),
                            ("RUSTFLAGS", "--cfg=engine"),
                            ("CARGO_TERM_COLOR", "always"),
                            ("PERSEUS_RUN_WASM_TESTS", "true"),
                            ("PERSEUS_RUN_WASM_TESTS_HEADLESS", "true"),
                        ]
                    } else {
                        vec![
                            ("CARGO_TARGET_DIR", "dist/target_engine"),
                            ("RUSTFLAGS", "--cfg=engine"),
                            ("CARGO_TERM_COLOR", "always"),
                            ("PERSEUS_RUN_WASM_TESTS", "true"),
                        ]
                    }
                )?);

                Ok(0)
            },
            // See above
            false,
        );

        let test_res = test_thread
            .join()
            .map_err(|_| ExecutionError::ThreadWaitFailed)??;
        if test_res != 0 {
            return Ok(test_res);
        }

        // If the server has already terminated, it had an error, and that would be
        // reflected in the tests
        let _ = server.kill();

        // If the server is still running, that's fine, because it will be terminated
        // with the main thread (or the thread group, in the case of watching)
        //
        // TODO Figure out a way of getting a green tick on that step

        // We've handled errors in the component threads, so the exit code is now zero
        Ok(0)
    } else {
        Err(ExecutionError::GetServerExecutableFailedSimple.into())
    }
}
