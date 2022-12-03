use crate::errors::*;
#[cfg(not(target_arch = "wasm32"))]
use tokio::{
    fs::{create_dir_all, File},
    io::{AsyncReadExt, AsyncWriteExt},
};

/// A trait for implementations of stores that the Perseus engine can use for
/// mutable data, which may need to be altered while the server is running. In
/// exported apps, this is irrelevant, since there is no server process to speak
/// of. This store is used in particular by the revalidation and incremental
/// generation strategies, which will both update build artifacts for future
/// caching. By default, [`FsMutableStore`] is used for simplicity and low
/// latency, though this is only viable in deployments with writable
/// filesystems. Notably, this precludes usage in serverless functions.
///
/// However, this is written deliberately as a trait with exposed, isolated
/// error types (see [`StoreError`]), so that users can write their own
/// implementations of this. For instance, you could manage mutable artifacts in
/// a database, though this should be as low-latency as possible, since reads
/// and writes are required at extremely short-notice as new user requests
/// arrive.
///
/// **Warning:** the `NotFound` error is integral to Perseus' internal operation,
/// and must be returned if an asset does not exist. Do NOT return any other error
/// if everything else worked, but an asset simply did not exist, or the entire
/// render system will come crashing down around you!
#[async_trait::async_trait]
pub trait MutableStore: std::fmt::Debug + Clone + Send + Sync {
    /// Reads data from the named asset.
    async fn read(&self, name: &str) -> Result<String, StoreError>;
    /// Writes data to the named asset. This will create a new asset if one
    /// doesn't exist already.
    async fn write(&self, name: &str, content: &str) -> Result<(), StoreError>;
}

/// The default [`MutableStore`], which simply uses the filesystem. This is
/// suitable for development and production environments with
/// writable filesystems (in which it's advised), but this is of course not
/// usable on production read-only filesystems, and another implementation of
/// [`MutableStore`] should be preferred.
///
/// Note: the `.write()` methods on this implementation will create any missing
/// parent directories automatically.
#[derive(Clone, Debug)]
pub struct FsMutableStore {
    #[cfg(not(target_arch = "wasm32"))]
    root_path: String,
}
#[cfg(not(target_arch = "wasm32"))]
impl FsMutableStore {
    /// Creates a new filesystem configuration manager. You should provide a
    /// path like `dist/mutable` here. Make sure that this is not the same
    /// path as the [`ImmutableStore`](super::ImmutableStore), as this will
    /// cause potentially problematic overlap between the two systems.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(root_path: String) -> Self {
        Self { root_path }
    }
}
#[async_trait::async_trait]
impl MutableStore for FsMutableStore {
    #[cfg(not(target_arch = "wasm32"))]
    async fn read(&self, name: &str) -> Result<String, StoreError> {
        let asset_path = format!("{}/{}", self.root_path, name);
        let file_res = File::open(&asset_path).await;
        let mut file = match file_res {
            Ok(file) => file,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                return Err(StoreError::NotFound { name: asset_path })
            }
            Err(err) => {
                return Err(StoreError::ReadFailed {
                    name: asset_path,
                    source: err.into(),
                })
            }
        };
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
    #[cfg(not(target_arch = "wasm32"))]
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
    #[cfg(target_arch = "wasm32")]
    async fn read(&self, _name: &str) -> Result<String, StoreError> {
        Ok(String::new())
    }
    #[cfg(target_arch = "wasm32")]
    async fn write(&self, _name: &str, _content: &str) -> Result<(), StoreError> {
        Ok(())
    }
}
