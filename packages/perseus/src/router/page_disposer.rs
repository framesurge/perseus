use std::rc::Rc;
use sycamore::{prelude::RcSignal, reactive::ScopeDisposer};

/// This stores the disposers for user pages so that they can be safely
/// unmounted when the view changes.
///
/// If you're using the `#[template]` macro and the like, you will never need to
/// use this. If you're not using the macros for some reason, you shoudl consult
/// their code to make sure you use this correctly.
#[derive(Clone, Default)]
pub(crate) struct PageDisposer<'app> {
    /// The underlying `ScopeDisposer`. This will initially be `None` before any
    /// views have been rendered.
    ///
    /// There is no way to get this underlying scope disposer, it can only be
    /// set. Hence, we prevent there ever being multiple references to the
    /// underlying `Signal`.
    disposer: RcSignal<Option<ScopeDisposer<'app>>>,
}
impl<'app> PageDisposer<'app> {
    /// Updates the undelrying data structure to hold the given disposer, taking
    /// any previous disposer and disposing it.
    ///
    /// # Safety
    /// This must not be called inside the scope in which the previous disposer
    /// was created.
    pub(crate) unsafe fn update(&self, new_disposer: ScopeDisposer<'app>) {
        // Dispose of any old disposers
        if self.disposer.get().is_some() {
            let old_disposer_rc = self.disposer.take();
            let old_disposer_option = Rc::try_unwrap(old_disposer_rc).unwrap(); // See docs on `disposer` field
            let old_disposer = old_disposer_option.unwrap(); // We're in a conditional that checked this

            // SAFETY: This function is documented to be only called when we're not inside
            // the same scope as we're disposing of.
            old_disposer.dispose();
        }

        self.disposer.set(Some(new_disposer));
    }
}
