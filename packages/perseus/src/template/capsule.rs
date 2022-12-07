use std::ops::Deref;
use sycamore::{prelude::Scope, view::View, web::Html};
use super::Template;


/// A *capsule*, a special type of template in Perseus that can also accept
/// *properties*. Capsules are basically a very special type of Sycamore component
/// that can integrate fully with Perseus' state platform, generating their own states
/// at build-time, request-time, etc. They're then used in one or more pages, and provided
/// extra properties.
pub struct Capsule<G: Html> {
    /// The underlying template (since capsules are just a special type of template).
    pub(crate) template: Template<G>,
    /// A function that returns the fallback view to be rendered between when the page is ready
    /// and when the capsule's state has been fetched.
    ///
    /// Note that this starts as `None`, but, if it's not set, `PerseusApp` will panic. So, for later
    /// code, this can be assumed to be always `Some`.
    pub(crate) fallback: Option<Box<dyn Fn(Scope) -> View<G> + Send + Sync>>,
}
impl<G: Html> std::fmt::Debug for Capsule<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Capsule")
            // TODO
            .finish()
    }
}
impl<G: Html> Capsule<G> {
    /// Creates a new [`Capsule`] with the given path. The argument provided here functions
    /// in the same way as the argument given to [`Template`] does.
    pub fn new(path: impl Into<String> + std::fmt::Display) -> Self {
        // We create a template with this path, and then turn it into a capsule
        let mut template = Template::new(path);
        template.is_capsule = true;
        Self {
            template,
            fallback: None,
        }
    }
    /// Declares the fallback view to render for this capsule. When Perseus renders a page
    /// of your app, it fetches the page itself, along with all the capsules it needs.
    /// If the page is ready before all the capsules, then it will be displayed immediately,
    /// with fallback views for the capsules that aren't ready yet. Once they are ready, they
    /// will be updated.
    ///
    /// This fallback view cannot access any of the state that the capsule generated, but it can
    /// access any properties provided to it by the page, along with a translator and the like.
    /// This view is fully reactive, it just doesn't have the state yet.
    ///
    /// **Warning:** if you do not set a fallback view for a capsule, your app will not compile!
    // TODO This function should take properties
    pub fn fallback(mut self, view: impl Fn (Scope) -> View<G> + Send + Sync + 'static) -> Self {
        self.fallback = Some(Box::new(view));
        self
    }
    /// Sets the fallback for this capsule to be an empty view.
    ///
    /// You should be careful using this function in production, since it is very often not
    /// what you actually want (especially since empty views have no size, which may compromise
    /// your layouts: be sure to test this).
    pub fn empty_fallback(mut self) -> Self {
        self.fallback = Some(Box::new(|cx| sycamore::view! { cx, }));
        self
    }
}

// We want all the methods of `Template` directly accessible
impl<G: Html> Deref for Capsule<G> {
    type Target = Template<G>;

    fn deref(&self) -> &Self::Target {
        &self.template
    }
}
