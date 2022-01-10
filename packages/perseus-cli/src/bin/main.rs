use clap::Parser;
use command_group::stdlib::CommandGroup;
use fmterr::fmt_err;
use notify::{recommended_watcher, RecursiveMode, Watcher};
use perseus_cli::parse::{ExportOpts, ServeOpts, SnoopSubcommand};
use perseus_cli::{
    build, check_env, delete_artifacts, delete_bad_dir, deploy, eject, export, has_ejected,
    parse::{Opts, Subcommand},
    prepare, serve, serve_exported, tinker,
};
use perseus_cli::{errors::*, export_error_page, snoop_build, snoop_server, snoop_wasm_build};
use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::channel;

// All this does is run the program and terminate with the acquired exit code
#[tokio::main]
async fn main() {
    // In development, we'll test in the `basic` example
    if cfg!(debug_assertions) {
        let example_to_test =
            env::var("TEST_EXAMPLE").unwrap_or_else(|_| "../../examples/basic".to_string());
        env::set_current_dir(example_to_test).unwrap();
    }
    let exit_code = real_main().await;
    std::process::exit(exit_code)
}

// IDEA Watch files at the `core()` level and then panic, catching the unwind in the watcher loop

// This manages error handling and returns a definite exit code to terminate with
async fn real_main() -> i32 {
    // Get the working directory
    let dir = env::current_dir();
    let dir = match dir {
        Ok(dir) => dir,
        Err(err) => {
            eprintln!(
                "{}",
                fmt_err(&PrepError::CurrentDirUnavailable { source: err })
            );
            return 1;
        }
    };
    let res = core(dir.clone()).await;
    match res {
        // If it worked, we pass the executed command's exit code through
        Ok(exit_code) => exit_code,
        // If something failed, we print the error to `stderr` and return a failure exit code
        Err(err) => {
            let should_cause_deletion = err_should_cause_deletion(&err);
            eprintln!("{}", fmt_err(&err));
            // Check if the error needs us to delete a partially-formed '.perseus/' directory
            if should_cause_deletion {
                if let Err(err) = delete_bad_dir(dir) {
                    eprintln!("{}", fmt_err(&err));
                }
            }
            1
        }
    }
}

// This is used internally for message passing
enum Event {
    // Sent if we should restart the child process
    Reload,
    // Sent if we should temrinate the child process
    Terminate,
}

