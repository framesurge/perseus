use crate::cmd::{cfg_spinner, fail_spinner, succeed_spinner};
use crate::errors::*;
use crate::parse::Opts;
use console::Emoji;
use flate2::read::GzDecoder;
use futures::future::try_join;
use indicatif::ProgressBar;
use reqwest::Client;
use std::borrow::BorrowMut;
use std::fs;
use std::fs::File;
use std::{path::Path, process::Command};
use tar::Archive;
use tokio::io::AsyncWriteExt;

static INSTALLING: Emoji<'_, '_> = Emoji("📥", "");

// For each of the tools installed in this file, we preferentially
// manually download it. If that can't be achieved due to a platform
// mismatch, then we'll see if the user already has a verion installed.
//
// Importantly, if the user has specified an environment variable specifying
// where a tool can be found, we'll use that no matter what.

/// Gets a tool's path or installs it. Environment variables should be checked
/// before calling this.
async fn get_tool(
    target: &Path,
    tool: &str,
    (tool_path, tool_version): (&Option<String>, &Option<String>),
    // If this is built on an unsupported platform, this should be `None`
    // This should contain `%version` to be replaced with the version
    artifact_name: Option<&str>,
    // Needed for checking the latest version
    gh_repo: &str,
    // This should contain `%version` to be replaced with the version
    extracted_dir_name: &str,
    // This should be relative to the directory that's extracted,
    final_path: &str,
) -> Result<String, InstallError> {
    // Before we start, we sanity check the tool's presence again
    // This is because the main system will blindly rerun all installations, even if
    // only one tool is missing, which can lead to corruptions in some cases
    if let Some(path) = check_tool(target, tool, tool_path, final_path) {
        return Ok(path);
    };

    // The path within the extracted directory (which will be named as the tool is)
    #[cfg(unix)]
    let final_path = final_path.to_string();
    #[cfg(windows)]
    let final_path = final_path.to_string() + ".exe";

    // If there's no precompiled binary for this platform, then
    // we'll check if there's anything at the usual path. The reason we don't do
    // this first is because we want version matches for `wasm-bindgen` in
    // particular.
    if artifact_name.is_none() {
        #[cfg(unix)]
        let shell_exec = "sh";
        #[cfg(windows)]
        let shell_exec = "powershell";
        #[cfg(unix)]
        let shell_param = "-c";
        #[cfg(windows)]
        let shell_param = "-command";

        let check_cmd = format!("{} --version", tool); // Not exactly bulletproof, but it works!
        let res = Command::new(shell_exec)
            .args([shell_param, &check_cmd])
            .output();
        if let Err(err) = res {
            // Unlike `wasm-pack`, we don't try to install with `cargo install`, because
            // that's a little pointless to me (the user will still have to get
            // `wasm-opt` somehow...)
            //
            // TODO Installation script that can build manually on any platform
            return Err(InstallError::ExternalToolUnavailable {
                tool: tool.to_string(),
                source: err,
            });
        } else {
            // It works, so we don't need to install anything
            return Ok(tool.to_string());
        }
    }
    // If we've gotten this far, we're installing something for which there are
    // binaries available
    let artifact_name = artifact_name.unwrap();
    // Get the latest version if the user hasn't specified something else
    let version = match tool_version {
        Some(v) => v.to_string(),
        None => {
            let json = Client::new()
                .get(&format!(
                    "https://api.github.com/repos/{}/releases/latest",
                    gh_repo
                ))
                .header("User-Agent", "perseus-cli")
                .send()
                .await
                .map_err(|err| InstallError::GetLatestToolVersionFailed {
                    source: err,
                    tool: tool.to_string(),
                })?
                .json::<serde_json::Value>()
                .await
                .map_err(|err| InstallError::GetLatestToolVersionFailed {
                    source: err,
                    tool: tool.to_string(),
                })?;
            let latest_version =
                json.get("name")
                    .ok_or_else(|| InstallError::ParseToolVersionFailed {
                        tool: tool.to_string(),
                    })?;

            latest_version
                .as_str()
                .ok_or_else(|| InstallError::ParseToolVersionFailed {
                    tool: tool.to_string(),
                })?
                .to_string()
        }
    };

    let artifact_name_with_version = artifact_name.replace("%version", &version);
    let url = format!(
        "https://github.com/{gh_repo}/releases/download/{version}/{artifact_name}.tar.gz",
        artifact_name = &artifact_name_with_version,
        version = version,
        gh_repo = gh_repo
    );
    // Download the tarball (source https://github.com/seanmonstar/reqwest/issues/1266#issuecomment-1106187437)
    // We do this by chunking to minimize memory usage (we're downloading fairly
    // large files!)
    let tar_dest = target.join(&format!("{}.tar.gz", tool));
    let mut res = Client::new().get(url).send().await.map_err(|err| {
        InstallError::BinaryDownloadRequestFailed {
            source: err,
            tool: tool.to_string(),
        }
    })?;
    let mut file = tokio::fs::File::create(&tar_dest)
        .await
        .map_err(|err| InstallError::CreateToolDownloadDestFailed { source: err })?;
    while let Some(mut item) = res
        .chunk()
        .await
        .map_err(|err| InstallError::ChunkBinaryDownloadFailed { source: err })?
    {
        file.write_all_buf(item.borrow_mut())
            .await
            .map_err(|err| InstallError::WriteBinaryDownloadChunkFailed { source: err })?;
    }
    // Now unzip the tarball
    // TODO Async?
    let tar_gz = File::open(&tar_dest)
        .map_err(|err| InstallError::CreateToolExtractDestFailed { source: err })?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    // We'll extract straight into `dist/tools/` and then rename the resulting
    // directory
    archive
        .unpack(target)
        .map_err(|err| InstallError::ToolExtractFailed {
            source: err,
            tool: tool.to_string(),
        })?;
    // Now delete the original archive file
    fs::remove_file(&tar_dest)
        .map_err(|err| InstallError::ArchiveDeletionFailed { source: err })?;
    // Finally, rename the extracted directory
    fs::rename(
        target.join(extracted_dir_name.replace("%version", &version)),
        target.join(tool),
    )
    .map_err(|err| InstallError::DirRenameFailed { source: err })?;

    // Return the path inside the directory we extracted
    Ok(target
        .join(tool)
        .join(final_path)
        .to_str()
        .unwrap()
        .to_string())
}

