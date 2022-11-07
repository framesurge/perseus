use serde::{Deserialize, Serialize};
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
