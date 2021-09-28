// This file contains the logic for a universal interface to read and write to static files
// At simplest, this is just a filesystem interface, but it's more likely to be a CMS in production
// This has its own error management logic because the user may implement it separately

use thiserror::Error;

/// Errors that can occur in a config manager.
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum ConfigManagerError {
    #[error("asset '{name}' not found")]
    NotFound { name: String },
    #[error("asset '{name}' couldn't be read")]
    ReadFailed {
        name: String,
        #[source]
        source: Box<dyn std::error::Error>,
    },
    #[error("asset '{name}' couldn't be written")]
    WriteFailed {
        name: String,
        #[source]
        source: Box<dyn std::error::Error>,
    },
}

use std::fs;

/// A trait for systems that manage where to put configuration files. At simplest, we'll just write them to static files, but they're
/// more likely to be stored on a CMS.
#[async_trait::async_trait]
pub trait ConfigManager: Clone {
    /// Reads data from the named asset.
    async fn read(&self, name: &str) -> Result<String, ConfigManagerError>;
    /// Writes data to the named asset. This will create a new asset if one doesn't exist already.
    async fn write(&self, name: &str, content: &str) -> Result<(), ConfigManagerError>;
}

/// The default config manager. This will store static files in the specified location on disk. This should be suitable for nearly all
/// development and serverful use-cases. Serverless is another matter though (more development needs to be done).
///
/// Note: the `.write()` methods on this implementation will create any missing parent directories automatically.
#[derive(Clone)]
pub struct FsConfigManager {
    root_path: String,
}
impl FsConfigManager {
    /// Creates a new filesystem configuration manager. You should provide a path like `/dist` here.
    pub fn new(root_path: String) -> Self {
        Self { root_path }
    }
}
#[async_trait::async_trait]
impl ConfigManager for FsConfigManager {
    async fn read(&self, name: &str) -> Result<String, ConfigManagerError> {
        let asset_path = format!("{}/{}", self.root_path, name);
        match fs::metadata(&asset_path) {
            Ok(_) => {
                fs::read_to_string(&asset_path).map_err(|err| ConfigManagerError::ReadFailed {
                    name: asset_path,
                    source: err.into(),
                })
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                return Err(ConfigManagerError::NotFound { name: asset_path })
            }
            Err(err) => {
                return Err(ConfigManagerError::ReadFailed {
                    name: asset_path,
                    source: err.into(),
                })
            }
        }
    }
    // This creates a directory structure as necessary
    async fn write(&self, name: &str, content: &str) -> Result<(), ConfigManagerError> {
        let asset_path = format!("{}/{}", self.root_path, name);
        let mut dir_tree: Vec<&str> = asset_path.split('/').collect();
        dir_tree.pop();

        fs::create_dir_all(dir_tree.join("/")).map_err(|err| ConfigManagerError::WriteFailed {
            name: asset_path.clone(),
            source: err.into(),
        })?;
        fs::write(&asset_path, content).map_err(|err| ConfigManagerError::WriteFailed {
            name: asset_path,
            source: err.into(),
        })
    }
}
