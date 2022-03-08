use crate::errors::*;
use tokio::{
    fs::{create_dir_all, File},
    io::{AsyncReadExt, AsyncWriteExt},
};

/// A trait for mutable stores. This is abstracted away so that users can implement a non-filesystem mutable store, which is useful
/// for read-only filesystem environments, as on many modern hosting providers. See the book for further details on this subject.
#[async_trait::async_trait]
pub trait MutableStore: std::fmt::Debug + Clone + Send + Sync {
    /// Reads data from the named asset.
    async fn read(&self, name: &str) -> Result<String, StoreError>;
    /// Writes data to the named asset. This will create a new asset if one doesn't exist already.
    async fn write(&self, name: &str, content: &str) -> Result<(), StoreError>;
}

/// The default mutable store, which simply uses the filesystem. This is suitable for development and production environments with
/// writable filesystems (in which it's advised), but this is of course not usable on production read-only filesystems, and another
/// implementation of `MutableStore` should be preferred.
///
/// Note: the `.write()` methods on this implementation will create any missing parent directories automatically.
#[derive(Clone, Debug)]
pub struct FsMutableStore {
    root_path: String,
}
impl FsMutableStore {
    /// Creates a new filesystem configuration manager. You should provide a path like `dist/mutable` here. Make sure that this is
    /// not the same path as the immutable store, as this will cause potentially problematic overlap between the two systems.
    pub fn new(root_path: String) -> Self {
        Self { root_path }
    }
}
#[async_trait::async_trait]
impl MutableStore for FsMutableStore {
    async fn read(&self, name: &str) -> Result<String, StoreError> {
        let asset_path = format!("{}/{}", self.root_path, name);
        let mut file = File::open(&asset_path)
            .await
            .map_err(|err| StoreError::ReadFailed {
                name: asset_path.clone(),
                source: err.into(),
            })?;
        let metadata = file.metadata().await;

        match metadata {
            Ok(_) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .await
                    .map_err(|err| StoreError::ReadFailed {
                        name: asset_path,
                        source: err.into(),
                    })?;
                Ok(contents)
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                Err(StoreError::NotFound { name: asset_path })
            }
            Err(err) => Err(StoreError::ReadFailed {
                name: asset_path,
                source: err.into(),
            }),
        }
    }
    // This creates a directory structure as necessary
    async fn write(&self, name: &str, content: &str) -> Result<(), StoreError> {
        let asset_path = format!("{}/{}", self.root_path, name);
        let mut dir_tree: Vec<&str> = asset_path.split('/').collect();
        dir_tree.pop();

        create_dir_all(dir_tree.join("/"))
            .await
            .map_err(|err| StoreError::WriteFailed {
                name: asset_path.clone(),
                source: err.into(),
            })?;

        // This will either create the file or truncate it if it already exists
        let mut file = File::create(&asset_path)
            .await
            .map_err(|err| StoreError::WriteFailed {
                name: asset_path.clone(),
                source: err.into(),
            })?;
        file.write_all(content.as_bytes())
            .await
            .map_err(|err| StoreError::WriteFailed {
                name: asset_path.clone(),
                source: err.into(),
            })?;
        // TODO Can we use `sync_data()` here to reduce I/O?
        file.sync_all()
            .await
            .map_err(|err| StoreError::WriteFailed {
                name: asset_path,
                source: err.into(),
            })?;

        Ok(())
    }
}
