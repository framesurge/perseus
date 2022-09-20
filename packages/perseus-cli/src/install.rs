use crate::cmd::{cfg_spinner, fail_spinner, run_stage, succeed_spinner};
use crate::errors::*;
use crate::parse::Opts;
use cargo_lock::Lockfile;
use cargo_metadata::MetadataCommand;
use console::Emoji;
use directories::ProjectDirs;
use flate2::read::GzDecoder;
use futures::future::try_join;
use indicatif::ProgressBar;
use reqwest::Client;
use std::borrow::BorrowMut;
use std::fs;
use std::fs::File;
use std::{
    path::{Path, PathBuf},
    process::Command,
};
use tar::Archive;
use tokio::io::AsyncWriteExt;

static INSTALLING: Emoji<'_, '_> = Emoji("📥", "");
static GENERATING_LOCKFILE: Emoji<'_, '_> = Emoji("🔏", "");

// For each of the tools installed in this file, we preferentially
// manually download it. If that can't be achieved due to a platform
// mismatch, then we'll see if the user already has a version installed.
//
// Importantly, if the user has specified an environment variable specifying
// where a tool can be found, we'll use that no matter what.

/// Gets the directory to store tools in. This will preferentially use the
/// system-wide cache, falling back to a local version.
///
/// If the user specifies that we're running on CI, we'll use the local version
/// regardless.
pub fn get_tools_dir(project: &Path, no_system_cache: bool) -> Result<PathBuf, InstallError> {
    match ProjectDirs::from("", "perseus", "perseus_cli") {
        Some(dirs) if !no_system_cache => {
            let target = dirs.cache_dir().join("tools");
            if target.exists() {
                Ok(target)
            } else {
                // Try to create the system-wide cache
                if fs::create_dir_all(&target).is_ok() {
                    Ok(target)
                } else {
                    // Failed, so we'll resort to the local cache
                    let target = project.join("dist/tools");
                    if !target.exists() {
                        // If this fails, we have no recourse, so we'll have to fail
                        fs::create_dir_all(&target)
                            .map_err(|err| InstallError::CreateToolsDirFailed { source: err })?;
                    }
                    // It either already existed or we've just created it
                    Ok(target)
                }
            }
        }
        _ => {
            let target = project.join("dist/tools");
            if !target.exists() {
                // If this fails, we have no recourse, so we'll have to fail
                fs::create_dir_all(&target)
                    .map_err(|err| InstallError::CreateToolsDirFailed { source: err })?;
            }
            // It either already existed or we've just created it
            Ok(target)
        }
    }
}

