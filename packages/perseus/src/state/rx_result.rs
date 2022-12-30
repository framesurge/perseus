use super::{Freeze, MakeRx, MakeUnrx};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::ops::Deref;
#[cfg(target_arch = "wasm32")]
use sycamore::prelude::Scope;
use sycamore::prelude::{create_rc_signal, RcSignal};

/// A wrapper for fallible reactive state.
///
/// This is intended for use with the suspense state feature and nested state.
/// Imagine you have a `struct Song` that you derive `ReactiveState` on, having
/// the `album: Album` field use nested reactivity, since `Album` has multiple
/// fields itself. However, for some reason, you'd like to make the `album`
/// field use `#[rx(suspense = "your_handler_fn")]`. If `your_handler_fn()`
/// could lead to an error, then you have to be able to use the pattern
/// `song.album.get()?.name.get()`, rather than just `song.album.name.get()`.
/// The extra `.get()?` is needed since the `Album` is what is suspended state,
/// and it could be an error of some sort. What you need is a reactive `Result`,
/// and this is that. Any type can be placed in this that implements `MakeRx`,
/// `Serialize`, and `Deserialize`. No restrictions are placed on the error
/// type.
///
/// Note that this is intended for use with fallible, nested, suspended state,
/// although it could be easily used in any case where you want to use reactive
/// state that could be an error, conveniently enabling the pattern explained
/// above.
///
/// If you want non-nested, fallible, suspended state, you can simply use
/// `Result<T, E>` from the standard library.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RxResult<T, E>(Result<T, E>)
where
    T: MakeRx + 'static, /* Serialize + DeserializeOwned are handled automatically by the derive
                          * macro on both `T` and `E` */
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + Clone + 'static,
    E: Clone + 'static;
impl<T, E> MakeRx for RxResult<T, E>
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + Clone + 'static,
    E: Serialize + DeserializeOwned + Clone + 'static,
{
    type Rx = RxResultRx<T, E>;

    fn make_rx(self) -> Self::Rx {
        match self.0 {
            Ok(state) => RxResultRx(create_rc_signal(Ok(state.make_rx()))),
            Err(err) => RxResultRx(create_rc_signal(Err(err))),
        }
    }
}

/// The intermediate reactive type for [`RxResult`]. You shouldn't need to
/// interface with this manually.
#[derive(Clone, Debug)]
pub struct RxResultRx<T, E>(RcSignal<Result<T::Rx, E>>)
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + Clone + 'static,
    E: Serialize + DeserializeOwned + Clone + 'static;
impl<T, E> MakeUnrx for RxResultRx<T, E>
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + Clone + 'static,
    E: Serialize + DeserializeOwned + Clone + 'static,
{
    type Unrx = RxResult<T, E>;

    fn make_unrx(self) -> Self::Unrx {
        match &*self.0.get_untracked() {
            Ok(state) => RxResult(Ok(state.clone().make_unrx())),
            Err(err) => RxResult(Err(err.clone())),
        }
    }
    // Having a nested field that is not suspended, that has nested suspended
    // fields, is fine. When that top-level field is *also* suspended, that is
    // very much not okay! (We would have multiple handlers operating on the
    // same fields, which is not a pattern I want to encourage.)
    #[cfg(target_arch = "wasm32")]
    fn compute_suspense<'a>(&self, _cx: Scope<'a>) {}
}
impl<T, E> Freeze for RxResultRx<T, E>
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + Clone + 'static,
    E: Serialize + DeserializeOwned + Clone + 'static,
{
    fn freeze(&self) -> String {
        let self_clone = Self(self.0.clone());
        let unrx = self_clone.make_unrx();
        serde_json::to_string(&unrx).unwrap()
    }
}

// We can implement all the `Signal` etc. methods by simply implementing the
// appropriate dereferencing
impl<T, E> Deref for RxResultRx<T, E>
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + Clone + 'static,
    E: Serialize + DeserializeOwned + Clone + 'static,
{
    type Target = RcSignal<Result<T::Rx, E>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// We also want a usual `Result` to be able to be turned into `RxResult` for
// convenience
impl<T, E> From<Result<T, E>> for RxResult<T, E>
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + Clone + 'static,
    E: Serialize + DeserializeOwned + Clone + 'static,
{
    fn from(val: Result<T, E>) -> Self {
        Self(val)
    }
}

/// An analogue of [`std::convert::Infallible`] that can be serialized and
/// deserialized, since Serde currently does not implement those traits on the
/// standard library's `Infallible`. Until [this issue](https://github.com/serde-rs/serde/issues/2073) is
/// resolved, this must be used instead in the state platform.
///
/// The intended usage of this is in `Result`s or `RxResult`s for suspended
/// state whose handlers cannot fail.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerdeInfallible;
