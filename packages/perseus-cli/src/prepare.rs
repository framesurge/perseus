use crate::cmd::run_cmd_directly;
use crate::errors::*;
use crate::parse::Opts;
use std::path::PathBuf;
use std::process::Command;

/// Checks if the user has the necessary prerequisites on their system (i.e.
/// `cargo` and `wasm-pack`). These can all be checked by just trying to run
/// their binaries and looking for errors. If the user has other paths for
/// these, they can define them under the environment variables
/// `PERSEUS_CARGO_PATH` and `PERSEUS_WASM_PACK_PATH`.
///
/// Checks if the user has `cargo` installed, and tries to install the
/// `wasm32-unknown-unknown` target with `rustup` if it's available.
pub fn check_env(global_opts: &Opts) -> Result<(), Error> {
    #[cfg(unix)]
    let shell_exec = "sh";
    #[cfg(windows)]
    let shell_exec = "powershell";
    #[cfg(unix)]
    let shell_param = "-c";
    #[cfg(windows)]
    let shell_param = "-command";

    // Check for `cargo`
    let cargo_cmd = global_opts.cargo_engine_path.to_string() + " --version";
    let cargo_res = Command::new(shell_exec)
        .args([shell_param, &cargo_cmd])
        .output()
        .map_err(|err| Error::CargoNotPresent { source: err })?;
    let exit_code = match cargo_res.status.code() {
        Some(exit_code) => exit_code,
        None if cargo_res.status.success() => 0,
        None => 1,
    };
    if exit_code != 0 {
        return Err(Error::CargoNotPresent {
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "non-zero exit code"),
        });
    }
    // If the user has `rustup`, make sure they have `wasm32-unknown-unknown`
    // installed If they don'aren't using `rustup`, we won't worry about this
    let rustup_cmd = global_opts.rustup_path.to_string() + " target list";
    let rustup_res = Command::new(shell_exec)
        .args([shell_param, &rustup_cmd])
        .output();
    if let Ok(rustup_res) = rustup_res {
        let exit_code = match rustup_res.status.code() {
            Some(exit_code) => exit_code,
            None if rustup_res.status.success() => 0,
            None => 1,
        };
        if exit_code == 0 {
            let stdout = String::from_utf8_lossy(&rustup_res.stdout);
            let has_wasm_target = stdout.contains("wasm32-unknown-unknown (installed)");
            if !has_wasm_target {
                let exit_code = run_cmd_directly(
                    "rustup target add wasm32-unknown-unknown".to_string(),
                    &PathBuf::from("."),
                    vec![],
                )?;
                if exit_code != 0 {
                    return Err(Error::RustupTargetAddFailed { code: exit_code });
                }
            }
        }
    }

    Ok(())
}
