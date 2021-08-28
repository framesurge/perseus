use std::env;
use std::io::Write;
use std::path::PathBuf;
use lib::{PERSEUS_VERSION, help, check_env, prepare, delete_bad_dir, build, serve};
use lib::errors::*;

// All this does is run the program and terminate with the acquired exit code
fn main() {
    // In development, we'll test in the basic example
    if cfg!(debug_assertions) {
        env::set_current_dir("../../examples/basic").unwrap();
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
            let err = ErrorKind::CurrentDirUnavailable(err.to_string());
            eprintln!("{}", err);
            return 1
        }
    };
    let res = core(dir.clone());
    match res {
        // If it worked, we pass the executed command's exit code through
        Ok(exit_code) => exit_code,
        // If something failed, we print the error to `stderr` and return a failure exit code
        Err(err) => {
            eprintln!("{}", err);
            // Check if the error needs us to delete a partially-formed '.perseus/' directory
            if err_should_cause_deletion(&err) {
                if let Err(err) = delete_bad_dir(dir) {
                    eprintln!("{}", err);
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
fn core(dir: PathBuf) -> Result<i32> {
    // Get `stdout` so we can write warnings appropriately
    let stdout = &mut std::io::stdout();
    // Get the arguments to this program, removing the first one (something like `perseus`)
    let mut prog_args: Vec<String> = env::args().collect();
    // This will panic if the first argument is not found (which is probably someone trying to fuzz us)
    let _executable_name = prog_args.remove(0);
    // Check the user's environment to make sure they have prerequisites
    check_env()?;
    // Check for special arguments
    if matches!(prog_args.get(0), Some(_)) {
        if prog_args[0] == "-v" || prog_args[0] == "--version" {
            writeln!(stdout, "You are currently running the Perseus CLI v{}! You can see the latest release at https://github.com/arctic-hen7/perseus/releases.", PERSEUS_VERSION).expect("Failed to write version.");
            Ok(0)
        } else if prog_args[0] == "-h" || prog_args[0] == "--help" {
            help(stdout);
            Ok(0)
		} else {
            // Now we can check commands
		    if prog_args[0] == "build" {
                // Set up the '.perseus/' directory if needed
                prepare(dir.clone())?;
		    	build(dir, &prog_args)?;
                Ok(0)
		    } else if prog_args[0] == "serve" {
                // Set up the '.perseus/' directory if needed
                prepare(dir.clone())?;
		    	serve(dir, &prog_args)?;
                Ok(0)
            } else if prog_args[0] == "clean" {
		    	// Just delete the '.perseus/' directory directly, as we'd do in a corruption
                delete_bad_dir(dir)?;
                Ok(0)
            } else {
                writeln!(stdout, "Unknown command '{}'. You can see the help page with -h/--help.", prog_args[0]);
			    Ok(1)
            }
		}
    } else {
		writeln!(stdout, "Please provide a command to run, or use -h/--help to see the help page.");
		Ok(1)
	}
}
