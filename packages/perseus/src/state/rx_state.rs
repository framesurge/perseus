use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::any::Any;
use sycamore::prelude::Scope;

/// A trait for `struct`s that can be made reactive. Typically, this will be
/// derived with the `#[make_rx]` macro, though it can be implemented manually
/// if you have more niche requirements.
pub trait MakeRx {
    /// The type of the reactive version that we'll convert to. By having this
    /// as an associated type, we can associate the reactive type with the
    /// unreactive, meaning greater inference and fewer arguments that the
    /// user needs to provide to macros.
    type Rx: MakeUnrx;
    /// Transforms an instance of the `struct` into its reactive version.
    fn make_rx(self) -> Self::Rx;
}

/// A trait for reactive `struct`s that can be made un-reactive. This is the
/// opposite of `MakeRx`, and is intended particularly for state freezing. Like
/// `MakeRx`, this will usually be derived automatically with the `#[make_rx]`
/// macro, but you can also implement it manually.
pub trait MakeUnrx {
    /// The type of the unreactive version that we'll convert to.
    type Unrx: Serialize + for<'de> Deserialize<'de> + MakeRx;
    /// Transforms an instance of the `struct` into its unreactive version. By
    /// having this as an associated type, we can associate the reactive type
    /// with the unreactive, meaning greater inference and fewer arguments
    /// that the user needs to provide to macros.
    fn make_unrx(self) -> Self::Unrx;
    /// Calls all handlers on suspended state, spawning scoped futures for each of them
    /// (the futures *must* be scoped to prevent the same handler being run multiple
    /// times concurrently if a user leaves the page and then comes back).
    ///
    /// This has no return type, since it simply spawns futures for each of the user's
    /// handlers. Each handler must have the following function signature:
    ///
    /// ```
    /// Fn(Scope<'a>, RxRef<'a>);
    /// ```
    ///
    /// Here, `RxRef` denotes the reference `struct` their template would be provided with.
    /// In the case of an individual, non-nested field, this will be `&'a Signal<T>`, where
    /// `T` is the type of the field.
    ///
    /// Fallible handlers should operate on fields with type `Result<T, E>` so they
    /// can propagate errors directly back to the user's template code.
    ///
    /// If you're implementing `MakeUnrx` manually, you can usually leave the
    /// body of this function empty unless you're using the suspended state
    /// system.
    #[cfg(target_arch = "wasm32")]
    fn compute_suspended_state<'a>(&self, cx: Scope<'a>);
}

/// A trait for reactive `struct`s that can be made to use `&'a Signal`s
/// rather than `RcSignal`s, when provided with a Sycamore reactive scope.
/// This is necessary for reaping the benefits of the ergonomics of Sycamore's
/// v2 reactive primitives.
pub trait MakeRxRef {
    /// The type of the reactive `struct` using `&'a Signal`s (into which
    /// the type implementing this trait can be converted).
    type RxRef<'a>;
    /// Convert this into a version using `&'a Signal`s using `create_ref()`.
    fn to_ref_struct<'a>(self, cx: Scope<'a>) -> Self::RxRef<'a>;
}

/// A trait for `struct`s that are both reactive *and* using `&'a Signal`s
/// to store their underlying data. This exists solely to link such types to
/// their intermediate, `RcSignal`, equivalents.
pub trait RxRef {
    /// The linked intermediate type using `RcSignal`s. Note that this is
    /// itself reactive, just not very ergonomic.
    type RxNonRef: MakeUnrx;
}

/// A trait for reactive `struct`s that can be made unreactive and serialized to
/// a `String`. `struct`s that implement this should implement `MakeUnrx` for
/// simplicity, but they technically don't have to (they always do in Perseus
/// macro-generated code).
pub trait Freeze {
    /// 'Freezes' the reactive `struct` by making it unreactive and converting
    /// it to a `String`.
    fn freeze(&self) -> String;
}

/// A convenience super-trait for `Freeze`able things that can be downcast to
/// concrete types.
pub trait AnyFreeze: Freeze + Any {
    /// Gives `&dyn Any` to enable downcasting.
    fn as_any(&self) -> &dyn Any;
}
impl<T: Any + Freeze> AnyFreeze for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// A marker trait for types that you want to be able to use with the Perseus
/// state platform, without using `#[make_rx]`. If you want to use unreactive
/// state, implement this, and you'll automatically be able to use your
/// unreactive type without problems!
pub trait UnreactiveState {}

/// A wrapper for storing unreactive state in Perseus, and allowing it to
/// interface with the (fundamentally reactive) state platform. Generally, you
/// would just use reactive state, however, sometimes, you may wish to use
/// unreactive state without sacrificing features like automatic page state
/// caching: this `struct` allows that.
///
/// This is handled automatically by the `#[template]` macro, and you should
/// never need to use this manually unless you don't use the macros.
///
/// This wrapper will automatically implement all the necessary `trait`s to
/// interface with Perseus' reactive state platform, along with `Serialize` and
/// `Deserialize` (provided the underlying type also implements the latter two).
#[derive(Clone)]
pub struct UnreactiveStateWrapper<
    T: Serialize + for<'de> Deserialize<'de> + UnreactiveState + Clone,