// This performs the actual logic, separated for deduplication of error handling and destructor control
// This returns the exit code of the executed command, which we should return from the process itself
// This prints warnings using the `writeln!` macro, which allows the parsing of `stdout` in production or a vector in testing
// If at any point a warning can't be printed, the program will panic
async fn core(dir: PathBuf) -> Result<i32, Error> {
    // Get `stdout` so we can write warnings appropriately
    let stdout = &mut std::io::stdout();

    // Warn the user if they're using the CLI single-threaded mode
    if env::var("PERSEUS_CLI_SEQUENTIAL").is_ok() {
        writeln!(stdout, "Note: the Perseus CLI is running in single-threaded mode, which is less performant on most modern systems. You can switch to multi-threaded mode by unsetting the 'PERSEUS_CLI_SEQUENTIAL' environment variable. If you've deliberately enabled single-threaded mode, you can safely ignore this.\n").expect("Failed to write to stdout.");
    }

    // Parse the CLI options with `clap`
    let opts: Opts = Opts::parse();
    // Check the user's environment to make sure they have prerequisites
    // We do this after any help pages or version numbers have been parsed for snappiness
    check_env()?;

    // Check if this process is allowed to watch for changes
    // This will be set to `true` if this is a child process
    // The CLI will actually spawn another version of itself if we're watching for changes
    // The reason for this is to avoid having to manage handlers for multiple threads and other child processes
    // After several days of attempting, this is the only feasible solution (short of a full rewrite of the CLI)
    let watch_allowed = env::var("PERSEUS_WATCHING_PROHIBITED").is_err();
    // Check if the user wants to watch for changes
    match &opts.subcmd {
        Subcommand::Export(ExportOpts { watch, .. })
        | Subcommand::Serve(ServeOpts { watch, .. })
            if *watch && watch_allowed =>
        {
            let (tx_term, rx) = channel();
            let tx_fs = tx_term.clone();
            // Set the handler for termination events (more than just SIGINT) on all platforms
            // We do this before anything else so that, if it fails, we don't have servers left open
            ctrlc::set_handler(move || {
                tx_term
                    .send(Event::Terminate)
                    .expect("couldn't shut down child processes (servers may have been left open)")
            })
            .expect("couldn't set handlers to gracefully terminate process");

            // Find out where this binary is
            // SECURITY: If the CLI were installed with root privileges, it would be possible to create a hard link to the
            // binary, execute through that, and then replace it with a malicious binary before we got here which would
            // allow privilege escalation. See https://vulners.com/securityvulns/SECURITYVULNS:DOC:22183.
            // TODO Drop root privileges at startup
            let bin_name =
                env::current_exe().map_err(|err| WatchError::GetSelfPathFailed { source: err })?;
            // Get the arguments to provide
            // These are the same, but we'll disallow watching with an environment variable
            let mut args = env::args().collect::<Vec<String>>();
            // We'll remove the first element of the arguments (binary name, but less reliable)
            args.remove(0);

            // Set up a watcher
            let mut watcher = recommended_watcher(move |_| {
                // If this fails, the watcher channel was completely disconnected, which should never happen (it's in a loop)
                tx_fs.send(Event::Reload).unwrap();
            })
            .map_err(|err| WatchError::WatcherSetupFailed { source: err })?;
            // Watch the current directory
            for entry in std::fs::read_dir(".")
                .map_err(|err| WatchError::ReadCurrentDirFailed { source: err })?
            {
                // We want to exclude `target/` and `.perseus/`, otherwise we should watch everything
                let entry = entry.map_err(|err| WatchError::ReadDirEntryFailed { source: err })?;
                let name = entry.file_name();
                if name != "target" && name != ".perseus" && name != ".git" {
                    watcher
                        .watch(&entry.path(), RecursiveMode::Recursive)
                        .map_err(|err| WatchError::WatchFileFailed {
                            filename: entry.path().to_str().unwrap().to_string(),
                            source: err,
                        })?;
                }
            }

            // This will store the handle to the child process
            // This will be updated every time we re-create the process
            // We spawn it as a process group, whcih means signals go to grandchild processes as well, which means hot reloading
            // can actually work!
            let mut child = Command::new(&bin_name)
                .args(&args)
                .env("PERSEUS_WATCHING_PROHIBITED", "true")
                .group_spawn()
                .map_err(|err| WatchError::SpawnSelfFailed { source: err })?;

            let res = loop {
                match rx.recv() {
                    Ok(Event::Reload) => {
                        // Kill the current child process
                        // This will return an error if the child has already exited, which is fine
                        // This gracefully kills the process in the sense that it kills it and all its children
                        let _ = child.kill();
                        // Restart it
                        child = Command::new(&bin_name)
                            .args(&args)
                            .env("PERSEUS_WATCHING_PROHIBITED", "true")
                            .group_spawn()
                            .map_err(|err| WatchError::SpawnSelfFailed { source: err })?;
                    }
                    Ok(Event::Terminate) => {
                        // This means the user is trying to stop the process
                        // We have to manually terminate the process group, because it's a process *group*
                        let _ = child.kill();
                        // From here, we can let the prgoram terminate naturally
                        break Ok(0);
                    }
                    Err(err) => break Err(WatchError::WatcherError { source: err }),
                }
            };
            let exit_code = res?;
            Ok(exit_code)
        }
        // If not, just run the central logic normally
        _ => core_watch(dir, opts).await,
    }
}

