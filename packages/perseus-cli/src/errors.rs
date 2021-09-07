#![allow(missing_docs)]

pub use error_chain::bail;
use error_chain::error_chain;

// The `error_chain` setup for the whole crate
error_chain! {
    // The custom errors for this crate (very broad)
    errors {
        /// For when executing a prerequisite command fails.
        PrereqFailed(cmd: String, env_var: String, err: String) {
            description("prerequisite command execution failed")
            display("You seem to be missing the prerequisite '{}', which is required for the Perseus CLI to work. If you've installed it at another path, please provide the executable through the '{}' variable. Error was: '{}'.", cmd, env_var, err)
        }
        /// For when the user's curreent directory couldn't be found.
        CurrentDirUnavailable(err: String) {
            description("couldn't get current directory")
            display("Couldn't get your current directory. This is probably an issue with your system configuration. Error was: '{}'.", err)
        }
        /// For when extracting the subcrates failed.
        // The `PathBuf` will be converted to a string, and unwrapping is bad in that context
        ExtractionFailed(target_dir: Option<String>, err: String) {
            description("subcrate extraction failed")
            display("Couldn't extract internal subcrates to '{:?}'. You may not have the permissions necessary to write to this location, or the directory disappeared out from under the CLI. The '.perseus/' directory has been automatically deleted for safety. Error was: '{}'.", target_dir, err)
        }
        /// For when updating the user's .gitignore fails
        GitignoreUpdateFailed(err: String) {
            description("updating gitignore failed")
            display("Couldn't update your .gitignore file to ignore the Perseus subcrates. The '.perseus/' directory has been automatically deleted (necessary further steps not executed). Error was: '{}'.", err)
        }
        /// For when updating relative paths and package names in the manifest failed.
        ManifestUpdateFailed(target: Option<String>, err: String) {
            description("updating manifests failed")
            display("Couldn't update internal manifest file at '{:?}'. If the error persists, make sure you have file write permissions. The '.perseus/' directory has been automatically deleted. Error was: '{}'.", target, err)
        }
        /// For when we can't get the user's `Cargo.toml` file.
        GetUserManifestFailed(err: String) {
            description("reading user manifest failed")
            display("Couldn't read your crate's manifest (Cargo.toml) file. Please make sure this file exists, is valid, and that you're running Perseus in the right directory.The '.perseus/' directory has been automatically deleted. Error was: '{}'.", err)
        }
        /// For when a partially-formed '.perseus/' directory couldn't be removed, but did exist.
        RemoveBadDirFailed(target: Option<String>, err: String) {
            description("removing corrupted '.perseus/' directory failed")
            display("Couldn't remove '.perseus/' directory at '{:?}'. Please remove the '.perseus/' directory manually (particularly if you didn't intentionally run the 'clean' command, that means the directory has been corrupted). Error was: '{}'.", target, err)
        }
        /// For when executing a system command after preparation failed. This shouldn't cause a directory deletion.
        CmdExecFailed(cmd: String, err: String) {
            description("command exeuction failed")
            display("Couldn't execute command '{}'. Error was: '{}'.", cmd, err)
        }
        /// For when watching failes for changes failed.
        WatcherFailed(path: String, err: String) {
            description("watching files failed")
            display("Couldn't watch '{}' for changes. Error was: '{}'.", path, err)
        }
        /// For when the next line of the stdout of a command is `None` when it shouldn't have been.
        NextStdoutLineNone {
            description("next stdout line was None, expected Some(_)")
            display("Executing a command failed because it seemed to stop reporting prmeaturely. If this error persists, you should file a bug report (particularly if you've just upgraded Rust).")
        }
        /// For when getting the path to the built executable for the server from the JSON build output failed.
        GetServerExecutableFailed(err: String) {
            description("getting server executable path failed")
            display("Couldn't get the path to the server executable from `cargo build`. If this problem persists, please report it as a bug (especially if you just updated cargo). Error was: '{}'.", err)
        }
        /// For when getting the path to the built executable for the server from the JSON build output failed.
        PortNotNumber(err: String) {
            description("port in PORT environment variable couldn't be parsed as number")
            display("Couldn't parse 'PORT' environment variable as a number, please check that you've provided the correct value. Error was: '{}'.", err)
        }
        /// For when build artifacts either couldn't be removed or the directory couldn't be recreated.
        RemoveArtifactsFailed(target: Option<String>, err: String) {
            description("reconstituting build artifacts failed")
            display("Couldn't remove and replace '.perseus/dist/static/' directory at '{:?}'. Please try again or run 'perseus clean' if the error persists. Error was: '{}'.", target, err)
        }
    }
}

/// Checks if the given error should cause the CLI to delete the '.perseus/' folder so the user doesn't have something incomplete.
/// When deleting the directory, it should only be deleted if it exists, if not don't worry. If it does and deletion fails, fail like hell.
pub fn err_should_cause_deletion(err: &Error) -> bool {
    matches!(
        err.kind(),
        ErrorKind::ExtractionFailed(_, _)
            | ErrorKind::GitignoreUpdateFailed(_)
            | ErrorKind::ManifestUpdateFailed(_, _)
    )
}
