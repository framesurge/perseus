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
