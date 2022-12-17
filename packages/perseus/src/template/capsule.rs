use super::{Entity, Template, TemplateInner};
use std::{ops::Deref, sync::Arc};
use sycamore::{prelude::Scope, view::View, web::Html};

/// A *capsule*, a special type of template in Perseus that can also accept
/// *properties*. Capsules are basically a very special type of Sycamore
/// component that can integrate fully with Perseus' state platform, generating
/// their own states at build-time, request-time, etc. They're then used in one
/// or more pages, and provided extra properties.
pub struct Capsule<G: Html> {
    /// The underlying entity (in this case, a capsule).
    pub(crate) inner: Entity<G>,
}
impl<G: Html> std::fmt::Debug for Capsule<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Capsule")
            .finish()
    }
}

/// The equivalent of [`TemplateInner`] for capsules.
///
/// # Implementation
///
/// Really, this is just a wrapper over [`TemplateInner`] with the additional
/// methods capsules need. For example, templates have fallback views on their
/// own, they just don't use them, and there's no way to set them as an end user.
/// This means Perseus can treat templates and capsules in the same way internally,
/// since they both have the same representation. Types like this are mere convenience
/// wrappers.
pub struct CapsuleInner<G: Html> {
    template_inner: TemplateInner<G>
}

impl<G: Html> Capsule<G> {
    /// Creates a new [`Capsule`] with the given path. The argument provided
    /// here functions in the same way as the argument given to [`Template`]
    /// does.
    ///
    /// You will need to call `.build()` when you're done with this.
    pub fn new(path: &str) -> CapsuleInner<G> {
        // We create a template with this path, and then turn it into a capsule
        let mut template_inner = Template::new(path);
        template_inner.is_capsule = true;
        CapsuleInner { template_inner }
    }
}
impl<G: Html> CapsuleInner<G> {
    /// Declares the fallback view to render for this capsule. When Perseus
    /// renders a page of your app, it fetches the page itself, along with
    /// all the capsules it needs. If the page is ready before all the
    /// capsules, then it will be displayed immediately, with fallback views
    /// for the capsules that aren't ready yet. Once they are ready, they
    /// will be updated.
    ///
    /// This fallback view cannot access any of the state that the capsule
    /// generated, but it can access any properties provided to it by the
    /// page, along with a translator and the like. This view is fully
    /// reactive, it just doesn't have the state yet.
    ///
    /// **Warning:** if you do not set a fallback view for a capsule, your app
    /// will not compile!
    pub fn fallback(mut self, view: impl Fn(Scope) -> View<G> + Send + Sync + 'static) -> Self {
        {
            self.template_inner.fallback = Some(Arc::new(view));
        }
        self
    }
    /// Sets the fallback for this capsule to be an empty view.
    ///
    /// You should be careful using this function in production, since it is
    /// very often not what you actually want (especially since empty views
    /// have no size, which may compromise your layouts: be sure to test
    /// this).
    pub fn empty_fallback(mut self) -> Self {
        {
            self.template_inner.fallback = Some(Arc::new(|cx| sycamore::view! { cx, }));
        }
        self
    }
    /// Builds a full [`Capsule`] from this [`CapsuleInner`], consuming it in the process.
    /// Once called, the capsule cannot be modified anymore, and it will be placed into a
    /// smart pointer, allowing it to be cloned freely with minimal costs.
    ///
    /// You should call this just before you return your capsule.
    pub fn build(self) -> Capsule<G> {
        Capsule { inner: Entity::from(self.template_inner) }
    }
}

// We want all the methods of `TemplateInner` directly accessible
impl<G: Html> Deref for CapsuleInner<G> {
    type Target = TemplateInner<G>;

    fn deref(&self) -> &Self::Target {
        &self.template_inner
    }
}
