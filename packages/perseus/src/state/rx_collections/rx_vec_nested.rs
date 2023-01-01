use crate::state::{Freeze, MakeRx, MakeUnrx};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::ops::Deref;
#[cfg(client)]
use sycamore::prelude::Scope;

/// A reactive version of [`Vec`] that uses nested reactivity on its elements.
/// That means the type inside the vector must implement [`MakeRx`] (usually
/// derived with the `ReactiveState` macro). If you want to store simple types
/// inside the vector, without nested reactivity (e.g. `String`s), you should
/// use [`RxVec`].
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RxVecNested<T>(Vec<T>)
where
    // We get the `Deserialize` derive macro working by tricking Serde by not
    // including the actual bounds here
    T: MakeRx + 'static,
    T::Rx: MakeUnrx<Unrx = T> + Freeze + Clone;
/// The reactive version of [`RxVecNested`].
#[derive(Clone, Debug)]
pub struct RxVecNestedRx<T>(Vec<T::Rx>)
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    T::Rx: MakeUnrx<Unrx = T> + Freeze + Clone;

// --- Reactivity implementations ---
impl<T> MakeRx for RxVecNested<T>
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    T::Rx: MakeUnrx<Unrx = T> + Freeze + Clone,
{
    type Rx = RxVecNestedRx<T>;

    fn make_rx(self) -> Self::Rx {
        RxVecNestedRx(self.0.into_iter().map(|x| x.make_rx()).collect())
    }
}
impl<T> MakeUnrx for RxVecNestedRx<T>
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    T::Rx: MakeUnrx<Unrx = T> + Freeze + Clone,
{
    type Unrx = RxVecNested<T>;

    fn make_unrx(self) -> Self::Unrx {
        RxVecNested(self.0.into_iter().map(|x| x.make_unrx()).collect())
    }

    #[cfg(client)]
    fn compute_suspense(&self, cx: Scope) {
        for elem in self.0.iter() {
            elem.compute_suspense(cx);
        }
    }
}
// --- Dereferencing ---
impl<T> Deref for RxVecNested<T>
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    T::Rx: MakeUnrx<Unrx = T> + Freeze + Clone,
{
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> Deref for RxVecNestedRx<T>
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    T::Rx: MakeUnrx<Unrx = T> + Freeze + Clone,
{
    type Target = Vec<T::Rx>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// --- Conversion implementation ---
impl<T> From<Vec<T>> for RxVecNested<T>
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    T::Rx: MakeUnrx<Unrx = T> + Freeze + Clone,
{
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

// --- Freezing implementation ---
impl<T> Freeze for RxVecNestedRx<T>
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    T::Rx: MakeUnrx<Unrx = T> + Freeze + Clone,
{
    fn freeze(&self) -> String {
        let unrx = Self(self.0.clone()).make_unrx();
        // This should never panic, because we're dealing with a vector
        serde_json::to_string(&unrx).unwrap()
    }
}
