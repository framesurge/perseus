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

/// Tests a running app by executing the given command. This will safely
/// terminate any child processes in the event of an error.
#[cfg(test)]
fn test_serve(cmd: &mut Command, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use command_group::CommandGroup;

    // We use a group process spawn because the child will spawn the server process,
    // which can't be cleaned up if SIGKILL is sent
    let mut child = cmd.group_spawn()?;

    std::thread::sleep(std::time::Duration::from_millis(5000));

    // Check if the child process has failed (this is the only way to catch
    // things like binding errors); this will not block trying to wait
    let exit_status = child.try_wait()?;
    if let Some(status) = exit_status {
        panic!("server process returned non-zero exit code '{}'", status);
    }

    // We don't extensively test things here, since that's what Perseus' testing
    // system is for, and that tests *all* the core examples extensively in a
    // headless browser, which is more realistic than simple HTTP requests
    // anyway
    let body = ureq::get(path)
        .call()
        .map_err(|err| {
            let _ = child.kill();
            err
        })?
        .into_string()
        .map_err(|err| {
            let _ = child.kill();
            err
        })?;
    assert!(body.contains("Welcome to Perseus!"));

    let _ = child.kill();

    Ok(())
}
