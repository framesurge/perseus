use clap::Clap;
use perseus_cli::errors::*;
use perseus_cli::{
    build, check_env, delete_artifacts, delete_bad_dir, eject, export, has_ejected,
    parse::{Opts, Subcommand},
    prepare, report_err, serve,
};
use std::env;
use std::io::Write;
use std::path::PathBuf;

// All this does is run the program and terminate with the acquired exit code
fn main() {
    // In development, we'll test in the `basic` example
    if cfg!(debug_assertions) {
        let example_to_test =
            env::var("TEST_EXAMPLE").unwrap_or_else(|_| "../../examples/basic".to_string());
        env::set_current_dir(example_to_test).unwrap();
    }
    let exit_code = real_main();
    std::process::exit(exit_code)
}

// This manages error handling and returns a definite exit code to terminate with
fn real_main() -> i32 {
    // Get the working directory
    let dir = env::current_dir();
    let dir = match dir {
        Ok(dir) => dir,
        Err(err) => {
            report_err!(PrepError::CurrentDirUnavailable { source: err });
            return 1;
        }
    };
    let res = core(dir.clone());
    match res {
        // If it worked, we pass the executed command's exit code through
        Ok(exit_code) => exit_code,
        // If something failed, we print the error to `stderr` and return a failure exit code
        Err(err) => {
            let should_cause_deletion = err_should_cause_deletion(&err);
            report_err!(err);
            // Check if the error needs us to delete a partially-formed '.perseus/' directory
            if should_cause_deletion {
                if let Err(err) = delete_bad_dir(dir) {
                    report_err!(err);
                }
            }
            1
        }
    }
}

// This performs the actual logic, separated for deduplication of error handling and destructor control
// This returns the exit code of the executed command, which we should return from the process itself
// This prints warnings using the `writeln!` macro, which allows the parsing of `stdout` in production or a vector in testing
// If at any point a warning can't be printed, the program will panic
fn core(dir: PathBuf) -> Result<i32, Error> {
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
            // Delete old build/exportation artifacts
            delete_artifacts(dir.clone(), "static")?;
            delete_artifacts(dir.clone(), "exported")?;
            export(dir, export_opts)?
        }
        Subcommand::Serve(serve_opts) => {
            // Delete old build artifacts if `--no-build` wasn't specified
            if !serve_opts.no_build {
                delete_artifacts(dir.clone(), "static")?;
            }
            serve(dir, serve_opts)?
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
            serve(dir, test_opts)?
        }
        Subcommand::Clean(clean_opts) => {
            if clean_opts.dist {
                // The user only wants to remove distribution artifacts
                // We don't delete `render_conf.json` because it's literally impossible for that to be the source of a problem right now
                delete_artifacts(dir.clone(), "static")?;
                delete_artifacts(dir, "pkg")?;
            } else {
                // This command deletes the `.perseus/` directory completely, which musn't happen if the user has ejected
                if has_ejected(dir.clone()) && clean_opts.force {
                    return Err(EjectionError::CleanAfterEject.into());
                }
                // Just delete the '.perseus/' directory directly, as we'd do in a corruption
                delete_bad_dir(dir)?;
            }
            0
        }
        Subcommand::Eject => {
            eject(dir)?;
            0
        }
        Subcommand::Prep => {
            // The `.perseus/` directory has already been set up in the preliminaries, so we don't need to do anything here
            0
        }
    };
    Ok(exit_code)
}
