use assert_cmd::prelude::*;
use assert_fs::{
    prelude::{PathAssert, PathChild},
    TempDir,
};
use predicates::prelude::*;
use std::process::Command;

/// Makes sure that `perseus build` produces the correct artifacts.
///
/// This test is tightly coupled to the form of the static artifacts, and can also
/// act as a canary for some other problems.
#[test]
#[ignore]
fn build_produces_artifacts() -> Result<(), Box<dyn std::error::Error>> {
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
    cmd.env("TEST_EXAMPLE", dir.path()).arg("build");
    cmd.assert().success();

    // Assert on all the artifacts, based on the code in the `init` example
    dir.child("dist/render_conf.json")
        .assert(predicate::path::exists());
    dir.child("dist/pkg/perseus_engine.d.ts")
       .assert(predicate::path::exists());
    dir.child("dist/pkg/perseus_engine.js")
        .assert(predicate::path::exists());
    dir.child("dist/pkg/perseus_engine_bg.wasm")
       .assert(predicate::path::exists());
    dir.child("dist/pkg/perseus_engine_bg.wasm.d.ts")
       .assert(predicate::path::exists());
    dir.child("dist/static/xx-XX-.html")
        // We don't assert any more than this due to hydration IDs and minification
       .assert(predicate::str::contains("Welcome to Perseus!"));
    dir.child("dist/static/xx-XX-.head.html")
       .assert(predicate::str::is_match("^<title>Welcome to Perseus!</title>$").unwrap());
    #[cfg(unix)] // It would have `.exe` on Windows
    dir.child("dist/target_engine/debug/my-app")
       .assert(predicate::path::exists());
    dir.child("dist/target_wasm/wasm32-unknown-unknown/debug/my-app.wasm")
       .assert(predicate::path::exists());

    Ok(())
}
