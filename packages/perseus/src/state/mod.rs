mod freeze;
mod global_state;
mod page_state_store;
mod rx_state;

pub use freeze::{FrozenApp, PageThawPrefs, ThawPrefs};
pub use global_state::{GlobalState, GlobalStateCreator};
pub use page_state_store::PageStateStore;
pub use rx_state::{AnyFreeze, Freeze, MakeRx, MakeUnrx};

#[cfg(feature = "idb-freezing")]
mod freeze_idb;
#[cfg(feature = "idb-freezing")]
pub use freeze_idb::*; // TODO Be specific here

// We'll allow live reloading (of which HSR is a subset) if it's feature-enabled and we're in development mode
#[cfg(all(feature = "live-reload", debug_assertions))]
mod live_reload;
#[cfg(all(feature = "live-reload", debug_assertions))]
pub(crate) use live_reload::connect_to_reload_server;
#[cfg(all(feature = "live-reload", debug_assertions))]
pub use live_reload::force_reload;
