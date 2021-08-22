// This file contains the logic for a universal interface to read and write to static files
// At simplest, this is just a filesystem interface, but it's more likely to be a CMS in production
// This has its own error management logic because the user may implement it separately

use error_chain::{bail, error_chain};
use std::fs;

// This has no foreign links because everything to do with config management should be isolated and generic
error_chain! {
    errors {
        /// For when data wasn't found.
        NotFound(name: String) {
            description("data not found")
            display("data with name '{}' not found", name)
        }
        /// For when data couldn't be read for some generic reason.
        ReadFailed(name: String, err: String) {
            description("data couldn't be read")
            display("data with name '{}' couldn't be read, error was '{}'", name, err)
        }
        /// For when data couldn't be written for some generic reason.
        WriteFailed(name: String, err: String) {
            description("data couldn't be written")
            display("data with name '{}' couldn't be written, error was '{}'", name, err)
        }
    }
}

/// A trait for systems that manage where to put configuration files. At simplest, we'll just write them to static files, but they're
/// more likely to be stored on a CMS.
#[async_trait::async_trait]
pub trait ConfigManager: Clone {
    /// Reads data from the named asset.
    async fn read(&self, name: &str) -> Result<String>;
    /// Writes data to the named asset. This will create a new asset if one doesn't exist already.
    async fn write(&self, name: &str, content: &str) -> Result<()>;
}

#[derive(Default, Clone)]
pub struct FsConfigManager {}
impl FsConfigManager {
    /// Creates a new filesystem configuration manager. This function only exists to preserve the API surface of the trait.
    pub fn new() -> Self {
        Self::default()
    }
}
#[async_trait::async_trait]
impl ConfigManager for FsConfigManager {
    async fn read(&self, name: &str) -> Result<String> {
        match fs::metadata(name) {
            Ok(_) => fs::read_to_string(name)
                .map_err(|err| ErrorKind::ReadFailed(name.to_string(), err.to_string()).into()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                bail!(ErrorKind::NotFound(name.to_string()))
            }
            Err(err) => bail!(ErrorKind::ReadFailed(name.to_string(), err.to_string())),
        }
    }
    async fn write(&self, name: &str, content: &str) -> Result<()> {
        fs::write(name, content)
            .map_err(|err| ErrorKind::WriteFailed(name.to_string(), err.to_string()).into())
    }
}
