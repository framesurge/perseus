use crate::errors::*;
use std::fs;

/// An immutable storage system. This wraps filesystem calls in a sensible asynchronous API, allowing abstraction of the base path
/// to a distribution directory or the like. Perseus uses this to store assts created at build time that won't change, which is
/// anything not involved in the *revalidation* or *incremental generation* strategies.
#[derive(Clone)]
pub struct ImmutableStore {
    root_path: String,
}
impl ImmutableStore {
    /// Creates a new immutable store. You should provide a path like `dist/` here.
    pub fn new(root_path: String) -> Self {
        Self { root_path }
    }
    /// Reads the given asset from the filesystem asynchronously.
    pub async fn read(&self, name: &str) -> Result<String, ImmutableStoreError> {
        let asset_path = format!("{}/{}", self.root_path, name);
        match fs::metadata(&asset_path) {
            Ok(_) => {
                fs::read_to_string(&asset_path).map_err(|err| ImmutableStoreError::ReadFailed {
                    name: asset_path,
                    source: err.into(),
                })
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                Err(ImmutableStoreError::NotFound { name: asset_path })
            }
            Err(err) => Err(ImmutableStoreError::ReadFailed {
                name: asset_path,
                source: err.into(),
            }),
        }
    }
    /// Writes the given asset to the filesystem asynchronously. This must only be used at build-time, and must not be changed
    /// afterward.
    pub async fn write(&self, name: &str, content: &str) -> Result<(), ImmutableStoreError> {
        let asset_path = format!("{}/{}", self.root_path, name);
        let mut dir_tree: Vec<&str> = asset_path.split('/').collect();
        dir_tree.pop();

        fs::create_dir_all(dir_tree.join("/")).map_err(|err| ImmutableStoreError::WriteFailed {
            name: asset_path.clone(),
            source: err.into(),
        })?;
        fs::write(&asset_path, content).map_err(|err| ImmutableStoreError::WriteFailed {
            name: asset_path,
            source: err.into(),
        })
    }
}
