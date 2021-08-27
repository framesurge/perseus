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
            display("Couldn't extract internal subcrates to '{:?}'. You may not have the permissions necessary to write to this location, or the directory disappeared out from under the CLI. Error was: '{}'.", target_dir, err)
        }
        /// For when updating the user's .gitignore fails
        GitignoreUpdateFailed(err: String) {
            description("updating gitignore failed")
            display("Couldn't update your .gitignore file to ignore the Perseus subcrates. Error was: '{}'. In the meantime, please manually add 'perseus/' to your .gitignore.", err)
        }
    }
}
