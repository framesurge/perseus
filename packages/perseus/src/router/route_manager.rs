use super::PageDisposer;
use sycamore::{prelude::{Scope, ScopeDisposer, Signal, View, create_signal}, web::Html};

/// The internals that allow Perseus to manage the many routes of an app, including
/// child scope disposal. This should almost never be interacted with by end users!
///
/// This takes the lifetime of the whole app's root scope. Note that this is not put
/// in `RenderCtx`, since it should not be accessible except through raw templates.
///
/// Note that this is used instead of the component parts to ensure lifetime sameness.
#[derive(Clone)]
pub struct RouteManager<'cx, G: Html> {
    page_disposer: PageDisposer<'cx>,
    // We occasionally need to `.get()` and `.take()` this
    pub(crate) view: &'cx Signal<View<G>>,
}
// We don't allow direct field access to minimize the likelihood to users shooting themselves in the foot (or, in this case, kidney)
impl<'cx, G: Html> RouteManager<'cx, G> {
    /// Creates a new route manager, with an empty view and no scopes to dispose of yet.
    pub(crate) fn new(cx: Scope<'cx>) -> Self {
        Self {
            page_disposer: PageDisposer::new(cx),
            view: create_signal(cx, View::empty()),
        }
    }
    /// Updates the current view of the app. The argument here will be rendered to the root of the app.
    ///
    /// This should NEVER be invoked outside the typical lifecycle of Perseus routing! If you want to render
    /// and error page or the like, use that API, not this one!
    pub fn update_view(&self, new_view: View<G>) {
        self.view.set(new_view);
    }
    /// Updates the underlying scope disposer. See the docs for [`PageDisposer`] for more information.
    pub fn update_disposer(&mut self, new_disposer: ScopeDisposer<'cx>) {
        self.page_disposer.update(new_disposer);
    }
}
