use assert_cmd::prelude::*;
use assert_fs::{
    prelude::{PathAssert, PathChild},
    TempDir,
};
use predicates::prelude::*;
use std::process::Command;

/// Makes sure that `perseus build` with tools installed in the local directory works
/// fully, which tests the tool installation process and use (their paths are adjustable,
/// and it's not really feasible to test the system-wide installation locally, but that's
/// implicit on CI).
#[test]
#[ignore]
fn local_tool_installation_works() -> Result<(), Box<dyn std::error::Error>> {
    let dir = TempDir::new()?;
    let mut cmd = Command::cargo_bin("perseus")?;
    cmd.env("TEST_EXAMPLE", dir.path()) // In dev, the CLI can be made to run anywhere!
        .arg("init")
        .arg("my-app");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Your new app has been created!"));

    // Build the app
    let mut cmd = Command::cargo_bin("perseus")?;
    cmd.env("TEST_EXAMPLE", dir.path())
        .arg("build")
        .arg("--no-system-tools-cache");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Installing external tools...✅"));

    Ok(())
}
