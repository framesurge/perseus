use sycamore::prelude::{
    create_signal, provide_context_ref, try_use_context, use_context, Scope, Signal,
};

/// Adds the given value to the given reactive scope inside a `Signal`,
/// replacing a value of that type if one is already present. This returns a
/// reference to the `Signal` inserted.
pub(crate) fn provide_context_signal_replace<T: 'static>(cx: Scope, val: T) -> &Signal<T> {
    if let Some(ctx) = try_use_context::<Signal<T>>(cx) {
        ctx.set(val);
    } else {
        provide_context_ref(cx, create_signal(cx, val));
    }

    use_context(cx)
}
