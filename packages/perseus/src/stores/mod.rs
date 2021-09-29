/// Utilities for working with immutable stores.
pub mod immutable;
/// Utilities for working with mutable stores.
pub mod mutable;

pub use immutable::ImmutableStore;
pub use mutable::{FsMutableStore, MutableStore};
