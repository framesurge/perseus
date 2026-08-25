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
    use std::path::{Path, PathBuf};
    use std::process::Command;

    fn env_truthy(name: &str) -> bool {
        std::env::var(name)
            .map(|val| val == "1" || val.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    }

    fn get_cache_tools_dir() -> Option<PathBuf> {
        std::env::var_os("HOME")
            .map(PathBuf::from)
            .map(|path| path.join(".cache").join("perseus_cli").join("tools"))
    }

    fn latest_tool_binary(
        tools_dir: &Path,
        prefix: &str,
        relative_binary_path: &str,
    ) -> Option<PathBuf> {
        let mut candidates = std::fs::read_dir(tools_dir)
            .ok()?
            .filter_map(Result::ok)
            .filter(|entry| entry.file_name().to_string_lossy().starts_with(prefix))
            .map(|entry| entry.path().join(relative_binary_path))
            .filter(|path| path.is_file())
            .collect::<Vec<_>>();
        candidates.sort_unstable();
        candidates.pop()
    }

    /// Creates a preconfigured `perseus` command for integration tests.
    ///
    /// This always points the CLI at the temporary test app and can optionally
    /// force offline mode and tool paths through environment variables:
    /// - `PERSEUS_CLI_TEST_OFFLINE=1|true`
    /// - `PERSEUS_CLI_TEST_WASM_BINDGEN_PATH=/path/to/wasm-bindgen`
    /// - `PERSEUS_CLI_TEST_WASM_OPT_PATH=/path/to/wasm-opt`
    pub fn perseus_cmd(dir: &TempDir) -> Command {
        let offline = env_truthy("PERSEUS_CLI_TEST_OFFLINE");
        let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("perseus"));
        cmd.env("TEST_EXAMPLE", dir.path())
            .env("XDG_CACHE_HOME", dir.path());

        if offline {
            cmd.env("CARGO_NET_OFFLINE", "true")
                .arg("--cargo-engine-args=--offline")
                .arg("--cargo-browser-args=--offline");
        }

        if let Ok(path) = std::env::var("PERSEUS_CLI_TEST_WASM_BINDGEN_PATH") {
            if !path.is_empty() {
                cmd.arg("--wasm-bindgen-path").arg(path);
            }
        } else if offline {
            if let Some(tools_dir) = get_cache_tools_dir() {
                if let Some(path) = latest_tool_binary(&tools_dir, "wasm-bindgen-", "wasm-bindgen")
                {
                    cmd.arg("--wasm-bindgen-path").arg(path);
                }
            }
        }
        if let Ok(path) = std::env::var("PERSEUS_CLI_TEST_WASM_OPT_PATH") {
            if !path.is_empty() {
                cmd.arg("--wasm-opt-path").arg(path);
            }
        } else if offline {
            if let Some(tools_dir) = get_cache_tools_dir() {
                if let Some(path) =
                    latest_tool_binary(&tools_dir, "wasm-opt-version_", "bin/wasm-opt")
                {
                    cmd.arg("--wasm-opt-path").arg(path);
                }
            }
        }

        cmd
    }

    /// Initializes a Perseus CLI test by creating a new example app and setting
    /// it to use the bleeding-edge version of the core, so that it tests
    /// correctly.
    ///
    /// This uses the `init` command of the CLI under the hood.
    pub fn init_test(dir: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = perseus_cmd(dir);
        cmd.arg("init").arg("my-app");
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

        if env_truthy("PERSEUS_CLI_TEST_NO_SERVER") {
            return Ok(());
        }

        let mut child = cmd.group_spawn()?;

        std::thread::sleep(std::time::Duration::from_millis(5000));

        let exit_status = child.try_wait()?;
        if let Some(status) = exit_status {
            panic!(
                "server process exited unexpectedly with status '{}' (expected it to keep running)",
                status
            );
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