>(pub T);
// Automatically implement `MakeRx` for any marked unreactive type, using
// `UnreactiveStateWrapper` as the reactive type
impl<T: Serialize + for<'de> Deserialize<'de> + UnreactiveState + Clone> MakeRx for T {
    type Rx = UnreactiveStateWrapper<T>;
    fn make_rx(self) -> Self::Rx {
        UnreactiveStateWrapper(self)
    }
}
// And let it be converted back
impl<T: Serialize + for<'de> Deserialize<'de> + UnreactiveState + Clone> MakeUnrx
    for UnreactiveStateWrapper<T>
{
    type Unrx = T;
    fn make_unrx(self) -> Self::Unrx {
        self.0
    }
    // Suspense is not allowed on unreactive state
    #[cfg(target_arch = "wasm32")]
    fn compute_suspended_state(&self, _cx: Scope) {}
}
// And, since the underlying type can be serialized, implement `Freeze`
impl<T: Serialize + for<'de> Deserialize<'de> + UnreactiveState + Clone> Freeze
    for UnreactiveStateWrapper<T>
{
    fn freeze(&self) -> String {
        // Just serialize the underlying type
        serde_json::to_string(&self.0).unwrap()
    }
}

/// A wrapper for fallible reactive state.
///
/// This is intended for use with the suspense state feature and nested state. Imagine you
/// have a `struct Song` that you derive `ReactiveState` on, having the `album: Album` field
/// use nested reactivity, since `Album` has multiple fields itself. However, for some reason,
/// you'd like to make the `album` field use `#[rx(suspense = "your_handler_fn")]`. If `your_handler_fn()`
/// could lead to an error, then you have to be able to use the pattern `song.album.get()?.name.get()`,
/// rather than just `song.album.name.get()`. The extra `.get()?` is needed since the `Album` is
/// what is suspended state, and it could be an error of some sort. What you need is a reactive `Result`,
/// and this is that. Any type can be placed in this that implements `MakeRx`, `Serialize`, and `Deserialize`.
/// No restrictions are placed on the error type.
///
/// Note that this is intended for use with fallible, nested, suspended state, although it could be
/// easily used in any case where you want to use reactive state that could be an error, conveniently
/// enabling the pattern explained above.
///
/// If you want non-nested, fallible, suspended state, you can simply use `Result<T, E>` from the standard
/// library.
#[derive(Serialize, Deserialize)]
pub enum RxResult<T, E>
where
    T: MakeRx, // Serialize + DeserializeOwned are handled automatically by the derive macro on both `T` and `E`
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + MakeRxRef + Clone,
    E: Clone,
{
    Ok(T),
    Err(E),
}
impl<T, E> MakeRx for RxResult<T, E>
where
    T: MakeRx + Serialize + DeserializeOwned,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + MakeRxRef + Clone,
    E: Serialize + DeserializeOwned + Clone,
    // <<T as MakeRx>::Rx as MakeUnrx>::Unrx: MakeRx + Serialize + DeserializeOwned,
{
    type Rx = RxResultIntermediate<T, E>;

    fn make_rx(self) -> Self::Rx {
        match self {
            Self::Ok(state) => Self::Rx::Ok(state.make_rx()),
            Self::Err(err) => Self::Rx::Err(err),
        }
    }
}

/// The intermediate reactive type for [`RxResult`]. You shouldn't need to interface with this manually.
#[derive(Clone)]
pub enum RxResultIntermediate<T, E>
where
    T: MakeRx + Serialize + DeserializeOwned,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + MakeRxRef + Clone,
    E: Serialize + DeserializeOwned + Clone,
{
    Ok(T::Rx),
    Err(E),
}
impl<T, E> MakeUnrx for RxResultIntermediate<T, E>
where
    T: MakeRx + Serialize + DeserializeOwned,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + MakeRxRef + Clone,
    E: Serialize + DeserializeOwned + Clone,
{
    type Unrx = RxResult<T, E>;

    fn make_unrx(self) -> Self::Unrx {
        match self {
            Self::Ok(state) => Self::Unrx::Ok(state.make_unrx()),
            Self::Err(err) => Self::Unrx::Err(err),
        }
    }
}
impl<T, E> Freeze for RxResultIntermediate<T, E>
where
    T: MakeRx + Serialize + DeserializeOwned,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + MakeRxRef + Clone,
    E: Serialize + DeserializeOwned + Clone,
{
    fn freeze(&self) -> String {
        // I have no clue why this is necessary...
        let self_clone = match &self {
            Self::Ok(rx_state) => Self::Ok(rx_state.clone()),
            Self::Err(err) => Self::Err(err.clone())
        };
        let unrx = self_clone.make_unrx();
        serde_json::to_string(&unrx).unwrap()
    }
}
impl<T, E> MakeRxRef for RxResultIntermediate<T, E>
where
    T: MakeRx + Serialize + DeserializeOwned,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + MakeRxRef + Clone,
    E: Serialize + DeserializeOwned + Clone,
{
    type RxRef<'rx> = RxResultRef<'rx, T, E>;

    fn to_ref_struct<'rx>(self, cx: Scope<'rx>) -> Self::RxRef<'rx> {
        match self {
            Self::Ok(rx_state) => Self::RxRef::Ok(rx_state.to_ref_struct(cx)),
            Self::Err(err) => Self::RxRef::Err(err),
        }
    }
}

/// The final reference reactive type for [`RxResult`]. This is what you'll get passed
/// to suspense handlers that deal with a field wrapper in [`RxResult`].
pub enum RxResultRef<'rx, T, E>
where
    T: MakeRx + Serialize + DeserializeOwned,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + MakeRxRef + Clone,
    E: Serialize + DeserializeOwned + Clone,
{
    Ok(<T::Rx as MakeRxRef>::RxRef<'rx>),
    Err(E)
}
impl<'rx, T, E> RxRef for RxResultRef<'rx, T, E>
where
    T: MakeRx + Serialize + DeserializeOwned,
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + MakeRxRef + Clone,
    E: Serialize + DeserializeOwned + Clone,
{
    type RxNonRef = T::Rx;
}
