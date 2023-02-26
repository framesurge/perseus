mod build;
mod clean;
mod deploy;
mod export;
mod export_error_page;
mod help;
mod new;
mod serve;
mod snoop_build;
mod snoop_serve;
mod snoop_wasm_build;
// mod tools;

mod utils {
    use assert_cmd::prelude::*;
    use assert_fs::{prelude::PathChild, TempDir};
    use predicates::prelude::*;
    use std::process::Command;

    /// Initializes a Perseus CLI test by creating a new example app and setting
    /// it to use the bleeding-edge version of the core, so that it tests
    /// correctly.
    ///
    /// This uses the `init` command of the CLI under the hood.
    pub fn init_test(dir: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("perseus")?;
        cmd.env("TEST_EXAMPLE", dir.path()) // In dev, the CLI can be made to run anywhere!
            .arg("init")
            .arg("my-app");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Your new app has been created!"));

        // Switch to the development version so we're not testing the bleeding-edge CLI
        // with the most recently released version of the core itself (where a
        // lot of bugs will originate)
        let manifest = dir.child("Cargo.toml");
        let contents = std::fs::read_to_string(&manifest).unwrap();
        // The manifest directory is `packages/perseus-cli` within the project
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let updated_contents = contents
            .replace(
                r#"perseus = { "#,
                &format!(r#"perseus = {{ path = "{}/packages/perseus", "#, &path),
            )
            .replace(
                r#"perseus-axum = { "#,
                &format!(
                    r#"perseus-axum = {{ path = "{}/packages/perseus-axum", "#,
                    &path
                ),
            );
        std::fs::write(manifest, updated_contents).unwrap();

        Ok(())
    }

    /// Tests a running app by executing the given command. This will safely
    /// terminate any child processes in the event of an error.
    pub fn test_serve(cmd: &mut Command, path: &str) -> Result<(), Box<dyn std::error::Error>> {
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
}
