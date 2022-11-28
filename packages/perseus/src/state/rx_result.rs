use crate::template::TemplateState;

use super::{Freeze, MakeRx, MakeRxRef, MakeUnrx, RxRef};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashMap, ops::Deref};
use sycamore::prelude::{create_rc_signal, create_ref, RcSignal, Scope};

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
#[derive(Serialize, Deserialize)]
pub struct RxResult<T, E>(Result<T, E>)
where
    T: MakeRx + 'static, /* Serialize + DeserializeOwned are handled automatically by the derive
                          * macro on both `T` and `E` */
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + MakeRxRef + Clone + 'static,
    E: Clone + 'static;
impl<T, E> MakeRx for RxResult<T, E>
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + MakeRxRef + Clone + 'static,
    E: Serialize + DeserializeOwned + Clone + 'static,
{
    type Rx = RxResultIntermediate<T, E>;

    fn make_rx(self) -> Self::Rx {
        match self.0 {
            Ok(state) => RxResultIntermediate(create_rc_signal(Ok(state.make_rx()))),
            Err(err) => RxResultIntermediate(create_rc_signal(Err(err))),
        }
    }

    // We'll just defer to the underlying type if it's `Ok`
    // TODO See notes
    fn split_delayed(self) -> (HashMap<String, TemplateState>, Self) {
        match self.0 {
            Ok(state) => {
                let split = state.split_delayed();
                (split.0, Self(Ok(split.1)))
            },
            Err(err) => (HashMap::new(), Self(Err(err)))
        }
    }
}

/// The intermediate reactive type for [`RxResult`]. You shouldn't need to
/// interface with this manually.
#[derive(Clone)]
pub struct RxResultIntermediate<T, E>(RcSignal<Result<T::Rx, E>>)
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + MakeRxRef + Clone + 'static,
    E: Serialize + DeserializeOwned + Clone + 'static;
impl<T, E> MakeUnrx for RxResultIntermediate<T, E>
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + MakeRxRef + Clone + 'static,
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
impl<T, E> Freeze for RxResultIntermediate<T, E>
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + MakeRxRef + Clone + 'static,
    E: Serialize + DeserializeOwned + Clone + 'static,
{
    fn freeze(&self) -> String {
        let self_clone = Self(self.0.clone());
        let unrx = self_clone.make_unrx();
        serde_json::to_string(&unrx).unwrap()
    }
}
impl<T, E> MakeRxRef for RxResultIntermediate<T, E>
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + MakeRxRef + Clone + 'static,
    E: Serialize + DeserializeOwned + Clone + 'static,
{
    type RxRef<'rx> = RxResultRef<'rx, T, E>; // where <<T as MakeRx>::Rx as MakeRxRef>::RxRef<'rx>: 'rx;

    fn to_ref_struct<'rx>(self, cx: Scope<'rx>) -> Self::RxRef<'rx> {
        RxResultRef(create_ref(cx, self.0))
    }
}

/// The final reference reactive type for [`RxResult`]. This is what you'll get
/// passed to suspense handlers that deal with a field wrapper in [`RxResult`].
///
/// Note that the underlying nested type will not be in its final reference
/// form, it will be in its intermediate form (otherwise dependency tracking is
/// impossible), although, due to the high-level scoped wrapping, ergonomics are
/// preserved.
#[derive(Clone)]
pub struct RxResultRef<'rx, T, E>(&'rx RcSignal<Result<T::Rx, E>>)
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + MakeRxRef + Clone + 'static,
    E: Serialize + DeserializeOwned + Clone + 'static;
impl<'rx, T, E> RxRef for RxResultRef<'rx, T, E>
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + MakeRxRef + Clone + 'static,
    E: Serialize + DeserializeOwned + Clone + 'static,
{
    type RxNonRef = T::Rx;
}

// We can implement all the `Signal` etc. methods by simply implementing the
// appropriate dereferencing
impl<T, E> Deref for RxResultIntermediate<T, E>
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + MakeRxRef + Clone + 'static,
    E: Serialize + DeserializeOwned + Clone + 'static,
{
    type Target = RcSignal<Result<T::Rx, E>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'rx, T, E> Deref for RxResultRef<'rx, T, E>
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + MakeRxRef + Clone + 'static,
    E: Serialize + DeserializeOwned + Clone + 'static,
{
    type Target = &'rx RcSignal<Result<T::Rx, E>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// We also want a usual `Result` to be able to be turned into `RxResult` for
// convenience
impl<T, E> From<Result<T, E>> for RxResult<T, E>
where
    T: MakeRx + Serialize + DeserializeOwned + 'static,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + MakeRxRef + Clone + 'static,
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
