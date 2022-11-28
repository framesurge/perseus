use std::collections::HashMap;

use crate::template::TemplateState;

/// A wrapper for state that will be fetched asynchronously by the browser *after*
/// the page it is needed by has been loaded, to reduce page size and maximize load speed.
pub enum Delayed<T> {
    /// The state is ready for use.
    Ready(T),
    /// The state is being fetched.
    Waiting,
    /// There was an error while trying to fetch the state. Given
    /// the page itself was successfully loaded, this is probably either
    /// a server corruption or a network failure (more likely the latter).
    Error,
}
impl<T> Delayed<T> {
    /// Takes the underlying value out of the wrapper if it's ready, replacing it
    /// with `Delayed::Waiting`. If `self` is not `Delayed::Ready`, this will return `None`.
    pub fn take(mut self) -> Option<T> {
        match self {
            Delayed::Ready(val) => {
                self = Delayed::Waiting;
                Some(val)
            },
            _ => None,
        }
    }
}

// For convenience, we want engine-side generations to be able to create delayed state with
// `.into()`
impl<T> From<T> for Delayed<T> {
    fn from(val: T) -> Self {
        Delayed::Ready(val)
    }
}

/// A container `struct` for both the delayed and non-delayed components of a template's state.
pub struct DelayedParts {
    /// A map of the keys of delayed states to their actual states. Each key must be unique per-page.
    pub delayed: HashMap<String, TemplateState>,
    /// The state that was not delayed, with delayed parts filled in as [`Delayed::Waiting`],
    pub undelayed: TemplateState,
}
