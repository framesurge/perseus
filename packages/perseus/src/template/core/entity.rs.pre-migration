use std::{collections::HashMap, ops::Deref};

use sycamore::web::Html;

use super::TemplateInner;

/// An internal container over a [`TemplateInner`]. Conceptually,
/// this represents *either* a template or a capsule within Perseus. Both
/// [`Template`] and [`Capsule`] simply wrap this with their own unique methods.
///
/// You can determine if this is a capsule or not by checking the underlying
/// `is_capsule` property.
///
/// # Capsule specifics
///
/// Although this functionally represents either a template or a capsule, there
/// are some parts of capsule functionality that are only accessible through the
/// `Capsule` type itself, such as fallback views and properties. This is fine,
/// however, as capsules are used by calling a component method on them, meaning
/// the widget rendering process always has access to the capsule itself.
#[derive(Debug)]
pub struct Entity<G: Html>(TemplateInner<G>);

impl<G: Html> From<TemplateInner<G>> for Entity<G> {
    fn from(val: TemplateInner<G>) -> Self {
        Self(val)
    }
}

// Immutable methods should be able to be called such that this can be treated
// as a template/capsule
impl<G: Html> std::ops::Deref for Entity<G> {
    type Target = TemplateInner<G>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// An alias for a map of entities, keyed by their names/root paths.
pub type EntityMap<G> = HashMap<String, Forever<Entity<G>>>;

/// A helpful wrapper type that allows something to be stored as either an owned
/// type or a static reference, which prevents unnecessary memory leaks when
/// handling user-provided templates and capsules, maintaining compatibility
/// between the static and function definition patterns (if that makes no sense,
/// see the book).
///
/// This is named `Forever` because it guarantees that what it holds is
/// accessible for the lifetime of the holder, no matter what that is.
#[derive(Debug)]
pub enum Forever<T: 'static> {
    Owned(T),
    StaticRef(&'static T),
}
impl<T: 'static> Deref for Forever<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match &self {
            // Bear in mind that this is implicitly a reference because we're working with `&self`
            Self::Owned(val) => val,
            Self::StaticRef(val) => val,
        }
    }
}
impl<T: 'static> From<T> for Forever<T> {
    fn from(val: T) -> Self {
        Self::Owned(val)
    }
}
impl<T: 'static> From<&'static T> for Forever<T> {
    fn from(val: &'static T) -> Self {
        Self::StaticRef(val)
    }
}
// Convenience conversions to reduce the burden in `init.rs`
