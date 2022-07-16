use crate::errors::*;
use std::env;
use std::process::Command;

/// Checks if the user has the necessary prerequisites on their system (i.e.
/// `cargo` and `wasm-pack`). These can all be checked by just trying to run
/// their binaries and looking for errors. If the user has other paths for
/// these, they can define them under the environment variables
/// `PERSEUS_CARGO_PATH` and `PERSEUS_WASM_PACK_PATH`.
pub fn check_env() -> Result<(), Error> {
    // We'll loop through each prerequisite executable to check their existence
    // If the spawn returns an error, it's considered not present, success means
    // presence
    let prereq_execs = vec![(
        env::var("PERSEUS_CARGO_PATH").unwrap_or_else(|_| "cargo".to_string()),
        "cargo",
        "PERSEUS_CARGO_PATH",
    )];

    for exec in prereq_execs {
        let res = Command::new(&exec.0).output();
        // Any errors are interpreted as meaning that the user doesn't have the
        // prerequisite installed properly.
        if let Err(err) = res {
            return Err(Error::PrereqNotPresent {
                cmd: exec.1.to_string(),
                env_var: exec.2.to_string(),
                source: err,
            });
        }
    }

    Ok(())
}
