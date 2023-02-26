use crate::utils::{init_test, test_serve};
use assert_cmd::prelude::*;
use assert_fs::{
    prelude::{PathAssert, PathChild},
    TempDir,
};
use predicates::prelude::*;
use std::process::Command;

/// Makes sure that `perseus deploy` works correctly.
///
/// This is a critical canary test, as deployment has statistically been broken
/// the most often of all the CLI commands. If this test does not pass,
/// immediate action should be taken!
#[test]
// #[ignore]
fn deploy_works() -> Result<(), Box<dyn std::error::Error>> {
    let dir = TempDir::new()?;
    init_test(&dir)?;

    // Deploy the app without first specifying our own error views (this should
    // fail)
    let mut cmd = Command::cargo_bin("perseus")?;
    cmd.env("TEST_EXAMPLE", dir.path()).arg("deploy");
    cmd.assert().failure();

    // Now add the development default error views for simplicity
    let main_rs = dir.child("src/main.rs");
    let contents = std::fs::read_to_string(&main_rs).unwrap();
    let contents_with_dbg = contents.replace(
        r#".template(crate::templates::index::get_template())"#,
        r#".template(crate::templates::index::get_template())
    .error_views(perseus::prelude::ErrorViews::unlocalized_development_default())"#,
    );
    std::fs::write(main_rs, contents_with_dbg).unwrap();

    // Deploy the app now, and it should work
    let mut cmd = Command::cargo_bin("perseus")?;
    cmd.env("TEST_EXAMPLE", dir.path()).arg("deploy");
    cmd.assert().success();

    // Assert on all the artifacts, based on the code in the `init` example
    dir.child("pkg/server").assert(predicate::path::exists());
    dir.child("pkg/dist/render_conf.json")
        .assert(predicate::path::exists());
    dir.child("pkg/dist/pkg/perseus_engine.d.ts")
        .assert(predicate::path::exists());
    dir.child("pkg/dist/pkg/perseus_engine.js")
        .assert(predicate::path::exists());
    dir.child("pkg/dist/pkg/perseus_engine_bg.wasm")
        .assert(predicate::path::exists());
    dir.child("pkg/dist/pkg/perseus_engine_bg.wasm.d.ts")
        .assert(predicate::path::exists());
    dir.child("pkg/dist/static/xx-XX-.html")
        // We don't assert any more than this due to hydration IDs and minification
        .assert(predicate::str::contains("Welcome to Perseus!"));
    dir.child("pkg/dist/static/xx-XX-.head.html")
        .assert(predicate::str::is_match("^<title>Welcome to Perseus!</title>$").unwrap());

    // And now try actually running the server
    let mut cmd = Command::new(dir.child("pkg/server").path());
    test_serve(&mut cmd, "http://localhost:8080")?;

    Ok(())
}
