use crate::{
    errors::ClientError,
    path::PathMaybeWithLocale,
    reactor::Reactor,
    state::{AnyFreeze, MakeRx, MakeUnrx, TemplateState, UnreactiveState},
};

use super::{Entity, PreloadInfo, TemplateInner};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use sycamore::{
    prelude::{create_child_scope, create_scope, BoundedScope, Scope, ScopeDisposer},
    view::View,
    web::Html,
};

/// The type of functions that are given a state and properties to render a
/// widget.
pub(crate) type CapsuleFn<G, P> = Box<
    dyn for<'a> Fn(
            Scope<'a>,
            PreloadInfo,
            TemplateState,
            P,
            PathMaybeWithLocale,
        ) -> Result<(View<G>, ScopeDisposer<'a>), ClientError>
        + Send
        + Sync,
>;

/// A *capsule*, a special type of template in Perseus that can also accept
/// *properties*. Capsules are basically a very special type of Sycamore
/// component that can integrate fully with Perseus' state platform, generating
/// their own states at build-time, request-time, etc. They're then used in one
/// or more pages, and provided extra properties.
///
/// Note that capsules store their view functions and fallbacks independently of
/// their underlying templates, for properties support.
pub struct Capsule<G: Html, P: Clone + 'static> {
    /// The underlying entity (in this case, a capsule).
    pub(crate) inner: Entity<G>,
    /// The capsule rendering function, which is a template function that also
    /// takes properties.
    capsule_view: CapsuleFn<G, P>,
    /// A function that returns the fallback view to be rendered between when
    /// the page is ready and when the capsule's state has been fetched.
    ///
    /// Note that this starts as `None`, but, if it's not set, `PerseusApp` will
    /// panic. So, for later code, this can be assumed to be always `Some`.
    ///
    /// This will not be defined for templates, only for capsules.
    #[allow(clippy::type_complexity)]
    pub(crate) fallback: Option<Arc<dyn Fn(Scope, P) -> View<G> + Send + Sync>>,
}
impl<G: Html, P: Clone + 'static> std::fmt::Debug for Capsule<G, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Capsule").finish()
    }
}

/// The equivalent of [`TemplateInner`] for capsules.
///
/// # Implementation
///
/// Really, this is just a wrapper over [`TemplateInner`] with the additional
/// methods capsules need. For example, templates have fallback views on their
/// own, they just don't use them, and there's no way to set them as an end
/// user. This means Perseus can treat templates and capsules in the same way
/// internally, since they both have the same representation. Types like this
/// are mere convenience wrappers.
pub struct CapsuleInner<G: Html, P: Clone + 'static> {
    template_inner: TemplateInner<G>,
    capsule_view: CapsuleFn<G, P>,
    /// A function that returns the fallback view to be rendered between when
    /// the page is ready and when the capsule's state has been fetched.
    ///
    /// Note that this starts as `None`, but, if it's not set, `PerseusApp` will
    /// panic. So, for later code, this can be assumed to be always `Some`.
    ///
    /// This will not be defined for templates, only for capsules.
    #[allow(clippy::type_complexity)]
    pub(crate) fallback: Option<Arc<dyn Fn(Scope, P) -> View<G> + Send + Sync>>,
}
impl<G: Html, P: Clone + 'static> std::fmt::Debug for CapsuleInner<G, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CapsuleInner")
            .field("template_inner", &self.template_inner)
            .finish_non_exhaustive()
    }
}

