use std::cell::RefCell;
use std::rc::Rc;

type Disposer = Box<dyn FnOnce()>;

/// This stores the disposers for user pages so that they can be safely
/// unmounted when the view changes.
///
/// If you're using the `#[template]` macro and the like, you will never need to
/// use this. If you're not using the macros for some reason, you shoudl consult
/// their code to make sure you use this correctly.
#[derive(Clone, Default)]
pub(crate) struct PageDisposer {
    /// The underlying disposer function. This will initially be `None` before any
    /// views have been rendered.
    ///
    /// There is no way to get this underlying disposer function, it can only be
    /// set. Hence, we prevent there ever being multiple references to the
    /// underlying `Signal`.
    disposer: Rc<RefCell<Option<Disposer>>>,
}
impl PageDisposer {
    /// Updates the underlying data structure to hold the given disposer, taking
    /// any previous disposer and disposing it.
    ///
    /// # Safety
    /// This must not be called inside a scope in which the previous disposer
    /// was created.
    pub(crate) fn update(&self, new_disposer: Disposer) {
        // Dispose of any old disposers
        if let Some(old_disposer) = self.disposer.replace(Some(new_disposer)) {
            // SAFETY: This function is documented to be only called when we're not inside
            // same scope as we're disposing of.
            old_disposer();
        }
    }
}
