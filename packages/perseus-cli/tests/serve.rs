use crate::utils::test_serve;
use assert_cmd::prelude::*;
use assert_fs::{
    prelude::{PathAssert, PathChild},
    TempDir,
};
use predicates::prelude::*;
use std::process::Command;

/// Makes sure that `perseus serve --no-run` produces the correct artifacts.
/// This will then test that the local server actually works by running `perseus
/// serve` properly after that (assuming it will start almost immediately the
/// second time).
#[test]
#[ignore]
fn serve_produces_artifacts_and_serves() -> Result<(), Box<dyn std::error::Error>> {
    let dir = TempDir::new()?;
    let mut cmd = Command::cargo_bin("perseus")?;
    cmd.env("TEST_EXAMPLE", dir.path()) // In dev, the CLI can be made to run anywhere!
        .arg("init")
        .arg("my-app");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Your new app has been created!"));

    // Serve the app without running it
    let mut cmd = Command::cargo_bin("perseus")?;
    cmd.env("TEST_EXAMPLE", dir.path())
        .arg("serve")
        .arg("--no-run");
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

    // Serve the app properly
    let mut cmd = Command::cargo_bin("perseus")?;
    cmd.env("TEST_EXAMPLE", dir.path()).arg("serve");
    test_serve(&mut cmd, "http://localhost:8080")?;

    // Try serving on a different port
    let mut cmd = Command::cargo_bin("perseus")?;
    cmd.env("TEST_EXAMPLE", dir.path())
        .arg("serve")
        .arg("--port")
        .arg("8000");
    test_serve(&mut cmd, "http://localhost:8000")?;

    // And try with `--no-build`
    let mut cmd = Command::cargo_bin("perseus")?;
    cmd.env("TEST_EXAMPLE", dir.path())
        .arg("serve")
        .arg("--no-build");
    test_serve(&mut cmd, "http://localhost:8080")?;

    Ok(())
}