/// Installs `wasm-bindgen`, returning the path to the tool. This should not be
/// called if the tool already exists, or if we've been given an environment
/// variable for it.
async fn install_wasm_bindgen(dir: &Path, global_opts: &Opts) -> Result<String, InstallError> {
    // This is based on https://github.com/rustwasm/wasm-bindgen/releases
    let artifact_name = match true {
        // Linux
        _ if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") => {
            Some("wasm-bindgen-%version-x86_64-unknown-linux-musl")
        }
        // MacOS (incl. Apple Silicon)
        _ if cfg!(target_os = "macos")
            && (cfg!(target_arch = "x86_64") || cfg!(target_arch = "aarch64")) =>
        {
            Some("wasm-bindgen-%version-x86_64-apple-darwin")
        }
        // Windows
        _ if cfg!(target_os = "windows") && cfg!(target_arch = "x86_64") => {
            Some("wasm-bindgen-%version-x86_64-pc-windows-msvc")
        }
        _ => None,
    };

    get_tool(
        &dir.join("dist/tools"),
        "wasm-bindgen",
        (
            &global_opts.wasm_bindgen_path,
            &global_opts.wasm_bindgen_version,
        ),
        artifact_name,
        "rustwasm/wasm-bindgen",
        // The name of the extarcted directory is the same as the artifact name
        // If we don't have an artifact name, this will NEVER be used
        artifact_name.unwrap_or(""),
        "wasm-bindgen",
    )
    .await
}

