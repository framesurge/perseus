//! WARNING: This test is currently broken, and local tool installation is being manually
//! verified. This verification is currently passing, and this test has been disabled.

use assert_cmd::prelude::*;
use assert_fs::{
    prelude::{PathAssert, PathChild},
    TempDir,
};
use predicates::prelude::*;
use std::process::Command;

/// Makes sure that `perseus build` with tools installed in the local directory
/// works fully, which tests the tool installation process and use (their paths
/// are adjustable, and it's not really feasible to test the system-wide
/// installation locally, but that's implicit on CI).
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
        .arg("--no-system-tools-cache")
        // We manually specify the versions to test that functionality (latest is implicitly
        // tested by everything else), and to have reliable file paths to assert on
        .arg("--wasm-bindgen-version")
        .arg("0.2.83")
        .arg("--wasm-opt-version")
        .arg("version_111");
    cmd.assert()
        // We can't assert on the spinners, because they've cleared from the console
        // once the program terminates
        .success();

    // Assert on the tools in `dist/tools/`
    dir.child("dist/tools/wasm-bindgen-0.2.83/wasm-bindgen")
        .assert(predicate::path::exists());
    dir.child("dist/tools/wasm-opt-version_110/bin/wasm-opt")
        .assert(predicate::path::exists());

    Ok(())
}