async fn core_watch(dir: PathBuf, opts: Opts) -> Result<i32, Error> {
    // If we're not cleaning up artifacts, create them if needed
    if !matches!(opts.subcmd, Subcommand::Clean(_)) {
        prepare(dir.clone())?;
    }
    let exit_code = match opts.subcmd {
        Subcommand::Build(build_opts) => {
            // Delete old build artifacts
            delete_artifacts(dir.clone(), "static")?;
            build(dir, build_opts)?
        }
        Subcommand::Export(export_opts) => {
            // Delete old build/export artifacts
            delete_artifacts(dir.clone(), "static")?;
            delete_artifacts(dir.clone(), "exported")?;
            let exit_code = export(dir.clone(), export_opts.clone())?;
            if exit_code != 0 {
                return Ok(exit_code);
            }
            if export_opts.serve {
                serve_exported(dir, export_opts.host, export_opts.port).await;
            }
            0
        }
        Subcommand::Serve(serve_opts) => {
            if !serve_opts.no_build {
                delete_artifacts(dir.clone(), "static")?;
            }
            let (exit_code, _server_path) = serve(dir, serve_opts)?;
            exit_code
        }
        Subcommand::Test(test_opts) => {
            // This will be used by the subcrates
            env::set_var("PERSEUS_TESTING", "true");
            // Set up the '.perseus/' directory if needed
            prepare(dir.clone())?;
            // Delete old build artifacts if `--no-build` wasn't specified
            if !test_opts.no_build {
                delete_artifacts(dir.clone(), "static")?;
            }
            let (exit_code, _server_path) = serve(dir, test_opts)?;
            exit_code
        }
        Subcommand::Clean(clean_opts) => {
            if clean_opts.dist {
                // The user only wants to remove distribution artifacts
                // We don't delete `render_conf.json` because it's literally impossible for that to be the source of a problem right now
                delete_artifacts(dir.clone(), "static")?;
                delete_artifacts(dir.clone(), "pkg")?;
                delete_artifacts(dir, "exported")?;
            } else {
                // This command deletes the `.perseus/` directory completely, which musn't happen if the user has ejected
                if has_ejected(dir.clone()) && !clean_opts.force {
                    return Err(EjectionError::CleanAfterEject.into());
                }
                // Just delete the '.perseus/' directory directly, as we'd do in a corruption
                delete_bad_dir(dir)?;
            }
            0
        }
        Subcommand::Deploy(deploy_opts) => {
            delete_artifacts(dir.clone(), "static")?;
            delete_artifacts(dir.clone(), "exported")?;
            delete_artifacts(dir.clone(), "pkg")?;
            deploy(dir, deploy_opts)?
        }
        Subcommand::Eject => {
            eject(dir)?;
            0
        }
        Subcommand::Tinker(tinker_opts) => {
            // We shouldn't run arbitrary plugin code designed to alter the engine if the user has made their own changes after ejecting
            if has_ejected(dir.clone()) && !tinker_opts.force {
                return Err(EjectionError::TinkerAfterEject.into());
            }
            // Unless we've been told not to, we start with a blank slate
            // This will remove old tinkerings and eliminate any possible corruptions (which are very likely with tinkering!)
            if !tinker_opts.no_clean {
                delete_bad_dir(dir.clone())?;
                // Recreate the '.perseus/' directory
                prepare(dir.clone())?;
            }
            tinker(dir)?
        }
        Subcommand::Snoop(snoop_subcmd) => match snoop_subcmd {
            SnoopSubcommand::Build => snoop_build(dir)?,
            SnoopSubcommand::WasmBuild(snoop_wasm_opts) => snoop_wasm_build(dir, snoop_wasm_opts)?,
            SnoopSubcommand::Serve(snoop_serve_opts) => snoop_server(dir, snoop_serve_opts)?,
        },
        Subcommand::ExportErrorPage(opts) => export_error_page(dir, opts)?,
        Subcommand::Prep => {
            // The `.perseus/` directory has already been set up in the preliminaries, so we don't need to do anything here
            0
        }
    };
    Ok(exit_code)
}
