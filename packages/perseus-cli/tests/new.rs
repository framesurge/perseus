use assert_cmd::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;
use std::process::Command;

/// Makes sure `perseus new` successfully generates the hardcoded example.
// TODO
#[test]
fn default() -> Result<(), Box<dyn std::error::Error>> {
    let dir = TempDir::new()?;
    let mut cmd = Command::cargo_bin("perseus")?;
    cmd.env("TEST_EXAMPLE", dir.path()) // In dev, the CLI can be made to run anywhere!
        .arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("perseus [OPTIONS] <SUBCOMMAND>"));

    Ok(())
}
