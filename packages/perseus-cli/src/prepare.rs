use crate::cmd::run_cmd_directly;
use crate::errors::*;
use crate::extraction::extract_dir;
#[allow(unused_imports)]
use crate::PERSEUS_VERSION;
use cargo_toml::Manifest;
use include_dir::{include_dir, Dir};
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

// This literally includes the entire subcrate in the program, allowing more efficient development.
// This MUST be copied in from `../../examples/cli/.perseus/` every time the CLI is tested (use the Bonnie script).
const SUBCRATES: Dir = include_dir!("./.perseus");

/// Prepares the user's project by copying in the `.perseus/` subcrates. We use these subcrates to do all the building/serving, we just
/// have to execute the right commands in the CLI. We can essentially treat the subcrates themselves as a blackbox of just a folder.
pub fn prepare(dir: PathBuf, engine_url: &str) -> Result<(), PrepError> {
    // The location in the target directory at which we'll put the subcrates
    let target = dir.join(".perseus");

    if target.exists() {
        // We don't care if it's corrupted etc., it just has to exist
        // If the user wants to clean it, they can do that
        // Besides, we want them to be able to customize stuff
        Ok(())
    } else {
        // Create the directory first
        if let Err(err) = fs::create_dir(&target) {
            return Err(PrepError::ExtractionFailed {
                target_dir: target.to_str().map(|s| s.to_string()),
                source: err,
            });
        }
        // Check if we're using the bundled engine or a custom one
        if engine_url == "default" {
            // Write the stored directory to the target location
            // Notably, this function will not do anything or tell us if the directory already exists...
            if let Err(err) = extract_dir(SUBCRATES, &target) {
                return Err(PrepError::ExtractionFailed {
                    target_dir: target.to_str().map(|s| s.to_string()),
                    source: err,
                });
            }
        } else {
            // We're using a non-standard engine, which we'll download using Git
            // All other steps of integration with the user's package after this are the same
            let url_parts = engine_url.split('@').collect::<Vec<&str>>();
            let engine_url = url_parts[0];
            // A custom branch can be specified after a `@`, or we'll use `stable`
            let engine_branch = url_parts.get(1).unwrap_or(&"stable");
            let cmd = format!(
                // We'll only clone the production branch, and only the top level, we don't need the whole shebang
                "{} clone --single-branch --branch {branch} --depth 1 {repo} {output}",
                env::var("PERSEUS_GIT_PATH").unwrap_or_else(|_| "git".to_string()),
                branch = engine_branch,
                repo = engine_url,
                output = target.to_string_lossy()
            );
            println!("Fetching custom engine with command: '{}'.", &cmd);
            // Tell the user what command we're running so that they can debug it
            let exit_code = run_cmd_directly(
                cmd,
                &dir, // We'll run this in the current directory and output into `.perseus/`
            )
            .map_err(|err| PrepError::GetEngineFailed { source: err })?;
            if exit_code != 0 {
                return Err(PrepError::GetEngineNonZeroExitCode { exit_code });
            }
            // Now delete the Git internals
            let git_target = target.join(".git");
            if let Err(err) = fs::remove_dir_all(&git_target) {
                return Err(PrepError::RemoveEngineGitFailed {
                    target_dir: git_target.to_str().map(|s| s.to_string()),
                    source: err,
                });
            }
        }

        // Prepare for transformations on the manifest files
        // We have to store `Cargo.toml` as `Cargo.toml.old` for packaging
        let root_manifest_pkg = target.join("Cargo.toml.old");
        let root_manifest = target.join("Cargo.toml");
        let server_manifest_pkg = target.join("server/Cargo.toml.old");
        let server_manifest = target.join("server/Cargo.toml");
        let builder_manifest_pkg = target.join("builder/Cargo.toml.old");
        let builder_manifest = target.join("builder/Cargo.toml");
        let root_manifest_contents = fs::read_to_string(&root_manifest_pkg).map_err(|err| {
            PrepError::ManifestUpdateFailed {
                target_dir: root_manifest_pkg.to_str().map(|s| s.to_string()),
                source: err,
            }
        })?;
        let server_manifest_contents = fs::read_to_string(&server_manifest_pkg).map_err(|err| {
            PrepError::ManifestUpdateFailed {
                target_dir: server_manifest_pkg.to_str().map(|s| s.to_string()),
                source: err,
            }
        })?;
        let builder_manifest_contents =
            fs::read_to_string(&builder_manifest_pkg).map_err(|err| {
                PrepError::ManifestUpdateFailed {
                    target_dir: builder_manifest_pkg.to_str().map(|s| s.to_string()),
                    source: err,
                }
            })?;
        // Get the name of the user's crate (which the subcrates depend on)
        // We assume they're running this in a folder with a Cargo.toml...
        let user_manifest = Manifest::from_path("./Cargo.toml")
            .map_err(|err| PrepError::GetUserManifestFailed { source: err })?;
        let user_crate_name = user_manifest.package;
        let user_crate_name = match user_crate_name {
            Some(package) => package.name,
            None => return Err(PrepError::MalformedUserManifest),
        };
        // Update the name of the user's crate (Cargo needs more than just a path and an alias)
        // We don't need to do that in the server manifest because it uses the root code (which does parsing after `define_app!`)
        // We used to add a workspace here, but that means size optimizations apply to both the client and the server, so that's not done anymore
        // Now, we use an empty workspace to make sure we don't include the engine in any user workspaces
        // We use a token here that's set by the Bonnie `copy-subcrates` script
        let updated_root_manifest =
            root_manifest_contents.replace("USER_PKG_NAME", &user_crate_name) + "\n[workspace]";
        let updated_server_manifest = server_manifest_contents + "\n[workspace]";
        let updated_builder_manifest = builder_manifest_contents + "\n[workspace]";

        // We also need to set the Perseus version
        // In production, we'll use the full version, but in development we'll use relative path references from the examples
        // The tokens here are set by Bonnie's `copy-subcrates` script once again
        // Production
        #[cfg(not(debug_assertions))]
        let updated_root_manifest = updated_root_manifest.replace(
            "PERSEUS_VERSION",
            &format!("version = \"{}\"", PERSEUS_VERSION),
        );
        #[cfg(not(debug_assertions))]
        let updated_server_manifest = updated_server_manifest
            .replace(
                "PERSEUS_VERSION",
                &format!("version = \"{}\"", PERSEUS_VERSION),
            )
            .replace(
                "PERSEUS_ACTIX_WEB_VERSION",
                &format!("version = \"{}\"", PERSEUS_VERSION),
            )
            .replace(
                "PERSEUS_WARP_VERSION",
                &format!("version = \"{}\"", PERSEUS_VERSION),
            );
        #[cfg(not(debug_assertions))]
        let updated_builder_manifest = updated_builder_manifest.replace(
            "PERSEUS_VERSION",
            &format!("version = \"{}\"", PERSEUS_VERSION),
        );
        // Development
        #[cfg(debug_assertions)]
        let updated_root_manifest = updated_root_manifest
            .replace("PERSEUS_VERSION", "path = \"../../../../packages/perseus\"");
        #[cfg(debug_assertions)]
        let updated_server_manifest = updated_server_manifest
            .replace(
                "PERSEUS_VERSION",
                "path = \"../../../../../packages/perseus\"",
            )
            .replace(
                "PERSEUS_ACTIX_WEB_VERSION",
                "path = \"../../../../../packages/perseus-actix-web\"",
            )
            .replace(
                "PERSEUS_WARP_VERSION",
                "path = \"../../../../../packages/perseus-warp\"",
            );
        #[cfg(debug_assertions)]
        let updated_builder_manifest = updated_builder_manifest.replace(
            "PERSEUS_VERSION",
            "path = \"../../../../../packages/perseus\"",
        );

        // Write the updated manifests back
        if let Err(err) = fs::write(&root_manifest, updated_root_manifest) {
            return Err(PrepError::ManifestUpdateFailed {
                target_dir: root_manifest.to_str().map(|s| s.to_string()),
                source: err,
            });
        }
        if let Err(err) = fs::write(&server_manifest, updated_server_manifest) {
            return Err(PrepError::ManifestUpdateFailed {
                target_dir: server_manifest.to_str().map(|s| s.to_string()),
                source: err,
            });
        }
        if let Err(err) = fs::write(&builder_manifest, updated_builder_manifest) {
            return Err(PrepError::ManifestUpdateFailed {
                target_dir: builder_manifest.to_str().map(|s| s.to_string()),
                source: err,
            });
        }

        // If we aren't already gitignoring the subcrates, update .gitignore to do so
        if let Ok(contents) = fs::read_to_string(".gitignore") {
            if contents.contains(".perseus/") {
                return Ok(());
            }
        }
        let file = OpenOptions::new()
            .append(true)
            .create(true) // If it doesn't exist, create it
            .open(".gitignore");
        let mut file = match file {
            Ok(file) => file,
            Err(err) => return Err(PrepError::GitignoreUpdateFailed { source: err }),
        };
        // Check for errors with appending to the file
        if let Err(err) = file.write_all(b"\n.perseus/") {
            return Err(PrepError::GitignoreUpdateFailed { source: err });
        }
        Ok(())
    }
}

