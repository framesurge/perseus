use sycamore::prelude::*;

/// Renders or hydrates the given view to the given node,
/// depending on feature flags. This will automatically handle
/// proper scoping.
///
/// This has the option to force a render by ignoring the initial elements.
///
/// **Warning:** if hydration is being used, it is expected that
/// the given view was created inside a `with_hydration_context()` closure.
#[cfg(any(client, doc))]
#[allow(unused_variables)]
pub(crate) fn render_or_hydrate(
    view: View,
    parent: web_sys::Element,
    force_render: bool,
) {
    #[cfg(feature = "hydrate")]
    {
        // If we're forcing a proper render, then we'll have to remove existing content
        if force_render {
            parent.set_inner_html("");
            sycamore::web::render_to(|| view, &parent);
        } else {
            sycamore::web::hydrate_to(|| view, &parent);
        }
    }
    #[cfg(not(feature = "hydrate"))]
    {
        // We have to delete the existing content before we can render the new stuff
        parent.set_inner_html("");
        sycamore::web::render_to(|| view, &parent);
    }
}

/// Renders the given view to a string in a fallible manner.
#[cfg(engine)]
pub(crate) fn ssr_fallible<E>(
    view_fn: impl FnOnce() -> Result<View, E>,
) -> Result<String, E> {
    // Execute the view function and render to string if successful
    let view_res = view_fn();
    match view_res {
        Ok(view) => {
            let view_str = sycamore::render_to_string(|| view);
            Ok(view_str)
        }
        Err(err) => Err(err),
    }
}