/// A representation of the paths to all the external tools we need.
/// This includes `cargo`, simply for convenience, even though it's not
/// actually independently installed.
///
/// This does not contain metadata for the installation process, but is rather
/// intended to be passed around through the rest of the CLI.
#[derive(Clone)]
pub struct Tools {
    /// The path to `cargo` on the engine-side.
    pub cargo_engine: String,
    /// The path to `cargo` on the browser-side.
    pub cargo_browser: String,
    /// The path to `wasm-bindgen`.
    pub wasm_bindgen: String,
    /// The path to `wasm-opt`.
    pub wasm_opt: String,
}
impl Tools {
    /// Gets a new instance of `Tools` by installing the tools if necessary.
    ///
    /// If tools are installed, this will create a CLI spinner automatically.
    pub async fn new(dir: &Path, global_opts: &Opts) -> Result<Self, InstallError> {
        // First, make sure `Cargo.lock` exists, since we'll need it for determining the
        // right version of `wasm-bindgen`
        let metadata = MetadataCommand::new()
            .no_deps()
            .exec()
            .map_err(|err| InstallError::MetadataFailed { source: err })?;
        let workspace_root = metadata.workspace_root.into_std_path_buf();
        let lockfile_path = workspace_root.join("Cargo.lock");
        if !lockfile_path.exists() {
            let lf_msg = format!("{} Generating Cargo lockfile", GENERATING_LOCKFILE);
            let lf_spinner = cfg_spinner(ProgressBar::new_spinner(), &lf_msg);
            let (_stdout, _stderr, exit_code) = run_stage(
                vec!["cargo generate-lockfile"],
                &workspace_root,
                &lf_spinner,
                &lf_msg,
                Vec::new(),
            )
            .map_err(|err| InstallError::LockfileGenerationFailed { source: err })?;
            if exit_code != 0 {
                // The output has already been handled, just terminate
                return Err(InstallError::LockfileGenerationNonZero { code: exit_code });
            }
        }
        let lockfile = Lockfile::load(lockfile_path)
            .map_err(|err| InstallError::LockfileLoadFailed { source: err })?;

        let target = get_tools_dir(dir, global_opts.no_system_tools_cache)?;

        // Instantiate the tools
        let wasm_bindgen = Tool::new(
            ToolType::WasmBindgen,
            &global_opts.wasm_bindgen_path,
            &global_opts.wasm_bindgen_version,
        );
        let wasm_opt = Tool::new(
            ToolType::WasmOpt,
            &global_opts.wasm_opt_path,
            &global_opts.wasm_opt_version,
        );

        // Get the statuses of all the tools
        let wb_status = wasm_bindgen.get_status(&target, &lockfile)?;
        let wo_status = wasm_opt.get_status(&target, &lockfile)?;
        // Figure out if everything is present
        // This is the only case in which we don't have to start the spinner
        if let (ToolStatus::Available(wb_path), ToolStatus::Available(wo_path)) =
            (&wb_status, &wo_status)
        {
            Ok(Tools {
                cargo_engine: global_opts.cargo_engine_path.clone(),
                cargo_browser: global_opts.cargo_browser_path.clone(),
                wasm_bindgen: wb_path.to_string(),
                wasm_opt: wo_path.to_string(),
            })
        } else {
            // We need to install some things, which may take some time
            let spinner_msg = format!("{} Installing external tools", INSTALLING);
            let spinner = cfg_spinner(ProgressBar::new_spinner(), &spinner_msg);

            // Install all the tools in parallel
            // These functions sanity-check their statuses, so we don't need to worry about
            // installing unnecessarily
            let res = try_join(
                wasm_bindgen.install(wb_status, &target),
                wasm_opt.install(wo_status, &target),
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
                cargo_engine: global_opts.cargo_engine_path.clone(),
                cargo_browser: global_opts.cargo_browser_path.clone(),
                wasm_bindgen: paths.0,
                wasm_opt: paths.1,
            })
        }
    }
}

