use assert_cmd::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;

/// Makes sure `perseus --help` works (if this fails, it acts as the canary of a
/// totally broken CLI!).
#[test]
#[ignore]
fn help_works() -> Result<(), Box<dyn std::error::Error>> {
    let dir = TempDir::new()?;
    let mut cmd = crate::utils::perseus_cmd(&dir);

    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("The command-line interface for Perseus, a super-fast WebAssembly frontend development framework!"));

    Ok(())
}
