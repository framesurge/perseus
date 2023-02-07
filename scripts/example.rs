// This script is intended to run a single example
// This is broken out from Bonnie so we can check if the example is using `perseus-integration`, and therefore if it should be run with
// a specific integration flag (setting this on one that doesn't use it would lead to an error)

use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let mut args: Vec<String> = std::env::args().collect();
    // This is brittle, but it's only ever called from Bonnie
    // The remaining args are sent to the CLI
    let _ = args.remove(0);
    let category = args.remove(0);
    let example = args.remove(0);
    let integration = args.remove(0);

    // Check if there's a file in the example that instructs the system that it's locked to a specific integration
    // TODO Make such examples only be run once in the CI
    // Note that this is all always relative to the root of the project because it's called from Bonnie
    let integration_locked = std::fs::metadata(format!("examples/{}/{}/.integration_locked", &category, &example)).is_ok();
    // These paths are for the CLI, which is inside `packages/perseus-cli`
    let cli_path = format!("../../examples/{}/{}", &category, &example);
    // We use `perseus-warp` because that's aliased to `perseus-integration` in all example crates for readability of code examples
    let cargo_args = format!("--features \"perseus-axum/{}\"", integration);
    if !integration_locked {
        args.push(format!("--cargo-engine-args='{}'", &cargo_args));
    }

    #[cfg(unix)]
    let shell_exec = "sh";
    #[cfg(windows)]
    let shell_exec = "powershell";
    #[cfg(unix)]
    let shell_param = "-c";
    #[cfg(windows)]
    let shell_param = "-command";

    let child = Command::new(shell_exec)
        // We don't provide any quoted arguments to the CLI ever, so this is fine
        .args([shell_param, &format!("cargo run -- {}", args.join(" "))])
        .current_dir("packages/perseus-cli") // We run on the bleeding-edge version of the CLI, from which we know the example from `TEST_EXAMPLE` above
        .env("TEST_EXAMPLE", &cli_path)
        .spawn()
        .expect("couldn't run example (command execution failed)");
    let output = child.wait_with_output().expect("couldn't wait on example executor process");

    // Pass through exit codes here (otherwise CI will think everything passes)
    if output.status.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
