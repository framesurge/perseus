use super::{Freeze, IdbFrozenStateStore};
use crate::templates::RenderCtx;
use wasm_bindgen::JsValue;

/// Freezes the app's state to IndexedDB to be accessed in future.
// TODO Error handling
pub async fn hsr_freeze(render_ctx: RenderCtx) {
    let frozen_state = render_ctx.freeze();
    // We use a custom name so we don't interfere with any state freezing the user's doing independently
    let idb_store = match IdbFrozenStateStore::new_with_name("perseus_hsr").await {
        Ok(idb_store) => idb_store,
        Err(_) => {
            return;
        }
    };
    match idb_store.set(&frozen_state).await {
        Ok(_) => log("State frozen."),
        Err(_) => {
            return;
        }
    };
}

/// Thaws a previous state frozen in development.
// This will be run at the beginning of every template function, which means it gets executed on the server as well, so we have to Wasm-gate this
#[cfg(target_arch = "wasm32")]
// TODO Error handling
pub async fn hsr_thaw(render_ctx: RenderCtx) {
    use super::{PageThawPrefs, ThawPrefs};

    let idb_store = match IdbFrozenStateStore::new_with_name("perseus_hsr").await {
        Ok(idb_store) => idb_store,
        Err(_) => {
            return;
        }
    };
    let frozen_state = match idb_store.get().await {
        Ok(Some(frozen_state)) => frozen_state,
        // If there's no frozen state available, we'll proceed as usual
        Ok(None) => return,
        Err(_) => {
            return;
        }
    };

    // This is designed to override everything to restore the app to its previous state, so we should override everything
    // This isn't problematic because the state will be frozen right before the reload and restored right after, so we literally can't miss anything (unless there's auto-typing tech involved!)
    let thaw_prefs = ThawPrefs {
        page: PageThawPrefs::IncludeAll,
        global_prefer_frozen: true,
    };
    // To be absolutely clear, this will NOT fail if the user has changed their data model, it will be triggered if the state is actually corrupted
    // If that's the case, we'll log it and wait for the next freeze to override the invalid stuff
    // If the user has updated their data model, the macros will fail with frozen state and switch to active or generated as necessary (meaning we lose the smallest amount of state
    // possible!)
    match render_ctx.thaw(&frozen_state, thaw_prefs) {
        Ok(_) => log("State restored."),
        Err(_) => log("Stored state corrupted, waiting for next code change to override."),
    };

    // We don't want this old state to persist if the user manually reloads (they'd be greeted with state that's probably out-of-date)
    match idb_store.clear().await {
        Ok(_) => (),
        Err(_) => {
            return;
        }
    }
}

/// Thaws a previous state frozen in development.
#[cfg(not(target_arch = "wasm32"))]
pub async fn hsr_thaw(_render_ctx: RenderCtx) {}

/// An internal function for logging data about HSR.
fn log(msg: &str) {
    web_sys::console::log_1(&JsValue::from("[HSR]: ".to_string() + msg));
}
