#![allow(missing_docs)]

use thiserror::Error;

/// All errors that can be returned by the CLI.
#[derive(Error, Debug)]
pub enum Error {
    #[error("couldn't find your system shell (sh on unix and powershell on windows), which is required to run the perseus cli")]
    ShellNotPresent {
        #[source]
        source: std::io::Error,
    },
    #[error("couldn't find `cargo`, which is a dependency of this cli (set 'PERSEUS_CARGO_PATH' to another location if you've installed it elsewhere)")]
    CargoNotPresent {
        #[source]
        source: std::io::Error,
    },
    #[error(
        "couldn't install `wasm32-unknown-unknown` target (do you have an internet connection?)"
    )]
    RustupTargetAddFailed { code: i32 },
    #[error("couldn't get current directory (have you just deleted it?)")]
    CurrentDirUnavailable {
        #[source]
        source: std::io::Error,
    },
    #[error(transparent)]
    ExecutionError(#[from] ExecutionError),
    #[error(transparent)]
    ExportError(#[from] ExportError),
    #[error(transparent)]
    DeployError(#[from] DeployError),
    #[error(transparent)]
    WatchError(#[from] WatchError),
    #[error(transparent)]
    InitError(#[from] InitError),
    #[error(transparent)]
    NewError(#[from] NewError),
    #[error(transparent)]
    InstallError(#[from] InstallError),
}

/// Errors that can occur while attempting to execute a Perseus app with
/// `build`/`serve` (export errors are separate).
#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("failed to parse command for execution")]
    CmdParseFailed {
        cmd: String,
        // Right now, this can only be an unmatched quote error
        #[source]
        source: shell_words::ParseError,
    },
    #[error("couldn't execute command '{cmd}' (this doesn't mean it threw an error, it means it couldn't be run at all)")]
    CmdExecFailed {
        cmd: String,
        #[source]
        source: std::io::Error,
    },
    #[error("couldn't execute command because it prematurely stopped reporting (if this persists, please report it as a bug)")]
    NextStdoutLineNone,
    #[error("couldn't get path to server executable (if this persists, please report it as a bug, especially if you've just updated `cargo`)")]
    GetServerExecutableFailed {
        #[source]
        source: serde_json::Error,
    },
    #[error("couldn't get path to server executable (if this persists, try `perseus clean`)")]
    GetServerExecutableFailedSimple,
    #[error("expected second-last message from Cargo to contain server executable path, none existed (too few messages) (report this as a bug if it persists)")]
    ServerExecutableMsgNotFound,
    #[error("couldn't parse server executable path from Cargo (report this as a bug if it persists): {err}")]
    ParseServerExecutableFailed { err: String },
    #[error("couldn't remove and replace internal build artifact directory '{target:?}' (run `perseus clean` if this persists)")]
    RemoveArtifactsFailed {
        target: Option<String>,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to wait on thread (please report this as a bug if it persists)")]
    ThreadWaitFailed,
    #[error("value in `PORT` environment variable couldn't be parsed as a number")]
    PortNotNumber {
        #[source]
        source: std::num::ParseIntError,
    },
    #[error("couldn't parse `Cargo.toml` (are you running in the right directory?)")]
    GetManifestFailed {
        #[source]
        source: cargo_toml::Error,
    },
    #[error("couldn't get crate name from `[package]` section of `Cargo.toml` (are you running in the right directory?)")]
    CrateNameNotPresentInManifest,
    #[error("couldn't create directory for distribution artifacts (do you have the necessary permissions?)")]
    CreateDistFailed {
        #[source]
        source: std::io::Error,
    },
}

/// Errors that can occur while running `perseus export`.
#[derive(Error, Debug)]
pub enum ExportError {
    #[error("couldn't create directory structure necessary for exporting (do you have the necessary permissions?)")]
    DirStructureCreationFailed {
        #[source]
        source: std::io::Error,
    },
    #[error("couldn't copy asset from '{from}' to '{to}' for exporting")]
    MoveAssetFailed {
        to: String,
        from: String,
        #[source]
        source: std::io::Error,
    },
    #[error("couldn't copy directory from '{from}' to '{to}' for exporting")]
    MoveDirFailed {
        to: String,
        from: String,
        #[source]
        source: fs_extra::error::Error,
    },
    // We need to execute in exports
    #[error(transparent)]
    ExecutionError(#[from] ExecutionError),
}

/// Errors that can occur while running `perseus deploy`.
#[derive(Error, Debug)]
pub enum DeployError {
    #[error("couldn't copy exported static files from '{from:?}' to '{to}'")]
    MoveExportDirFailed {
        to: String,
        from: String,
        #[source]
        source: fs_extra::error::Error,
    },
    #[error("couldn't delete and recreate output directory '{path}'")]
    ReplaceOutputDirFailed {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("couldn't copy file from '{from}' to '{to}' for deployment packaging")]
    MoveAssetFailed {
        to: String,
        from: String,
        #[source]
        source: std::io::Error,
    },
    #[error("couldn't copy directory from '{from}' to '{to}' for deployment packaging")]
    MoveDirFailed {
        to: String,
        from: String,
        #[source]
        source: fs_extra::error::Error,
    },
    #[error("couldn't read contents of export directory '{path}' for packaging (if this persists, try `perseus clean`)")]
    ReadExportDirFailed {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("couldn't create distribution artifacts directory for deployment (if this persists, try `perseus clean`)")]
    CreateDistDirFailed {
        #[source]
        source: std::io::Error,
    },

    #[error("failed to minify javascript bundle (this is probably an upstream bug, re-try with `--no-minify-js`)")]
    MinifyError {
        #[source]
        source: minify_js::Error,
    },

    #[error("minified js was not utf-8 (this is a bug, re-try with `--no-minify-js` for now)")]
    MinifyNotUtf8 {
        #[source]
        source: std::string::FromUtf8Error,
    },
    #[error("failed to read unminified js (if this persists, try `perseus clean`)")]
    ReadUnminifiedJsFailed {
        #[source]
        source: std::io::Error,
    },
    #[error("failed to write minified js (if this persists, try `perseus clean`)")]
    WriteMinifiedJsFailed {
        #[source]
        source: std::io::Error,
    },
}

#[derive(Error, Debug)]
pub enum WatchError {
    #[error("couldn't set up a file watcher, try re-running this command")]
    WatcherSetupFailed {
        #[source]
        source: notify::Error,
    },
    #[error("couldn't read your current directory to watch files, do you have the necessary permissions?")]
    ReadCurrentDirFailed {
        #[source]
        source: std::io::Error,
    },
    #[error("couldn't read entry in your current directory, try re-running this command")]
    ReadDirEntryFailed {
        #[source]
        source: std::io::Error,
    },
    #[error("the file/folder '{filename}' could not be resolved to an absolute path (does the file/folder exist?)")]
    WatchFileNotResolved {
        filename: String,
        source: std::io::Error,
    },
    #[error("couldn't watch file at '{filename}', try re-running the command")]
    WatchFileFailed {
        filename: String,
        #[source]
        source: notify::Error,
    },
    #[error("couldn't unwatch file at '{filename}', try re-running the command")]
    UnwatchFileFailed {
        filename: String,
        #[source]
        source: notify::Error,
    },
    #[error("an error occurred while watching files")]
    WatcherError {
        #[source]
        source: std::sync::mpsc::RecvError,
    },
    #[error("couldn't spawn a child process to build your app in watcher mode")]
    SpawnSelfFailed {
        #[source]
        source: std::io::Error,
    },
    #[error("couldn't get the path to the cli's executable, try re-running the command")]
    GetSelfPathFailed {
        #[source]
        source: std::io::Error,
    },
    #[error("couldn't read an entry in the targeted custom directory for watching, do you have the necessary permissions?")]
    ReadCustomDirEntryFailed {
        #[source]
        source: walkdir::Error,
    },
}

#[derive(Error, Debug)]
pub enum InitError {
    #[error("couldn't create directory structure for new project, do you have the necessary permissions?")]
    CreateDirStructureFailed {
        #[source]
        source: std::io::Error,
    },
    #[error("couldn't create file '{filename}' for project initialization")]
    CreateInitFileFailed {
        #[source]
        source: std::io::Error,
        filename: String,
    },
}

#[derive(Error, Debug)]
pub enum NewError {
    // The `new` command calls the `init` command in effect
    #[error(transparent)]
    InitError(#[from] InitError),
    #[error("couldn't create directory for new project, do you have the necessary permissions?")]
    CreateProjectDirFailed {
        #[source]
        source: std::io::Error,
    },
    #[error("fetching the custom initialization template failed")]
    GetCustomInitFailed {
        #[source]
        source: ExecutionError,
    },
    #[error(
        "fetching the custom initialization template returned non-zero exit code ({exit_code})"
    )]
    GetCustomInitNonZeroExitCode { exit_code: i32 },
    #[error(
        "couldn't remove git internals at '{target_dir:?}' for custom initialization template"
    )]
    RemoveCustomInitGitFailed {
        target_dir: Option<String>,
        #[source]
        source: std::io::Error,
    },
}

#[derive(Error, Debug)]
pub enum InstallError {
    #[error("couldn't create `dist/tools/` for external dependency installation")]
    CreateToolsDirFailed {
        #[source]
        source: std::io::Error,
    },
    // This will only be called after we've checked if the user has already installed the tool
    // themselves
    #[error("couldn't install '{tool}', as there are no precompiled binaries for your platform and it's not currently installed; please install this tool manually (see https://framesurge.sh/perseus/en-US/docs/0.4.x/reference/faq)")]
    ExternalToolUnavailable {
        tool: String,
        // This is from checking if the tool is installed at the usual path
        #[source]
        source: std::io::Error,
    },
    #[error("couldn't download binary for '{tool}' (do you have an internet connection?)")]
    BinaryDownloadRequestFailed {
        tool: String,
        #[source]
        source: reqwest::Error,
    },
    #[error(
        "couldn't create destination for tool download (do you have the necessary permissions?)"
    )]
    CreateToolDownloadDestFailed {
        #[source]
        source: tokio::io::Error,
    },
    #[error("couldn't chunk tool download properly (do you have an internet connection?)")]
    ChunkBinaryDownloadFailed {
        #[source]
        source: reqwest::Error,
    },
    #[error(
        "couldn't write downloaded chunk of external tool (do you have the necessary permissions?)"
    )]
    WriteBinaryDownloadChunkFailed {
        #[source]
        source: tokio::io::Error,
    },
    #[error("couldn't determine latest version of '{tool}' (do you have an internet connection?)")]
    GetLatestToolVersionFailed {
        tool: String,
        #[source]
        source: reqwest::Error,
    },
    #[error("couldn't parse latest version of '{tool}' (if this error persists, please report it as a bug)")]
    ParseToolVersionFailed { tool: String },
    #[error("couldn't create destination for extraction of external tool (do you have the necessary permissions?)")]
    CreateToolExtractDestFailed {
        #[source]
        source: std::io::Error,
    },
    #[error("couldn't extract '{tool}' (do you have the necessary permissions?)")]
    ToolExtractFailed {
        tool: String,
        #[source]
        source: std::io::Error,
    },
    #[error("couldn't delete archive from tool deletion")]
    ArchiveDeletionFailed {
        #[source]
        source: std::io::Error,
    },
    #[error("couldn't rename directory for external tool binaries")]
    DirRenameFailed {
        #[source]
        source: std::io::Error,
    },
    #[error("couldn't read `dist/tools/` to determine which tool versions were installed (do you have the necessary permissions?)")]
    ReadToolsDirFailed {
        #[source]
        source: std::io::Error,
    },
    #[error("directory found in `dist/tools/` with invalid name (running `perseus clean` should resolve this)")]
    InvalidToolsDirName { name: String },
    #[error("generating `Cargo.lock` returned non-zero exit code")]
    LockfileGenerationNonZero { code: i32 },
    #[error("couldn't generate `Cargo.lock`")]
    LockfileGenerationFailed {
        #[source]
        source: ExecutionError,
    },
    #[error("couldn't fetch metadata for current crate (have you run `perseus init` yet?)")]
    MetadataFailed {
        #[source]
        source: cargo_metadata::Error,
    },
    #[error("couldn't load `Cargo.lock` from workspace root")]
    LockfileLoadFailed {
        #[source]
        source: cargo_lock::Error,
    },
}