impl<G: Html, P: Clone + 'static> Capsule<G, P> {
    /// Creates a new [`CapsuleInner`] from the given [`TemplateInner`]. In
    /// Perseus, capsules are really just special kinds of pages, so you
    /// create them by first creating the underlying template. To make sure
    /// you get a capsule instead of a template, you just don't call
    /// `.build()` on the template, instead passing the [`TemplateInner`] to
    /// this function.
    ///
    /// **Warning:** [`TemplateInner`] has methods like `.view()` and
    /// `.view_with_state()` for setting the views of your templates, but you
    /// shouldn't use those when you're building a capsule, because those
    /// functions won't let you use *properties* that can be passed from
    /// pages that use your capsule. Instead, construct a [`TemplateInner`]
    /// that has no views, and then use the `.view()` etc. functions on
    /// [`CapsuleInner`] instead. (Unfortunately, dereferncing doesn't work
    /// with the builder pattern, so this is the best we can do in Rust
    /// right now.)
    ///
    /// You will need to call `.build()` when you're done with this to get a
    /// full [`Capsule`].
    pub fn build(mut template_inner: TemplateInner<G>) -> CapsuleInner<G, P> {
        template_inner.is_capsule = true;
        // Wipe the template's view function to make sure the errors aren't obscenely
        // weird
        template_inner.view = Box::new(|_, _, _, _| Ok((View::empty(), create_scope(|_| {}))));
        CapsuleInner {
            template_inner,
            capsule_view: Box::new(|_, _, _, _, _| Ok((View::empty(), create_scope(|_| {})))),
            // This must be manually specified
            fallback: None,
        }
    }

    /// Executes the user-given function that renders the *widget* on the
    /// client-side ONLY. This takes in an existing global state. This will
    /// ignore its internal scope disposer, since the given scope **must**
    /// be a page-level scope, which will be disposed from the root when the
    /// page changes, thereby disposing of all the child scopes, like those
    /// used for widgets.
    ///
    /// This should NOT be used to render pages!
    #[cfg(target_arch = "wasm32")]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn render_widget_for_template_client(
        &self,
        path: PathMaybeWithLocale,
        caller_path: &PathMaybeWithLocale,
        props: P,
        cx: Scope,
        preload_info: PreloadInfo,
    ) -> Result<View<G>, ClientError> {
        // The template state is ignored by widgets, they fetch it themselves
        // asynchronously
        let (view, _disposer) = (self.capsule_view)(
            cx,
            preload_info,
            TemplateState::empty(),
            props,
            path.clone(),
        )?;
        // The widget will have been registered in the state store, so declare the
        // dependency
        let reactor = Reactor::<G>::from_cx(cx);
        reactor.state_store.declare_dependency(&path, caller_path);
        Ok(view)
    }
    /// Executes the user-given function that renders the capsule on the
    /// server-side ONLY. This takes the scope from a previous call of
    /// `.render_for_template_server()`, assuming the reactor has already
    /// been fully instantiated.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn render_widget_for_template_server(
        &self,
        path: PathMaybeWithLocale,
        state: TemplateState,
        props: P,
        cx: Scope,
    ) -> Result<View<G>, ClientError> {
        // This is used for widget preloading, which doesn't occur on the engine-side
        let preload_info = PreloadInfo {};
        // We don't care about the scope disposer, since this scope is unique anyway
        let (view, _) = (self.capsule_view)(cx, preload_info, state, props, path)?;
        Ok(view)
    }
}
impl<G: Html, P: Clone + 'static> CapsuleInner<G, P> {
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
    pub fn fallback(mut self, view: impl Fn(Scope, P) -> View<G> + Send + Sync + 'static) -> Self {
        {
            self.fallback = Some(Arc::new(view));
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
            self.fallback = Some(Arc::new(|cx, _| sycamore::view! { cx, }));
        }
        self
    }
    /// Builds a full [`Capsule`] from this [`CapsuleInner`], consuming it in
    /// the process. Once called, the capsule cannot be modified anymore,
    /// and it will be placed into a smart pointer, allowing it to be cloned
    /// freely with minimal costs.
    ///
    /// You should call this just before you return your capsule.
    pub fn build(self) -> Capsule<G, P> {
        Capsule {
            inner: Entity::from(self.template_inner),
            capsule_view: self.capsule_view,
            fallback: self.fallback,
        }
    }

    // --- Shadow `.view()` functions for properties ---
    // These will set dummy closures for the underlying templates, as capsules
    // maintain their own separate functions, which can use properties in line
    // with the known generics. As capsules are themselves used as their own
    // components, these functions can therefore be accessed.

    /// Sets the rendering function to use for capsules that take reactive
    /// state. Capsules that do not take state should use `.view()` instead.
    ///
    /// The closure wrapping this performs will automatically handle suspense
    /// state.
    // Generics are swapped here for nicer manual specification
    pub fn view_with_state<I, F>(mut self, val: F) -> Self
    where
        // The state is made reactive on the child
        F: for<'app, 'child> Fn(BoundedScope<'app, 'child>, &'child I, P) -> View<G>
            + Clone
            + Send
            + Sync
            + 'static,
        I: MakeUnrx + AnyFreeze + Clone,
        I::Unrx: MakeRx<Rx = I> + Serialize + DeserializeOwned + Send + Sync + Clone + 'static,
    {
        self.template_inner.view =
            Box::new(|_, _, _, _| panic!("attempted to call template rendering logic for widget"));
        #[cfg(target_arch = "wasm32")]
        let entity_name = self.template_inner.get_path();
        #[cfg(target_arch = "wasm32")]
        let fallback_fn = self.fallback.clone(); // `Arc`ed, heaven help us
        self.capsule_view = Box::new(
            #[allow(unused_variables)]
            move |app_cx, preload_info, template_state, props, path| {
                let reactor = Reactor::<G>::from_cx(app_cx);
                reactor.get_widget_view::<I::Unrx, _, P>(
                    app_cx,
                    path,
                    #[cfg(target_arch = "wasm32")]
                    entity_name.clone(),
                    template_state,
                    props,
                    #[cfg(target_arch = "wasm32")]
                    preload_info,
                    val.clone(),
                    #[cfg(target_arch = "wasm32")]
                    fallback_fn.as_ref().unwrap(),
                )
            },
        );
        self
    }
    /// Sets the rendering function to use for capsules that take unreactive
    /// state.
    pub fn view_with_unreactive_state<F, S>(mut self, val: F) -> Self
    where
        F: Fn(Scope, S, P) -> View<G> + Clone + Send + Sync + 'static,
        S: MakeRx + Serialize + DeserializeOwned + UnreactiveState + 'static,
        <S as MakeRx>::Rx: AnyFreeze + Clone + MakeUnrx<Unrx = S>,
    {
        self.template_inner.view =
            Box::new(|_, _, _, _| panic!("attempted to call template rendering logic for widget"));
        #[cfg(target_arch = "wasm32")]
        let entity_name = self.template_inner.get_path();
        #[cfg(target_arch = "wasm32")]
        let fallback_fn = self.fallback.clone(); // `Arc`ed, heaven help us
        self.capsule_view = Box::new(
            #[allow(unused_variables)]
            move |app_cx, preload_info, template_state, props, path| {
                let reactor = Reactor::<G>::from_cx(app_cx);
                reactor.get_unreactive_widget_view(
                    app_cx,
                    path,
                    #[cfg(target_arch = "wasm32")]
                    entity_name.clone(),
                    template_state,
                    props,
                    #[cfg(target_arch = "wasm32")]
                    preload_info,
                    val.clone(),
                    #[cfg(target_arch = "wasm32")]
                    fallback_fn.as_ref().unwrap(),
                )
            },
        );
        self
    }

    /// Sets the rendering function for capsules that take no state. Capsules
    /// that do take state should use `.view_with_state()` instead.
    pub fn view<F>(mut self, val: F) -> Self
    where
        F: Fn(Scope, P) -> View<G> + Send + Sync + 'static,
    {
        self.template_inner.view =
            Box::new(|_, _, _, _| panic!("attempted to call template rendering logic for widget"));
        self.capsule_view = Box::new(move |app_cx, _preload_info, _template_state, props, path| {
            let reactor = Reactor::<G>::from_cx(app_cx);
            // Declare that this page/widget will never take any state to enable full
            // caching
            reactor.register_no_state(&path, true);

            // Nicely, if this is a widget, this means there need be no network requests
            // at all!
            let mut view = View::empty();
            let disposer = create_child_scope(app_cx, |child_cx| {
                view = val(child_cx, props);
            });
            Ok((view, disposer))
        });
        self
    }
}
