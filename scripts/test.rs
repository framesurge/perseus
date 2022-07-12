use std::process::Command;
use std::io::Write;

fn main() {
    let exit_code = real_main();
    std::process::exit(exit_code);
}

fn real_main() -> i32 {
    let args: Vec<String> = std::env::args().collect();
    // This is brittle, but it's only ever called from Bonnie
    let category = &args[1];
    let example = &args[2];
    let integration = &args[3];
    let is_headless = match args.get(4) {
        Some(arg) if arg == "--headless" => true,
        _ => false
    };

    println!("Running browser-requiring Wasm tests for example '{}' in category '{}' with integration '{}'. This script will only print an output if an error occurs, if no output is printed then all tests were successful.", &example, &category, &integration);

    // Build the server executable for testing and get its path
    #[cfg(unix)]
    let shell_exec = "sh";
    #[cfg(windows)]
    let shell_exec = "powershell";
    #[cfg(unix)]
    let shell_param = "-c";
    #[cfg(windows)]
    let shell_param = "-command";

    let exec_name = {
        // This is intended to be run from the root of the project (which this script always will be because of Bonnie's requirements)
        let output = Command::new(shell_exec)
            .args([shell_param, &format!(
                "bonnie dev example {category} {example} test --no-run",
                category=&category,
                example=&example,
            )])
            .env("EXAMPLE_INTEGRATION", &integration)
            .output()
            .expect("couldn't build tests (command execution failed)");
        let exit_code = match output.status.code() {
            Some(exit_code) => exit_code,         // If we have an exit code, use it
            None if output.status.success() => 0, // If we don't, but we know the command succeeded, return 0 (success code)
            None => 1, // If we don't know an exit code but we know that the command failed, return 1 (general error code)
        };
        // Print `stderr` and `stdout` only if there's something therein and the exit code is non-zero
        if !output.stderr.is_empty() && exit_code != 0 {
            std::io::stderr().write_all(&output.stdout).unwrap();
            std::io::stderr().write_all(&output.stderr).unwrap();
        }
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();

        // Extract the last line of that (the executable name)
        stdout.lines().last().expect("couldn't get server executable (the build failed)").trim().to_string()
    };

    // Run the server from that executable in the background
    // This has to be run from the correct execution context (inside the root of the target example)
    let mut server = Command::new(exec_name)
        .current_dir(&format!("examples/{}/{}", &category, &example))
        // Tell the server we're in testing mode
        .env("PERSEUS_TESTING", "true")
        // We're in dev mode, so we have to tell the binary what to do
        .env("PERSEUS_ENGINE_OPERATION", "serve")
        .spawn()
        .expect("couldn't start test server (command execution failed)");

    // Check if this example is locked to a single integration (in which case we shouldn't apply the user's integration setting)
    let integration_locked = std::fs::metadata(format!("examples/{}/{}/.integration_locked", &category, &example)).is_ok();
    let cargo_features = if integration_locked {
        println!("The given example is locked to a specific integration. The provided integration may not be used.");
        String::new()
    } else {
        format!("--features 'perseus-integration/{}'", &integration)
    };

    // We want to get an exit code from actually running the tests (which interact with the server we're running in the background), which we'll return from this process
    let exit_code = {
        let output = Command::new(shell_exec)
            .current_dir(&format!("examples/{}/{}", &category, &example))
            // TODO Confirm that this syntax works on Windows
            .args([shell_param, &format!("cargo test {} -- --test-threads 1", &cargo_features)])
            .envs(if is_headless {
                vec![("PERSEUS_RUN_WASM_TESTS", "true"), ("PERSEUS_RUN_WASM_TESTS_HEADLESS", "true")]
            } else {
                vec![("PERSEUS_RUN_WASM_TESTS", "true")]
            })
            .output()
            .expect("couldn't run tests (command execution failed)");
        let exit_code = match output.status.code() {
            Some(exit_code) => exit_code,         // If we have an exit code, use it
            None if output.status.success() => 0, // If we don't, but we know the command succeeded, return 0 (success code)
            None => 1, // If we don't know an exit code but we know that the command failed, return 1 (general error code)
        };
        // Print `stderr` and `stdout` only if there's something therein and the exit code is non-zero
        if !output.stderr.is_empty() && exit_code != 0 {
            std::io::stderr().write_all(&output.stdout).unwrap();
            std::io::stderr().write_all(&output.stderr).unwrap();
        }

        exit_code
    };

    // Terminate the server
    let res = server.kill();
    // If that returned an error, the server had already stopped, which doesn't make much sense
    if let Err(_) = res {
        eprintln!("[WARNING]: Couldn't terminate background server, it had already terminated. This shouldn't happen, and may be indicative of a broader issue.");
    }

    exit_code
}
