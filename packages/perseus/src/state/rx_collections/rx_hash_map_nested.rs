use crate::state::{Freeze, MakeRx, MakeUnrx};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
#[cfg(client)]
use sycamore::prelude::Scope;

/// A reactive version of [`Vec`] that uses nested reactivity on its elements.
/// That means the type inside the vector must implement [`MakeRx`] (usually
/// derived with the `ReactiveState` macro). If you want to store simple types
/// inside the vector, without nested reactivity (e.g. `String`s), you should
/// use [`RxVec`].
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RxHashMapNested<K, V>(HashMap<K, V>)
where
    K: Clone + Eq + Hash,
    // We get the `Deserialize` derive macro working by tricking Serde by not
    // including the actual bounds here
    V: MakeRx + 'static,
    V::Rx: MakeUnrx<Unrx = V> + Freeze + Clone;
/// The reactive version of [`RxHashMap`].
#[derive(Clone, Debug)]
pub struct RxHashMapNestedRx<K, V>(HashMap<K, V::Rx>)
where
    K: Clone + Serialize + DeserializeOwned + Eq + Hash,
    V: MakeRx + Serialize + DeserializeOwned + 'static,
    V::Rx: MakeUnrx<Unrx = V> + Freeze + Clone;

// --- Reactivity implementations ---
impl<K, V> MakeRx for RxHashMapNested<K, V>
where
    K: Clone + Serialize + DeserializeOwned + Eq + Hash,
    V: MakeRx + Serialize + DeserializeOwned + 'static,
    V::Rx: MakeUnrx<Unrx = V> + Freeze + Clone,
{
    type Rx = RxHashMapNestedRx<K, V>;

    fn make_rx(self) -> Self::Rx {
        RxHashMapNestedRx(self.0.into_iter().map(|(k, v)| (k, v.make_rx())).collect())
    }
}
impl<K, V> MakeUnrx for RxHashMapNestedRx<K, V>
where
    K: Clone + Serialize + DeserializeOwned + Eq + Hash,
    V: MakeRx + Serialize + DeserializeOwned + 'static,
    V::Rx: MakeUnrx<Unrx = V> + Freeze + Clone,
{
    type Unrx = RxHashMapNested<K, V>;

    fn make_unrx(self) -> Self::Unrx {
        RxHashMapNested(
            self.0
                .into_iter()
                .map(|(k, v)| (k, v.make_unrx()))
                .collect(),
        )
    }

    #[cfg(client)]
    fn compute_suspense(&self, cx: Scope) {
        for elem in self.0.values() {
            elem.compute_suspense(cx);
        }
    }
}
// --- Dereferencing ---
impl<K, V> Deref for RxHashMapNested<K, V>
where
    K: Clone + Serialize + DeserializeOwned + Eq + Hash,
    V: MakeRx + Serialize + DeserializeOwned + 'static,
    V::Rx: MakeUnrx<Unrx = V> + Freeze + Clone,
{
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<K, V> Deref for RxHashMapNestedRx<K, V>
where
    K: Clone + Serialize + DeserializeOwned + Eq + Hash,
    V: MakeRx + Serialize + DeserializeOwned + 'static,
    V::Rx: MakeUnrx<Unrx = V> + Freeze + Clone,
{
    type Target = HashMap<K, V::Rx>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// --- Conversion implementation ---
impl<K, V> From<HashMap<K, V>> for RxHashMapNested<K, V>
where
    K: Clone + Serialize + DeserializeOwned + Eq + Hash,
    V: MakeRx + Serialize + DeserializeOwned + 'static,
    V::Rx: MakeUnrx<Unrx = V> + Freeze + Clone,
{
    fn from(value: HashMap<K, V>) -> Self {
        Self(value)
    }
}

// --- Freezing implementation ---
impl<K, V> Freeze for RxHashMapNestedRx<K, V>
where
    K: Clone + Serialize + DeserializeOwned + Eq + Hash,
    V: MakeRx + Serialize + DeserializeOwned + 'static,
    V::Rx: MakeUnrx<Unrx = V> + Freeze + Clone,
{
    fn freeze(&self) -> String {
        let unrx = Self(self.0.clone()).make_unrx();
        // This should never panic, because we're dealing with a vector
        serde_json::to_string(&unrx).unwrap()
    }
}