/// The data we need about an external tool to be able to install it.
///
/// This does not contain data about arguments to these tools, that's passed
/// through the global options to the CLI directly (with defaults hardcoded).
pub struct Tool {
    /// The name of the tool. This will also be used for the name of the
    /// directory in `dist/tools/` in which the tool is stored (with the
    /// version number added on).
    pub name: String,
    /// A path provided by the user to the tool. If this is present, then the
    /// tool won't be installed, we'll use what the iuser has given us
    /// instead.
    pub user_given_path: Option<String>,
    /// A specific version number provided by the user. By default, the latest
    /// version is used.
    pub user_given_version: Option<String>,
    /// The path to the binary within the directory that is extracted from the
    /// downloaded archive.
    pub final_path: String,
    /// The name of the GitHub repo from which the tool can be downloaded.
    pub gh_repo: String,
    /// The name of the directory that will be extracted. This should contain
    /// `%version` to be replaced with the actual version and/or
    /// `%artifact_name` to be replaced with that.
    pub extracted_dir_name: String,
    /// The actual type of the tool. (All tools have the same data.)
    pub tool_type: ToolType,
}
impl Tool {
    /// Creates a new instance of this `struct`.
    pub fn new(
        tool_type: ToolType,
        user_given_path: &Option<String>,
        user_given_version: &Option<String>,
    ) -> Self {
        // Correct the final path for Windows (on which it'll have a `.exe` extension)
        #[cfg(unix)]
        let final_path = tool_type.final_path();
        #[cfg(windows)]
        let final_path = tool_type.final_path() + ".exe";

        Self {
            name: tool_type.name(),
            user_given_path: user_given_path.to_owned(),
            user_given_version: user_given_version.to_owned(),
            final_path,
            gh_repo: tool_type.gh_repo(),
            extracted_dir_name: tool_type.extracted_dir_name(),
            tool_type,
        }
    }
    /// Gets the name of the artifact to download based on the tool data and the
    /// version to download. Note that the version provided here entirely
    /// overrides anything the user might have provided.
    ///
    /// If no precompiled binary is expected to be available for the current
    /// platform, this will return `None`.
    fn get_artifact_name(&self, version: &str) -> Option<String> {
        match &self.tool_type {
            // --- `wasm-bindgen` ---
            // Linux
            ToolType::WasmBindgen if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") => {
                Some("wasm-bindgen-%version-x86_64-unknown-linux-musl")
            }
            // MacOS (incl. Apple Silicon)
            ToolType::WasmBindgen
                if cfg!(target_os = "macos")
                    && (cfg!(target_arch = "x86_64") || cfg!(target_arch = "aarch64")) =>
            {
                Some("wasm-bindgen-%version-x86_64-apple-darwin")
            }
            // Windows
            ToolType::WasmBindgen
                if cfg!(target_os = "windows") && cfg!(target_arch = "x86_64") =>
            {
                Some("wasm-bindgen-%version-x86_64-pc-windows-msvc")
            }
            ToolType::WasmBindgen => None,
            // --- `wasm-opt` ---
            // Linux
            ToolType::WasmOpt if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") => {
                Some("binaryen-%version-x86_64-linux")
            }
            // MacOS (Intel)
            ToolType::WasmOpt if cfg!(target_os = "macos") && cfg!(target_arch = "x86_64") => {
                Some("binaryen-%version-x86_64-macos")
            }
            // MacOS (Apple Silicon)
            ToolType::WasmOpt if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") => {
                Some("binaryen-%version-arm64-macos")
            }
            // Windows
            ToolType::WasmOpt if cfg!(target_os = "windows") && cfg!(target_arch = "x86_64") => {
                Some("binaryen-%version-x86_64-windows")
            }
            ToolType::WasmOpt => None,
        }
        .map(|s| s.replace("%version", version))
    }
    /// Gets the path to the already-installed version of the tool to use. This
    /// should take the full path to `dist/tools/`. This will automatically
    /// handle whether or not to install a new version, use a version already
    /// installed globally on the user's system, etc. If this returns
    /// `ToolStatus::NeedsInstall`, we can be sure that there are binaries
    /// available, and the same if it returns `ToolStatus::NeedsLatestInstall`.
    pub fn get_status(
        &self,
        target: &Path,
        lockfile: &Lockfile,
    ) -> Result<ToolStatus, InstallError> {
        // The status information will be incomplete from this first pass
        let initial_status = {
            // If there's a directory that matches with a given user version, we'll use it.
            // If not, we'll use the latest version. Only if there are no
            // installed versions available will this return `None`, or if the user wants a
            // specific one that doesn't exist.

            // If the user has given us a path, that overrides everything
            if let Some(path) = &self.user_given_path {
                Ok(ToolStatus::Available(path.to_string()))
            } else {
                // If they've given us a version, we'll check if that directory exists (we don't
                // care about any others)
                if let Some(version) = &self.user_given_version {
                    // If the user wants the latest version, just force an update
                    if version == "latest" {
                        Ok(ToolStatus::NeedsLatestInstall)
                    } else {
                        let expected_path = target.join(format!("{}-{}", self.name, version));
                        Ok(if fs::metadata(&expected_path).is_ok() {
                            ToolStatus::Available(
                                expected_path
                                    .join(&self.final_path)
                                    .to_string_lossy()
                                    .to_string(),
                            )
                        } else {
                            ToolStatus::NeedsInstall {
                                version: version.to_string(),
                                // This will be filled in on the second pass-through
                                artifact_name: String::new(),
                            }
                        })
                    }
                } else {
                    // We have no further information from the user, so we'll use whatever matches
                    // the user's `Cargo.lock`, or, if they haven't specified anything, we'll try
                    // the latest version.
                    // Either way, we need to know what we've got installed already by walking the
                    // directory.
                    let mut versions: Vec<String> = Vec::new();
                    for entry in fs::read_dir(target)
                        .map_err(|err| InstallError::ReadToolsDirFailed { source: err })?
                    {
                        let entry = entry
                            .map_err(|err| InstallError::ReadToolsDirFailed { source: err })?;
                        let dir_name = entry.file_name().to_string_lossy().to_string();
                        if dir_name.starts_with(&self.name) {
                            let dir_name_ref = dir_name.to_string();
                            // Valid directory names are of the form `<tool-name>-<tool-version>`
                            let version = dir_name
                                .strip_prefix(&format!("{}-", self.name))
                                .ok_or(InstallError::InvalidToolsDirName { name: dir_name_ref })?;
                            versions.push(version.to_string());
                        }
                    }
                    // Now order those from most recent to least recent
                    versions.sort();
                    let versions = versions.into_iter().rev().collect::<Vec<String>>();

                    // Now figure out what would match the current setup by checking `Cargo.lock`
                    // (it's entirely possible that there are multiple versions
                    // of `wasm-bindgen` in here, but that would be the user's problem).
                    // It doesn't matter that we do this erroneously for other tools, since they'll
                    // just return `None`.
                    match self.get_pkg_version_from_lockfile(lockfile)? {
                        Some(version) => {
                            if versions.contains(&version) {
                                let path_to_version = target
                                    .join(format!("{}-{}/{}", self.name, version, self.final_path));
                                Ok(ToolStatus::Available(
                                    path_to_version.to_string_lossy().to_string(),
                                ))
                            } else {
                                Ok(ToolStatus::NeedsInstall {
                                    version,
                                    // This will be filled in on the second pass-through
                                    artifact_name: String::new(),
                                })
                            }
                        }
                        // There's nothing in the lockfile, so we'll go with the latest we have
                        // installed
                        None => {
                            // If there are any at all, pick the first one
                            if !versions.is_empty() {
                                let latest_available_version = &versions[0];
                                // We know the directory for this version had a valid name, so we
                                // can determine exactly where it
                                // was
                                let path_to_latest_version = target.join(format!(
                                    "{}-{}/{}",
                                    self.name, latest_available_version, self.final_path
                                ));
                                Ok(ToolStatus::Available(
                                    path_to_latest_version.to_string_lossy().to_string(),
                                ))
                            } else {
                                // We don't check the latest version here because we haven't started
                                // the spinner yet
                                Ok(ToolStatus::NeedsLatestInstall)
                            }
                        }
                    }
                }
            }
        }?;
        // If we're considering installing something, we should make sure that there are
        // actually precompiled binaries available for this platform (if there
        // aren't, then we'll try to fall back on anything the user has installed
        // locally, and if they have nothing, an error will be returned)
        match initial_status {
            ToolStatus::Available(path) => Ok(ToolStatus::Available(path)),
            ToolStatus::NeedsInstall { version, .. } => {
                // This will be `None` if there are no precompiled binaries available
                let artifact_name = self.get_artifact_name(&version);
                if let Some(artifact_name) = artifact_name {
                    // There are precompiled binaries available, which we prefer to preinstalled
                    // global ones
                    Ok(ToolStatus::NeedsInstall {
                        version,
                        artifact_name,
                    })
                } else {
                    // If the user has something, we're good, but, if not, we have to fail
                    let preinstalled_path = self.get_path_to_preinstalled()?;
                    // We've got something, but it might not be the right version, so if the user
                    // told us to use a specific version, we should warn them
                    if self.user_given_version.is_some() {
                        eprintln!("[WARNING]: You requested a specific version, but no precompiled binaries of '{}' are available for your platform, so the version already installed on your system is being used. This may not correspond to the requested version!", self.name);
                    }
                    Ok(ToolStatus::Available(preinstalled_path))
                }
            }
            ToolStatus::NeedsLatestInstall => {
                // To get the proper artifact name for this, we would have to request the latest
                // version, but we don't want to do that until a CLI spinner has
                // been started, which is after the execution of this function (so we just use a
                // dummy version to check if there would be binaries)
                // This will be `None` if there are no precompiled binaries available
                let artifact_name = self.get_artifact_name("dummy");
                if artifact_name.is_some() {
                    // There are precompiled binaries available, which we prefer to preinstalled
                    // global ones
                    Ok(ToolStatus::NeedsLatestInstall)
                } else {
                    // If the user has something, we're good, but, if not, we have to fail
                    let preinstalled_path = self.get_path_to_preinstalled()?;
                    // The user can't have requested a specific version if we're in the market for
                    // the latest one, so we can wrap up here
                    Ok(ToolStatus::Available(preinstalled_path))
                }
            }
        }
    }
    /// Gets the latest version for this tool from its GitHub repository. One
    /// should only bother executing this if we know there are precompiled
    /// binaries for this platform.
    pub async fn get_latest_version(&self) -> Result<String, InstallError> {
        let json = Client::new()
            .get(&format!(
                "https://api.github.com/repos/{}/releases/latest",
                self.gh_repo
            ))
            // TODO Is this compliant with GH's ToS?
            .header("User-Agent", "perseus-cli")
            .send()
            .await
            .map_err(|err| InstallError::GetLatestToolVersionFailed {
                source: err,
                tool: self.name.to_string(),
            })?
            .json::<serde_json::Value>()
            .await
            .map_err(|err| InstallError::GetLatestToolVersionFailed {
                source: err,
                tool: self.name.to_string(),
            })?;
        let latest_version =
            json.get("name")
                .ok_or_else(|| InstallError::ParseToolVersionFailed {
                    tool: self.name.to_string(),
                })?;

        Ok(latest_version
            .as_str()
            .ok_or_else(|| InstallError::ParseToolVersionFailed {
                tool: self.name.to_string(),
            })?
            .to_string())
    }
    /// Installs the tool, taking the predetermined status as an argument to
    /// avoid installing if the tool is actually already available, since
    /// this method will be called on all tools if even one
    /// is not available.
    pub async fn install(&self, status: ToolStatus, target: &Path) -> Result<String, InstallError> {
        // Do a sanity check to prevent installing something that already exists
        match status {
            ToolStatus::Available(path) => Ok(path),
            ToolStatus::NeedsInstall {
                version,
                artifact_name,
            } => self.install_version(&version, &artifact_name, target).await,
            ToolStatus::NeedsLatestInstall => {
                let latest_version = self.get_latest_version().await?;
                // We *do* know at this point that there do exist precompiled binaries
                let artifact_name = self.get_artifact_name(&latest_version).unwrap();
                self.install_version(&latest_version, &artifact_name, target)
                    .await
            }
        }
    }
    /// Checks if the user already has this tool installed. This should only be
    /// called if there are no precompiled binaries available of this tool
    /// for the user's platform. In that case, this will also warn the user
    /// if they've asked for a specific version that their own version might not
    /// be that (we don't bother trying to parse the version of their
    /// installed program).
    ///
    /// If there's nothing the user has installed, then this will return an
    /// error, and hence it should only be called after all other options
    /// have been exhausted.
    fn get_path_to_preinstalled(&self) -> Result<String, InstallError> {
        #[cfg(unix)]
        let shell_exec = "sh";
        #[cfg(windows)]
        let shell_exec = "powershell";
        #[cfg(unix)]
        let shell_param = "-c";
        #[cfg(windows)]
        let shell_param = "-command";

        let check_cmd = format!("{} --version", self.name); // Not exactly bulletproof, but it works!
        let res = Command::new(shell_exec)
            .args([shell_param, &check_cmd])
            .output();
        if let Err(err) = res {
            // Unlike `wasm-pack`, we don't try to install with `cargo install`, because
            // that's a little pointless to me (the user will still have to get
            // `wasm-opt` somehow...)
            //
            // TODO Installation script that can build manually on any platform
            Err(InstallError::ExternalToolUnavailable {
                tool: self.name.to_string(),
                source: err,
            })
        } else {
            // It works, so we don't need to install anything
            Ok(self.name.to_string())
        }
    }
    /// Installs the given version of the tool, returning the path to the final
    /// binary.
    async fn install_version(
        &self,
        version: &str,
        artifact_name: &str,
        target: &Path,
    ) -> Result<String, InstallError> {
        let url = format!(
            "https://github.com/{gh_repo}/releases/download/{version}/{artifact_name}.tar.gz",
            artifact_name = artifact_name,
            version = version,
            gh_repo = self.gh_repo
        );
        let dir_name = format!("{}-{}", self.name, version);
        let tar_name = format!("{}.tar.gz", &dir_name);
        let dir_path = target.join(&dir_name);
        let tar_path = target.join(&tar_name);

        // Deal with placeholders in the name to expect from the extracted directory
        let extracted_dir_name = self
            .extracted_dir_name
            .replace("%artifact_name", artifact_name)
            .replace("%version", version);

        // Download the tarball (source https://github.com/seanmonstar/reqwest/issues/1266#issuecomment-1106187437)
        // We do this by chunking to minimize memory usage (we're downloading fairly
        // large files!)
        let mut res = Client::new().get(url).send().await.map_err(|err| {
            InstallError::BinaryDownloadRequestFailed {
                source: err,
                tool: self.name.to_string(),
            }
        })?;
        let mut file = tokio::fs::File::create(&tar_path)
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
        let tar_gz = File::open(&tar_path)
            .map_err(|err| InstallError::CreateToolExtractDestFailed { source: err })?;
        let mut archive = Archive::new(GzDecoder::new(tar_gz));
        // We'll extract straight into `dist/tools/` and then rename the resulting
        // directory
        archive
            .unpack(target)
            .map_err(|err| InstallError::ToolExtractFailed {
                source: err,
                tool: self.name.to_string(),
            })?;

        // Now delete the original archive file
        fs::remove_file(&tar_path)
            .map_err(|err| InstallError::ArchiveDeletionFailed { source: err })?;
        // Finally, rename the extracted directory
        fs::rename(
            target.join(extracted_dir_name), // We extracted into the root of the target
            &dir_path,
        )
        .map_err(|err| InstallError::DirRenameFailed { source: err })?;

        // Return the path inside the directory we extracted
        Ok(dir_path
            .join(&self.final_path)
            .to_str()
            .unwrap()
            .to_string())
    }
    /// Gets the version of a specific package in `Cargo.lock`, assuming it has
    /// already been generated.
    fn get_pkg_version_from_lockfile(
        &self,
        lockfile: &Lockfile,
    ) -> Result<Option<String>, InstallError> {
        let version = lockfile
            .packages
            .iter()
            .find(|p| p.name.as_str() == self.name)
            .map(|p| p.version.to_string());
        Ok(version)
    }
}

