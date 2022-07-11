// This benchmark tests the final, release-mode, optimized size of the given example to identify regressions

use std::process::Command;
use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // This is brittle, but it's only ever called from Bonnie
    let category = &args[1];
    let example = &args[2];
    let json = &args.get(3);

    let size = deploy_opt_plugin(category, example);
    // Check if we're supposed to be using JSON
    let text = match json {
        Some(param) if *param == "--json" => format!(r#"[{{"name": "Wasm Bundle Size", "unit": "Bytes", "value": "{}"}}]"#, size),
        _ => size.to_string(),
    };
    println!("{text}");
}

fn deploy_opt_plugin(category: &str, example: &str) -> u64 {
    // NOTE This is expected to be executed at the root of the project

    // The shell configurations for Windows and Unix
    #[cfg(unix)]
    let shell_exec = "sh";
    #[cfg(windows)]
    let shell_exec = "powershell";
    #[cfg(unix)]
    let shell_param = "-c";
    #[cfg(windows)]
    let shell_param = "-command";

    // Deploy some example
    let cmd = format!("bonnie dev example {} {} deploy", category, example);
    let output = Command::new(shell_exec)
        .args([shell_param, &cmd])
        .output()
        .expect("bonnie execution failed");
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

    // There's now a Wasm file we can check the size of
    let wasm_bundle_size = std::fs::metadata(format!("examples/{}/{}/pkg/dist/pkg/perseus_engine_bg.wasm", category, example)).expect("wasm bundle not found").len();
    wasm_bundle_size
}
