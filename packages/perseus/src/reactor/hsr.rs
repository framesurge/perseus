use crate::state::IdbFrozenStateStore;
use sycamore::web::Html;
use wasm_bindgen::JsValue;
use super::Reactor;

impl<G: Html> Reactor<G> {
    /// Freezes the app's state to IndexedDB to be accessed in future. This takes a
    /// pre-determined frozen state to avoid *really* annoying lifetime errors.
    pub(crate) async fn hsr_freeze(frozen_state: String) {
        // We use a custom name so we don't interfere with any state freezing the user's
        // doing independently
        let idb_store = match IdbFrozenStateStore::new_with_name("perseus_hsr").await {
            Ok(idb_store) => idb_store,
            Err(err) => return log(&format!("IndexedDB setup error: {}.", err)),
        };
        match idb_store.set(&frozen_state).await {
            Ok(_) => log("State frozen."),
            Err(err) => log(&format!("State freezing error: {}.", err)),
        };
    }

    /// Thaws a previous state frozen in development.
    pub(crate) async fn hsr_thaw(&self) {
        use crate::state::{PageThawPrefs, ThawPrefs};

        let idb_store = match IdbFrozenStateStore::new_with_name("perseus_hsr").await {
            Ok(idb_store) => idb_store,
            Err(err) => return log(&format!("IndexedDB setup error: {}.", err)),
        };
        let frozen_state = match idb_store.get().await {
            Ok(Some(frozen_state)) => frozen_state,
            // If there's no frozen state available, we'll proceed as usual
            Ok(None) => return,
            Err(err) => return log(&format!("Frozen state acquisition error: {}.", err)),
        };

        // This is designed to override everything to restore the app to its previous
        // state, so we should override everything This isn't problematic because
        // the state will be frozen right before the reload and restored right after, so
        // we literally can't miss anything (unless there's auto-typing tech involved!)
        let thaw_prefs = ThawPrefs {
            page: PageThawPrefs::IncludeAll,
            global_prefer_frozen: true,
        };
        // This invokes a modified type of thawing that is internal-only, which is more
        // lenient with invalid and/or corrupted state, thus allowing the user to radically
        // change their data model. In such cases, the frozen HSR state will simply be bypassed.
        match self._thaw(&frozen_state, thaw_prefs, true) {
            Ok(_) => log("State restored."),
            Err(_) => log("Stored state corrupted, waiting for next code change to override."),
        };

        // We don't want this old state to persist if the user manually reloads (they'd
        // be greeted with state that's probably out-of-date)
        match idb_store.clear().await {
            Ok(_) => (),
            Err(err) => log(&format!("Stale state clearing error: {}.", err)),
        }
    }
}

/// An internal function for logging data about HSR.
fn log(msg: &str) {
    web_sys::console::log_1(&JsValue::from("[HSR]: ".to_string() + msg));
}
