use assert_cmd::prelude::*;
use assert_fs::{
    prelude::{PathAssert, PathChild},
    TempDir,
};
use predicates::prelude::*;
use std::process::Command;

use crate::utils::init_test;

/// Makes sure that `perseus snoop wasm-build` produces the correct artifacts.
///
/// This test is tightly coupled to the form of the static artifacts, and can
/// also act as a canary for some other problems.
#[test]
#[ignore]
fn snoop_wasm_build_produces_artifacts() -> Result<(), Box<dyn std::error::Error>> {
    let dir = TempDir::new()?;
    init_test(&dir)?;

    // Build the app
    let mut cmd = Command::cargo_bin("perseus")?;
    cmd.env("TEST_EXAMPLE", dir.path())
        .arg("snoop")
        .arg("wasm-build");
    cmd.assert().success();

    // Assert on all the artifacts, based on the code in the `init` example
    dir.child("dist/render_conf.json")
        .assert(predicate::path::missing());
    dir.child("dist/pkg/perseus_engine.d.ts")
        .assert(predicate::path::exists());
    dir.child("dist/pkg/perseus_engine.js")
        .assert(predicate::path::exists());
    dir.child("dist/pkg/perseus_engine_bg.wasm")
        .assert(predicate::path::exists());
    dir.child("dist/pkg/perseus_engine_bg.wasm.d.ts")
        .assert(predicate::path::exists());
    dir.child("dist/static/xx-XX-.html")
        .assert(predicate::path::missing());
    dir.child("dist/static/xx-XX-.head.html")
        .assert(predicate::path::missing());
    #[cfg(unix)] // It would have `.exe` on Windows
    dir.child("dist/target_engine/debug/my-app")
        .assert(predicate::path::missing());
    dir.child("dist/target_wasm/wasm32-unknown-unknown/debug/my-app.wasm")
        .assert(predicate::path::exists());

    Ok(())
}
