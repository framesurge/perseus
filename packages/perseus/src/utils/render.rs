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
pub(crate) fn render_or_hydrate(view: View, parent: web_sys::Element, force_render: bool) {
    use sycamore::web::NoHydrate;

    // If we're forcing a render (not hydrating), we need to clear content and use regular rendering
    // This happens even when the hydrate feature is enabled, because force_render means
    // this content was not server-rendered and has no hydration markers
    if force_render {
        parent.set_inner_html("");
        // Wrap the view in NoHydrate to disable hydration for this render
        // This ensures we use regular DOM nodes instead of HydrateNodes
        sycamore::web::render_in_scope(|| view! { NoHydrate { (view) } }, &parent);
    } else {
        // Normal hydration path when hydrate feature is enabled
        #[cfg(feature = "hydrate")]
        {
            // Due to Sycamore 0.9.2's hydration architecture, we cannot use hydrate_in_scope
            // directly. The issue is that Perseus's SSR renders only the page template content,
            // but the client wraps this in a router with dynamic views. These dynamic views
            // create marker expectations (<!--/--> comments) that don't exist in the SSR output.
            //
            // As a workaround, we clear the SSR content and render fresh. This means the user
            // will see a brief flash of the SSR content being replaced, but it avoids the
            // hydration marker mismatch panic.
            //
            // TODO: Implement proper hydration by having SSR render through the same
            // router structure, or by investigating Sycamore's partial hydration options.
            parent.set_inner_html("");
            sycamore::web::render_in_scope(|| view, &parent);
        }
        #[cfg(not(feature = "hydrate"))]
        {
            // We have to delete the existing content before we can render the new stuff
            parent.set_inner_html("");
            // Use render_in_scope to stay within the current reactive root
            // This ensures that contexts (like Reactor) from the parent scope remain available
            sycamore::web::render_in_scope(|| view, &parent);
        }
    }
}

/// Renders the given view to a string in a fallible manner.
#[cfg(engine)]
pub(crate) fn ssr_fallible<E>(view_fn: impl FnOnce() -> Result<View, E>) -> Result<String, E> {
    // IMPORTANT: We must call view_fn() INSIDE render_to_string's closure,
    // not before. This is because render_to_string sets IS_HYDRATING = true
    // before calling the view function, which causes elements to be created
    // with hydration keys. If we create the view outside and pass it in,
    // the elements will already be created without hydration keys.

    // We use RefCell to communicate errors out of the closure
    use std::cell::RefCell;
    use std::rc::Rc;

    let error: Rc<RefCell<Option<E>>> = Rc::new(RefCell::new(None));
    let error_clone = Rc::clone(&error);

    let view_str = sycamore::render_to_string(|| {
        match view_fn() {
            Ok(view) => view,
            Err(err) => {
                *error_clone.borrow_mut() = Some(err);
                // Return an empty view in case of error
                sycamore::view! {}
            }
        }
    });

    // Check if an error occurred during view creation
    if let Some(err) = error.borrow_mut().take() {
        return Err(err);
    }

    Ok(view_str)
}
