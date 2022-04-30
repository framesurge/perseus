use sycamore::prelude::{create_signal, provide_context_ref, try_use_context, Scope, Signal};

/// Adds the given value to the given reactive scope inside a `Signal`, replacing a value of that type if one is already present. This returns a reference to the `Signal` inserted.
pub(crate) fn provide_context_signal_replace<T: 'static>(cx: Scope, val: T) -> &Signal<T> {
    try_use_context(cx).unwrap_or_else(|| provide_context_ref(cx, create_signal(cx, val)))
}
