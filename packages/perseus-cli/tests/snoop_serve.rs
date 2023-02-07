use crate::utils::test_serve;
use assert_cmd::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;
use std::process::Command;

/// Makes sure that `perseus snoop serve` runs the app.
#[test]
#[ignore]
fn snoop_serve_serves() -> Result<(), Box<dyn std::error::Error>> {
    let dir = TempDir::new()?;
    let mut cmd = Command::cargo_bin("perseus")?;
    cmd.env("TEST_EXAMPLE", dir.path()) // In dev, the CLI can be made to run anywhere!
        .arg("init")
        .arg("my-app");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Your new app has been created!"));

    // Build the app first
    let mut cmd = Command::cargo_bin("perseus")?;
    cmd.env("TEST_EXAMPLE", dir.path()).arg("build");
    cmd.assert().success();

    // Serve the app properly
    let mut cmd = Command::cargo_bin("perseus")?;
    cmd.env("TEST_EXAMPLE", dir.path())
        .arg("snoop")
        .arg("serve");
    test_serve(&mut cmd, "http://localhost:8080")?;

    // Try serving on a different port
    let mut cmd = Command::cargo_bin("perseus")?;
    cmd.env("TEST_EXAMPLE", dir.path())
        .arg("snoop")
        .arg("serve")
        .arg("--port")
        .arg("8000");
    test_serve(&mut cmd, "http://localhost:8000")?;

    Ok(())
}
