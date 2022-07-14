#![allow(missing_docs)]

use thiserror::Error;

/// All errors that can be returned by the CLI.
#[derive(Error, Debug)]
pub enum Error {
    #[error("prerequisite command execution failed for prerequisite '{cmd}' (set '{env_var}' to another location if you've installed it elsewhere)")]
    PrereqNotPresent {
        cmd: String,
        env_var: String,
        #[source]
        source: std::io::Error,
    },
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
}

/// Errors that can occur while attempting to execute a Perseus app with
/// `build`/`serve` (export errors are separate).
#[derive(Error, Debug)]
pub enum ExecutionError {
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
    #[error("expected second-last message from Cargo to contain server executable path, none existed (too few messages) (report this as a bug if it persists)")]
    ServerExectutableMsgNotFound,
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
    #[error("couldn't get path to server executable (if this persists, try `perseus clean`)")]
    GetServerExecutableFailed,
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
    #[error("couldn't watch file at '{filename}', try re-running the command")]
    WatchFileFailed {
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