/// Checks if the user has the necessary prerequisites on their system (i.e. `cargo` and `wasm-pack`). These can all be checked
/// by just trying to run their binaries and looking for errors. If the user has other paths for these, they can define them under the
/// environment variables `PERSEUS_CARGO_PATH` and `PERSEUS_WASM_PACK_PATH`.
pub fn check_env() -> Result<(), PrepError> {
    // We'll loop through each prerequisite executable to check their existence
    // If the spawn returns an error, it's considered not present, success means presence
    let prereq_execs = vec![
        (
            env::var("PERSEUS_CARGO_PATH").unwrap_or_else(|_| "cargo".to_string()),
            "cargo",
            "PERSEUS_CARGO_PATH",
        ),
        (
            env::var("PERSEUS_WASM_PACK_PATH").unwrap_or_else(|_| "wasm-pack".to_string()),
            "wasm-pack",
            "PERSEUS_WASM_PACK_PATH",
        ),
    ];

    for exec in prereq_execs {
        let res = Command::new(&exec.0).output();
        // Any errors are interpreted as meaning that the user doesn't have the prerequisite installed properly.
        if let Err(err) = res {
            return Err(PrepError::PrereqNotPresent {
                cmd: exec.1.to_string(),
                env_var: exec.2.to_string(),
                source: err,
            });
        }
    }

    Ok(())
}
