#![allow(missing_docs)]

use thiserror::Error;

/// All errors that can be returned by the CLI.
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    PrepError(#[from] PrepError),
    #[error(transparent)]
    ExecutionError(#[from] ExecutionError),
    #[error(transparent)]
    EjectionError(#[from] EjectionError),
    #[error(transparent)]
    ExportError(#[from] ExportError),
    #[error(transparent)]
    DeployError(#[from] DeployError),
    #[error(transparent)]
    WatchError(#[from] WatchError),
}

/// Errors that can occur while preparing.
#[derive(Error, Debug)]
pub enum PrepError {
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
    #[error("couldn't extract internal subcrates to '{target_dir:?}' (do you have the necessary permissions?)")]
    ExtractionFailed {
        target_dir: Option<String>,
        #[source]
        source: std::io::Error,
    },
    #[error("updating gitignore to ignore `.perseus/` failed (`.perseus/` has been automatically deleted)")]
    GitignoreUpdateFailed {
        #[source]
        source: std::io::Error,
    },
    #[error("couldn't update internal manifest file at '{target_dir:?}' (`.perseus/` has been automatically deleted)")]
    ManifestUpdateFailed {
        target_dir: Option<String>,
        #[source]
        source: std::io::Error,
    },
    #[error("couldn't get `Cargo.toml` for your project (have you run `cargo init` yet?)")]
    GetUserManifestFailed {
        #[source]
        source: cargo_toml::Error,
    },
    #[error(
        "your project's `Cargo.toml` doesn't have a `[package]` section (package name is required)"
    )]
    MalformedUserManifest,
    #[error("couldn't remove corrupted `.perseus/` directory as required by previous error (please delete `.perseus/` manually)")]
    RemoveBadDirFailed {
        #[source]
        source: std::io::Error,
    },
}
/// Checks if the given error should cause the CLI to delete the '.perseus/' folder so the user doesn't have something incomplete.
/// When deleting the directory, it should only be deleted if it exists, if not don't worry. If it does and deletion fails, fail like hell.
pub fn err_should_cause_deletion(err: &Error) -> bool {
    matches!(
        err,
        Error::PrepError(
            PrepError::ExtractionFailed { .. }
                | PrepError::GitignoreUpdateFailed { .. }
                | PrepError::ManifestUpdateFailed { .. }
        )
    )
}

/// Errors that can occur while attempting to execute a Perseus app with `build`/`serve` (export errors are separate).
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
    #[error("couldn't move `.perseus/pkg/` to `.perseus/dist/pkg/` (run `perseus clean` if this persists)")]
    MovePkgDirFailed {
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

/// Errors that can occur while ejecting or as a result of doing so.
#[derive(Error, Debug)]
pub enum EjectionError {
    #[error("couldn't remove perseus subcrates from gitignore for ejection")]
    GitignoreUpdateFailed {
        #[source]
        source: std::io::Error,
    },
    #[error("line `.perseus/` to remove not found in `.gitignore`")]
    GitignoreLineNotPresent,
    #[error("couldn't write ejection declaration file (`.perseus/.ejected`), please try again")]
    DeclarationWriteFailed {
        #[source]
        source: std::io::Error,
    },
    #[error("can't clean after ejection unless `--force` is provided (maybe you meant to use `--dist`?)")]
    CleanAfterEject,
    #[error("can't tinker after ejection unless `--force` is provided (ejecting and using plugins can be problematic depending on the plugins used)")]
    TinkerAfterEject,
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
