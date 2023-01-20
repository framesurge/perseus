use assert_cmd::prelude::*;
use assert_fs::{
    prelude::{PathAssert, PathChild},
    TempDir,
};
use predicates::prelude::*;
use std::process::Command;

/// Makes sure that `perseus export` produces the correct artifacts.
#[test]
#[ignore]
fn export_produces_artifacts() -> Result<(), Box<dyn std::error::Error>> {
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
    cmd.env("TEST_EXAMPLE", dir.path()).arg("export");
    cmd.assert().success();

    // Assert on all the artifacts, based on the code in the `init` example
    dir.child("dist/exported/index.html")
        .assert(predicate::str::contains("Welcome to Perseus!"));
    dir.child("dist/exported/.perseus/bundle.js")
        .assert(predicate::path::exists());
    dir.child("dist/exported/.perseus/bundle.wasm")
        .assert(predicate::path::exists());
    dir.child("dist/exported/.perseus/page/xx-XX/.json")
        .assert(predicate::str::starts_with(
            r#"{"state":null,"head":"<title>Welcome to Perseus!</title>"}"#,
        ));

    Ok(())
}
