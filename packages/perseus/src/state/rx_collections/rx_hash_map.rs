use crate::state::{Freeze, MakeRx, MakeUnrx};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
#[cfg(target_arch = "wasm32")]
use sycamore::prelude::Scope;
use sycamore::reactive::{create_rc_signal, RcSignal};

/// A reactive version of [`Vec`] that uses nested reactivity on its elements.
/// This requires nothing by `Clone + 'static` of the elements inside the map,
/// and it wraps them in `RcSignal`s to make them reactive. If you want to store
/// nested reactive types inside the map (e.g. `String`s), you should
/// use [`RxVecNested`].
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RxHashMap<K, V>(HashMap<K, V>)
where
    K: Clone + Eq + Hash,
    // We get the `Deserialize` derive macro working by tricking Serde by not
    // including the actual bounds here
    V: Clone + 'static;
/// The reactive version of [`RxHashMap`].
#[derive(Clone, Debug)]
pub struct RxHashMapRx<K, V>(HashMap<K, RcSignal<V>>)
where
    K: Clone + Serialize + DeserializeOwned + Eq + Hash,
    V: Clone + Serialize + DeserializeOwned + 'static;

// --- Reactivity implementations ---
impl<K, V> MakeRx for RxHashMap<K, V>
where
    K: Clone + Serialize + DeserializeOwned + Eq + Hash,
    V: Clone + Serialize + DeserializeOwned + 'static,
{
    type Rx = RxHashMapRx<K, V>;

    fn make_rx(self) -> Self::Rx {
        RxHashMapRx(
            self.0
                .into_iter()
                .map(|(k, v)| (k, create_rc_signal(v)))
                .collect(),
        )
    }
}
impl<K, V> MakeUnrx for RxHashMapRx<K, V>
where
    K: Clone + Serialize + DeserializeOwned + Eq + Hash,
    V: Clone + Serialize + DeserializeOwned + 'static,
{
    type Unrx = RxHashMap<K, V>;

    fn make_unrx(self) -> Self::Unrx {
        RxHashMap(
            self.0
                .into_iter()
                .map(|(k, v)| (k, (*v.get_untracked()).clone()))
                .collect(),
        )
    }

    #[cfg(target_arch = "wasm32")]
    fn compute_suspense(&self, cx: Scope) {}
}
// --- Dereferencing ---
impl<K, V> Deref for RxHashMap<K, V>
where
    K: Clone + Serialize + DeserializeOwned + Eq + Hash,
    V: Clone + Serialize + DeserializeOwned + 'static,
{
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<K, V> Deref for RxHashMapRx<K, V>
where
    K: Clone + Serialize + DeserializeOwned + Eq + Hash,
    V: Clone + Serialize + DeserializeOwned + 'static,
{
    type Target = HashMap<K, RcSignal<V>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// --- Conversion implementation ---
impl<K, V> From<HashMap<K, V>> for RxHashMap<K, V>
where
    K: Clone + Serialize + DeserializeOwned + Eq + Hash,
    V: Clone + Serialize + DeserializeOwned + 'static,
{
    fn from(value: HashMap<K, V>) -> Self {
        Self(value)
    }
}

// --- Freezing implementation ---
impl<K, V> Freeze for RxHashMapRx<K, V>
where
    K: Clone + Serialize + DeserializeOwned + Eq + Hash,
    V: Clone + Serialize + DeserializeOwned + 'static,
{
    fn freeze(&self) -> String {
        let unrx = Self(self.0.clone()).make_unrx();
        // This should never panic, because we're dealing with a vector
        serde_json::to_string(&unrx).unwrap()
    }
}
