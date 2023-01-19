use assert_cmd::prelude::*;
use assert_fs::{
    prelude::{PathAssert, PathChild},
    TempDir,
};
use predicates::prelude::*;
use std::process::Command;

/// Makes sure that `perseus clean` removes the `dist/` directory entirely after
/// a build.
#[test]
#[ignore]
fn clean_removes_dist() -> Result<(), Box<dyn std::error::Error>> {
    let dir = TempDir::new()?;
    let mut cmd = Command::cargo_bin("perseus")?;
    cmd.env("TEST_EXAMPLE", dir.path()) // In dev, the CLI can be made to run anywhere!
        .arg("init")
        .arg("my-app");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Your new app has been created!"));

    // Build the app and make sure `clean` removes the `dist/` directory
    let mut cmd = Command::cargo_bin("perseus")?;
    cmd.env("TEST_EXAMPLE", dir.path()).arg("build");
    cmd.assert().success();

    // The render config will always be produced
    dir.child("dist/render_conf.json")
        .assert(predicate::path::exists());

    let mut cmd = Command::cargo_bin("perseus")?;
    cmd.env("TEST_EXAMPLE", dir.path()).arg("clean");
    cmd.assert().success();

    dir.child("dist").assert(predicate::path::missing());

    Ok(())
}
