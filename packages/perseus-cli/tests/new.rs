use assert_cmd::prelude::*;
use assert_fs::{prelude::*, TempDir};
use predicates::prelude::*;
use std::process::Command;

/// Makes sure `perseus new` successfully generates the hardcoded example with
/// the right version of Perseus.
#[test]
fn new_creates_files() -> Result<(), Box<dyn std::error::Error>> {
    let dir = TempDir::new()?;
    let mut cmd = Command::cargo_bin("perseus")?;
    cmd.env("TEST_EXAMPLE", dir.path()) // In dev, the CLI can be made to run anywhere!
        .arg("new")
        .arg("my-app");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Your new app has been created!"));

    // Make sure `Cargo.toml` has the right Perseus version
    dir.child("my-app/Cargo.toml")
        .assert(predicate::str::contains(env!("CARGO_PKG_VERSION")));
    // Assert that the other files exist
    dir.child("my-app/.gitignore")
        .assert(predicate::path::exists());
    dir.child("my-app/src/main.rs")
        .assert(predicate::path::exists());
    dir.child("my-app/src/templates/mod.rs")
        .assert(predicate::path::exists());
    dir.child("my-app/src/templates/index.rs")
        .assert(predicate::path::exists());

    Ok(())
}

/// Makes sure `perseus init` successfully generates the hardcoded example with
/// the right version of Perseus.
///
/// Init uses the same code as `new`, so there's no point in testing whether or
/// not it can be properly served.
#[test]
fn init_creates_files() -> Result<(), Box<dyn std::error::Error>> {
    let dir = TempDir::new()?;
    let mut cmd = Command::cargo_bin("perseus")?;
    cmd.env("TEST_EXAMPLE", dir.path()) // In dev, the CLI can be made to run anywhere!
        .arg("init")
        .arg("my-app");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Your new app has been created!"));

    // Make sure `Cargo.toml` has the right Perseus version
    dir.child("Cargo.toml")
        .assert(predicate::str::contains(env!("CARGO_PKG_VERSION")));
    // Assert that the other files exist
    dir.child(".gitignore").assert(predicate::path::exists());
    dir.child("src/main.rs").assert(predicate::path::exists());
    dir.child("src/templates/mod.rs")
        .assert(predicate::path::exists());
    dir.child("src/templates/index.rs")
        .assert(predicate::path::exists());

    Ok(())
}

/// Makes sure the output of `perseus new` can be successfully served.
#[test]
#[ignore = "long-running"]
fn new_can_be_served() -> Result<(), Box<dyn std::error::Error>> {
    let dir = TempDir::new()?;
    let mut cmd = Command::cargo_bin("perseus")?;
    cmd.env("TEST_EXAMPLE", dir.path()).arg("new").arg("my-app");
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("perseus")?;
    cmd.env("TEST_EXAMPLE", dir.path().join("my-app"))
        .arg("serve")
        // We don't want to instantiate a full server
        .arg("--no-run");
    cmd.assert().success().stdout(predicate::str::contains(
        "Not running server because `--no-run` was provided.",
    ));

    Ok(())
}