/// A tool's status on-system.
pub enum ToolStatus {
    /// The tool needs to be installed.
    NeedsInstall {
        version: String,
        artifact_name: String,
    },
    /// The latest version of the tool needs to be determined from its repo and
    /// then installed.
    NeedsLatestInstall,
    /// The tool is already available at the attached path.
    Available(String),
}

/// The types of tools we can install.
pub enum ToolType {
    /// The `wasm-bindgen` CLI, used for producing final Wasm and JS artifacts.
    WasmBindgen,
    /// Binaryen's `wasm-opt` CLI, used for optimizing Wasm in release builds
    /// to achieve significant savings in bundle sizes.
    WasmOpt,
}
impl ToolType {
    /// Gets the tool's name.
    pub fn name(&self) -> String {
        match &self {
            Self::WasmBindgen => "wasm-bindgen",
            Self::WasmOpt => "wasm-opt",
        }
        .to_string()
    }
    /// Get's the path to the tool's binary inside the extracted directory
    /// from the downloaded archive.
    ///
    /// Note that the return value of this function is uncorrected for Windows.
    pub fn final_path(&self) -> String {
        match &self {
            Self::WasmBindgen => "wasm-bindgen",
            Self::WasmOpt => "bin/wasm-opt",
        }
        .to_string()
    }
    /// Gets the GitHub repo to install this tool from.
    pub fn gh_repo(&self) -> String {
        match &self {
            Self::WasmBindgen => "rustwasm/wasm-bindgen",
            Self::WasmOpt => "WebAssembly/binaryen",
        }
        .to_string()
    }
    /// Gets the name of the directory that will be extracted from the
    /// downloaded archive for this tool.
    ///
    /// This will return a `String` that might include placeholders for the
    /// version and downloaded artifact name.
    pub fn extracted_dir_name(&self) -> String {
        match &self {
            Self::WasmBindgen => "%artifact_name",
            Self::WasmOpt => "binaryen-%version",
        }
        .to_string()
    }
}
