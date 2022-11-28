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

// For convenience, we want engine-side generations to be able to create delayed state with
// `.into()`
impl<T> From<T> for Delayed<T> {
    fn from(val: T) -> Self {
        Delayed::Ready(val)
    }
}
