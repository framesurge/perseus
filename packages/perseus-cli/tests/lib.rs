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
    use std::io::Read;
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

        let mut child = cmd.group_spawn()?;

        std::thread::sleep(std::time::Duration::from_millis(5000));

        let exit_status = child.try_wait()?;
        if let Some(status) = exit_status {
            panic!("server process returned non-zero exit code '{}'", status);
        }

        let body = ureq::get(path)
            .call()
            .map_err(|err| {
                let _ = child.kill();
                err
            })?
            .body_mut()
            .read_to_string()
            .map_err(|err| {
                let _ = child.kill();
                err
            })?;
        assert!(body.contains("Welcome to Perseus!"));

        let _ = child.kill();

        Ok(())
    }
}
