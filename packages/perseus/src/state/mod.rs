mod freeze;
mod global_state;
mod page_state_store;
mod rx_result;
mod rx_state;
#[cfg(target_arch = "wasm32")]
mod suspense;

pub use freeze::{FrozenApp, PageThawPrefs, ThawPrefs};
pub use global_state::{GlobalState, GlobalStateCreator, GlobalStateType};
pub use page_state_store::{PageStateStore, PssContains, PssEntry, PssState};
pub use rx_result::{RxResult, RxResultIntermediate, RxResultRef, SerdeInfallible};
pub use rx_state::{AnyFreeze, Freeze, MakeRx, MakeRxRef, MakeUnrx, RxRef, UnreactiveState};

#[cfg(all(feature = "idb-freezing", target_arch = "wasm32"))]
mod freeze_idb;
#[cfg(all(feature = "idb-freezing", target_arch = "wasm32"))]
pub use freeze_idb::{IdbError, IdbFrozenStateStore};

// We'll allow live reloading (of which HSR is a subset) if it's feature-enabled
// and we're in development mode
#[cfg(all(feature = "live-reload", debug_assertions, target_arch = "wasm32"))]
mod live_reload;
#[cfg(all(feature = "live-reload", debug_assertions, target_arch = "wasm32"))]
pub(crate) use live_reload::connect_to_reload_server;
#[cfg(all(feature = "live-reload", debug_assertions, target_arch = "wasm32"))]
pub(crate) use live_reload::force_reload;
#[cfg(target_arch = "wasm32")]
pub use suspense::{compute_nested_suspense, compute_suspense};

#[cfg(all(feature = "hsr", debug_assertions, target_arch = "wasm32"))]
mod hsr;
#[cfg(all(feature = "hsr", debug_assertions, target_arch = "wasm32"))]
pub(crate) use hsr::{hsr_freeze, hsr_thaw};
