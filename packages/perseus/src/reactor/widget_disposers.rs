use std::{cell::RefCell, rc::Rc};
use sycamore::prelude::ScopeDisposer;

/// A wrapper over a list of [`ScopeDisposer`]s for widgets, which can be added
/// to by widget renders, or all disposed of simultaneously. While it's not UB
/// to dispose of these at different times, it would be exceptionally odd, and
/// may undermine some future cross-widget state system.
///
/// Note that there is no method to extract scope disposers from this, except by
/// clearing them, meaning we have control over the references to the underlying
/// `Rc`.
#[derive(Default)]
pub(crate) struct WidgetDisposers<'app>(Rc<RefCell<Vec<ScopeDisposer<'app>>>>);
impl<'app> WidgetDisposers<'app> {
    /// Adds the given disposer to the internal list.
    pub(crate) fn add_disposer(&self, disposer: ScopeDisposer<'app>) {
        self.0.borrow_mut().push(disposer);
    }
    /// Runs all the stored disposers in the reverse of the order they were
    /// added. The order shouldn't matter here, but we're being as safe as
    /// possible.
    ///
    /// SAFETY: This *will* cause undefined behavior if it is called inside any
    /// of the scopes it disposes. In other words, this is only ever safe at the
    /// top level.
    pub(crate) unsafe fn dispose(&self) {
        // This updates the internal holding to `Vec::default()`
        let list = self.0.take();
        for disposer in list.into_iter().rev() {
            disposer.dispose()
        }
    }
}
