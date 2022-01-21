use std::any::Any;

/// A trait for `struct`s that can be made reactive. Typically, this will be derived with the `#[make_rx]` macro, though it can be implemented manually if you have more niche requirements.
pub trait MakeRx {
    /// The type of the reactive version that we'll convert to. By having this as an associated type, we can associate the reactive type with the unreactive, meaning greater inference
    /// and fewer arguments that the user needs to provide to macros.
    type Rx;
    /// Transforms an instance of the `struct` into its reactive version.
    fn make_rx(self) -> Self::Rx;
}

/// A trait for reactive `struct`s that can be made un-reactive. This is the opposite of `MakeRx`, and is intended particularly for state freezing. Like `MakeRx`, this will usually be derived
/// automatically with the `#[make_rx]` macro, but you can also implement it manually.
pub trait MakeUnrx {
    /// The type of the unreactive version that we'll convert to.
    type Unrx;
    /// Transforms an instance of the `struct` into its unreactive version. By having this as an associated type, we can associate the reactive type with the unreactive, meaning greater inference
    /// and fewer arguments that the user needs to provide to macros.
    fn make_unrx(self) -> Self::Unrx;
}

/// A trait for reactive `struct`s that can be made unreactive and serialized to a `String`. `struct`s that implement this should implement `MakeUnrx` for simplicity, but they technically don't have
/// to (they always do in Perseus macro-generated code).
pub trait Freeze {
    /// 'Freezes' the reactive `struct` by making it unreactive and converting it to a `String`.
    fn freeze(&self) -> String;
}

// Perseus initializes the global state as an `Option::<()>::None`, so it has to implement `Freeze`. It may seem silly, because we wouldn't want to freeze the global state if it hadn't been
// initialized, but that means it's unmodified from the server, so there would be no point in freezing it (just as there'd be no point in freezing the router state).
impl Freeze for Option<()> {
    fn freeze(&self) -> String {
        serde_json::to_string(&Option::<()>::None).unwrap()
    }
}

/// A convenience super-trait for `Freeze`able things that can be downcast to concrete types.
pub trait AnyFreeze: Freeze + Any {
    /// Gives `&dyn Any` to enable downcasting.
    fn as_any(&self) -> &dyn Any;
}
impl<T: Any + Freeze> AnyFreeze for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
