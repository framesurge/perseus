use rexie::{Direction, Error as RexieError, ObjectStore, Rexie, TransactionMode};
use std::rc::Rc;
use thiserror::Error;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum IdbError {
    #[error("couldn't build database")]
    BuildError {
        #[source]
        source: RexieError,
    },
    // The source of this would be a `JsValue`, which we drop for performance
    #[error("persistence check failed")]
    PersistenceCheckFailed {
        /// Whether or not this persistence check could be retried and might result in a success (this is just an estimation).
        retry: bool,
    },
    #[error("an error occurred while constructing an IndexedDB transaction")]
    TransactionError {
        #[source]
        source: RexieError,
    },
    #[error("an error occured while trying to set a new value")]
    SetError {
        #[source]
        source: RexieError,
    },
    #[error("an error occurred while clearing the store of previous values")]
    ClearError {
        #[source]
        source: RexieError,
    },
    #[error("an error occurred while trying to get the latest value")]
    GetError {
        #[source]
        source: RexieError,
    },
}

/// A frozen state store that uses IndexedDB as a backend. This will only store a single frozen state at a time, removing all previously stored states every time a new one is set.
///
/// TODO Browser compatibility information.
#[derive(Clone)]
pub struct IdbFrozenStateStore {
    /// A handle to the database.
    db: Rc<Rexie>,
}
impl std::fmt::Debug for IdbFrozenStateStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IdbFrozenStateStore").finish()
    }
}
impl IdbFrozenStateStore {
    /// Creates a new store for this origin. If it already exists from a previous visit, the existing one will be interfaced with.
    pub async fn new() -> Result<Self, IdbError> {
        Self::new_with_name("perseus").await
    }
    /// Creates a new store for this origin. If it already exists from a previous visit, the existing one will be interfaced with. This also allows the provision of a custom name for the DB.
    pub(crate) async fn new_with_name(name: &str) -> Result<Self, IdbError> {
        // Build the database
        let rexie = Rexie::builder(name)
            // IndexedDB uses versions to track database schema changes
            // If the structure of this DB ever changes, this MUST be changed, and this should be considered a non-API-breaking, but app-breaking change!
            .version(1)
            .add_object_store(
                // We'll store many versions of frozen state so that the user can revert to previous states
                ObjectStore::new("frozen_state")
                    // We don't provide a key path because that would lock us in to storing only JS objects, and we want to store strings
                    .auto_increment(true), // IndexedDB doesn't need us to register value types, only things that should be indexed (gotta love JS type safety haven't you!)
            )
            .build()
            .await
            .map_err(|err| IdbError::BuildError { source: err })?;

        Ok(Self { db: Rc::new(rexie) })
    }
    /// Gets the stored frozen state. Be warned that the result of this could be arbitrarily old, or it may have been tampered with by the user (in which case Perseus will either return an
    /// error that you can handle or it'll fall back to the active state). If no state has been stored yet, this will return `Ok(None)`.
    pub async fn get(&self) -> Result<Option<String>, IdbError> {
        let transaction = self
            .db
            .transaction(&["frozen_state"], TransactionMode::ReadOnly)
            .map_err(|err| IdbError::TransactionError { source: err })?;
        let store = transaction
            .store("frozen_state")
            .map_err(|err| IdbError::TransactionError { source: err })?;

        // Get the last element from the store by working backwards and getting everything, with a limit of 1
        let frozen_states = store
            .get_all(None, Some(1), None, Some(Direction::Prev))
            .await
            .map_err(|err| IdbError::GetError { source: err })?;
        let frozen_state = match frozen_states.get(0) {
            Some((_key, value)) => value,
            None => return Ok(None),
        };
        // TODO Do this without cloning the whole thing into the Wasm table and then moving it into Rust
        let frozen_state = frozen_state.as_string().unwrap();
        transaction
            .commit()
            .await
            .map_err(|err| IdbError::TransactionError { source: err })?;

        Ok(Some(frozen_state))
    }
    /// Sets the content to a new frozen state.
    pub async fn set(&self, frozen_state: &str) -> Result<(), IdbError> {
        let transaction = self
            .db
            .transaction(&["frozen_state"], TransactionMode::ReadWrite)
            .map_err(|err| IdbError::TransactionError { source: err })?;
        let store = transaction
            .store("frozen_state")
            .map_err(|err| IdbError::TransactionError { source: err })?;

        // We only store a single frozen state, and they can be quite large, so we'll remove any that are already in here
        store
            .clear()
            .await
            .map_err(|err| IdbError::ClearError { source: err })?;
        // We can add the frozen state directly because it's already a serialized string
        // This returns the ID, but we don't need to care about that
        store
            .add(&JsValue::from(frozen_state), None)
            .await
            .map_err(|err| IdbError::SetError { source: err })?;
        transaction
            .commit()
            .await
            .map_err(|err| IdbError::SetError { source: err })?;

        Ok(())
    }
    /// Clears the stored frozen state entirely and irrecoverably.
    pub async fn clear(&self) -> Result<(), IdbError> {
        let transaction = self
            .db
            .transaction(&["frozen_state"], TransactionMode::ReadWrite)
            .map_err(|err| IdbError::TransactionError { source: err })?;
        let store = transaction
            .store("frozen_state")
            .map_err(|err| IdbError::TransactionError { source: err })?;

        store
            .clear()
            .await
            .map_err(|err| IdbError::ClearError { source: err })?;

        Ok(())
    }
    /// Checks if the storage is persistently stored. If it is, the browser isn't allowed to clear it, the user would have to manually. This doesn't provide a guarantee that all users who've
    /// been to your site before will have previous state stored, you should assume that they could well have cleared it manually (or with very stringent privacy settings).
    ///
    /// If this returns an error, a recommendation about whether or not to retry will be attached. You generally shouldn't retry this more than once if there was an error.
    ///
    /// For more information about persistent storage on the web, see [here](https://web.dev/persistent-storage).
    pub async fn is_persistent() -> Result<bool, IdbError> {
        let storage_manager = web_sys::window().unwrap().navigator().storage();
        // If we can't access this, we're probably in a very old browser, so retrying isn't worth it in all likelihood
        let persisted = storage_manager
            .persisted()
            .map_err(|_| IdbError::PersistenceCheckFailed { retry: false })?;
        let persisted = JsFuture::from(persisted)
            .await
            .map_err(|_| IdbError::PersistenceCheckFailed { retry: true })?;
        let persisted_bool = persisted
            .as_bool()
            .ok_or(IdbError::PersistenceCheckFailed { retry: true })?;

        Ok(persisted_bool)
    }
    /// Requests persistent storage from the browser. In Firefox, the user will be prompted, though in Chrome the browser will automatically accept or deny based on your site's level of
    /// engagement, whether ot not it's been installed or bookmarked, and whether or not it's been granted the permission to show notifications. In other words, do NOT assume that this will
    /// be accepted, even if you ask the user very nicely. That said, especially in Firefox, you should display a custom notification before this with `alert()` or similar that explains why
    /// your site needs persistent storage for frozen state.
    ///
    /// If this returns `false`, the request was rejected, but you can retry in future (for user experience though, it's recommended to only do so very sparingly). If this returns an error,
    /// a recommendation about whether or not to retry will be attached. You generally shouldn't retry this more than once if there was an error.
    ///
    /// For more information about persistent storage on the web, see [here](https://web.dev/persistent-storage).
    pub async fn request_persistence() -> Result<bool, IdbError> {
        let storage_manager = web_sys::window().unwrap().navigator().storage();
        // If we can't access this, we're probably in a very old browser, so retrying isn't worth it in all likelihood
        let res = storage_manager
            .persist()
            .map_err(|_| IdbError::PersistenceCheckFailed { retry: false })?;
        let res = JsFuture::from(res)
            .await
            .map_err(|_| IdbError::PersistenceCheckFailed { retry: true })?;
        let res_bool = res.as_bool().unwrap();

        Ok(res_bool)
    }
}
