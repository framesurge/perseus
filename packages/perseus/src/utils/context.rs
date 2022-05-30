use sycamore::prelude::{create_signal, use_context_or_else_ref, Scope, Signal};

/// Adds the given value to the given reactive scope inside a `Signal`, replacing a value of that type if one is already present. This returns a reference to the `Signal` inserted.
pub(crate) fn provide_context_signal_replace<T: 'static>(cx: Scope, val: T) -> &Signal<T> {
    use_context_or_else_ref(cx, || create_signal(cx, val))
}
