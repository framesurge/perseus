use crate::state::{Freeze, MakeRx, MakeUnrx};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::ops::Deref;
use sycamore::reactive::{create_signal, Signal};

/// A reactive version of [`Vec`] that uses nested reactivity on its elements.
/// This requires nothing by `Clone + 'static` of the elements inside the
/// vector, and it wraps them in `RcSignal`s to make them reactive. If you want
/// to store nested reactive types inside the vector (e.g. `String`s), you
/// should use [`super::RxVecNested`].
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RxVec<T>(Vec<T>)
where
    // We get the `Deserialize` derive macro working by tricking Serde by not
    // including the actual bounds here
    T: Clone + 'static;
/// The reactive version of [`RxVec`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RxVecRx<T>(Signal<Vec<Signal<T>>>)
where
    T: Clone + Serialize + DeserializeOwned + 'static;

// --- Reactivity implementations ---
impl<T> MakeRx for RxVec<T>
where
    T: Clone + Serialize + DeserializeOwned + 'static,
{
    type Rx = RxVecRx<T>;

    fn make_rx(self) -> Self::Rx {
        RxVecRx(create_signal(
            self.0.into_iter().map(|x| create_signal(x)).collect(),
        ))
    }
}
impl<T> MakeUnrx for RxVecRx<T>
where
    T: Clone + Serialize + DeserializeOwned + 'static,
{
    type Unrx = RxVec<T>;

    fn make_unrx(self) -> Self::Unrx {
        self.0.with_untracked(|vec| {
            RxVec(
                vec.iter()
                    .map(|x| x.with_untracked(|v| v.clone()))
                    .collect(),
            )
        })
    }

    #[cfg(any(client, doc))]
    fn compute_suspense(&self) {}
}
// --- Dereferencing ---
impl<T> Deref for RxVec<T>
where
    T: Clone + Serialize + DeserializeOwned + 'static,
{
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> Deref for RxVecRx<T>
where
    T: Clone + Serialize + DeserializeOwned + 'static,
{
    type Target = Signal<Vec<Signal<T>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// --- Conversion implementation ---
impl<T> From<Vec<T>> for RxVec<T>
where
    T: Clone + Serialize + DeserializeOwned + 'static,
{
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

// --- Freezing implementation ---
impl<T> Freeze for RxVecRx<T>
where
    T: Clone + Serialize + DeserializeOwned + 'static,
{
    fn freeze(&self) -> String {
        let unrx = Self(self.0.clone()).make_unrx();
        // This should never panic, because we're dealing with a vector
        serde_json::to_string(&unrx).unwrap()
    }
}