/// Installs `wasm-opt`, returning the path to the tool. This should not be
/// called if the tool already exists, or if we've been given an environment
/// variable for it.
async fn install_wasm_opt(dir: &Path, global_opts: &Opts) -> Result<String, InstallError> {
    // This is based on https://github.com/WebAssembly/binaryen/releases
    let artifact_name = match true {
        // Linux
        _ if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") => {
            Some("binaryen-%version-x86_64-linux")
        }
        // MacOS (Intel)
        _ if cfg!(target_os = "macos") && cfg!(target_arch = "x86_64") => {
            Some("binaryen-%version-x86_64-macos")
        }
        // MacOS (Apple Silicon)
        _ if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") => {
            Some("binaryen-%version-arm64-macos")
        }
        // Windows
        _ if cfg!(target_os = "windows") && cfg!(target_arch = "x86_64") => {
            Some("binaryen-%version-x86_64-windows")
        }
        _ => None,
    };

    get_tool(
        &dir.join("dist/tools"),
        "wasm-opt",
        (&global_opts.wasm_opt_path, &global_opts.wasm_opt_version),
        artifact_name,
        "WebAssembly/binaryen",
        "binaryen-%version",
        "bin/wasm-opt",
    )
    .await
}

/// Checks whether or not a tool has already been downloaded or if we've been
/// told to find it at a particular path (in an environment variable).
fn check_tool(
    target: &Path,
    tool: &str,
    tool_path: &Option<String>,
    // This will be automatically adjusted for Windows
    final_path: &str,
) -> Option<String> {
    // First, check if the user has told us where to find the tool
    // If they have, we just assume it's valid
    if let Some(path) = tool_path {
        return Some(path.to_string());
    }

    // The path within the extracted directory (which will be named as the tool is)
    #[cfg(unix)]
    let final_path = final_path.to_string();
    #[cfg(windows)]
    let final_path = final_path.to_string() + ".exe";

    // Check if the tool already exists
    let expected_binary = target.join(&tool).join(final_path);
    if fs::metadata(&expected_binary).is_ok() {
        Some(expected_binary.to_str().unwrap().to_string())
    } else {
        None
    }
}

/// Installs all the external tool dependencies we need. These will be installed
/// into `dist/tools/`. This returns a `struct` that tells the rest of the
/// program where to find these tools.
///
/// This also creates a CLI spinner to track progress (if we need to install
/// anything).
pub async fn install_tools(dir: &Path, global_opts: &Opts) -> Result<Tools, InstallError> {
    // Create the directory for tools
    let target = dir.join("dist/tools");
    if !target.exists() {
        fs::create_dir(&target)
            .map_err(|err| InstallError::CreateToolsDirFailed { source: err })?;
    }

    // Check if the tools exist (if they all do, we won't bother with the spinner)
    let expected_paths = (
        check_tool(
            &target,
            "wasm-bindgen",
            &global_opts.wasm_bindgen_path,
            "wasm-bindgen",
        ),
        check_tool(
            &target,
            "wasm-opt",
            &global_opts.wasm_opt_path,
            "bin/wasm-opt",
        ),
    );
    if let (Some(wb), Some(wo)) = expected_paths {
        return Ok(Tools {
            cargo: global_opts.cargo_path.clone(),
            wasm_bindgen: wb,
            wasm_opt: wo,
        });
    }

    // At least one of the tools isn't ready for use (installed or has env var), so
    // we'll run the installation procedure The installation functions sanity
    // check their existence, so we avoid installing things that already exist here
    // Set up a CLI spinner (this will probably take some time)
    let spinner_msg = format!("{} Installing external tools", INSTALLING);
    let spinner = cfg_spinner(ProgressBar::new_spinner(), &spinner_msg);

    // Install all the tools in parallel
    let res = try_join(
        install_wasm_bindgen(dir, global_opts),
        install_wasm_opt(dir, global_opts),
    )
    .await;
    if let Err(err) = res {
        fail_spinner(&spinner, &spinner_msg);
        return Err(err);
    }
    // If we're here, we have the paths
    succeed_spinner(&spinner, &spinner_msg);
    let paths = res.unwrap();

    Ok(Tools {
        cargo: global_opts.cargo_path.clone(),
        wasm_bindgen: paths.0,
        wasm_opt: paths.1,
    })
}

/// A representation of the paths to all the external tools we need.
/// This includes `cargo`, simply for convenience, even though it's not
/// actually independently installed.
#[derive(Clone)]
pub struct Tools {
    pub cargo: String,
    pub wasm_bindgen: String,
    pub wasm_opt: String,
}
