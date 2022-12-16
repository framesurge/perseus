#[cfg(not(target_arch = "wasm32"))]
use super::super::fn_types::*;
use super::Template;
#[cfg(not(target_arch = "wasm32"))]
use crate::errors::*;
use crate::{
    reactor::Reactor,
    state::{AnyFreeze, MakeRx, MakeRxRef, MakeUnrx, UnreactiveState},
};
#[cfg(not(target_arch = "wasm32"))]
use http::HeaderMap;
use serde::{de::DeserializeOwned, Serialize};
use sycamore::prelude::create_child_scope;
use sycamore::prelude::BoundedScope;
#[cfg(not(target_arch = "wasm32"))]
use sycamore::web::SsrNode;
use sycamore::{prelude::Scope, view::View, web::Html};

impl<G: Html> Template<G> {
    /// Sets the template rendering function to use, if the template takes
    /// state. Templates that do not take state should use `.template()`
    /// instead.
    ///
    /// The closure wrapping this performs will automatically handle suspense
    /// state.
    ///
    /// You will need tp provide this your original state type, so that it can
    /// be made reactive, and the second type parameter is for the actual
    /// function, which can always be left as `_` (e.g.
    /// `.template_with_state::<IndexPageState, _>(index_page)`).
    // Generics are swapped here for nicer manual specification
    pub fn template_with_state<S, F>(mut self, val: F) -> Template<G>
    where
        // The state is made reactive on the child
        F: for<'app, 'child> Fn(
                BoundedScope<'app, 'child>,
                <S::Rx as MakeRxRef>::RxRef<'child>,
            ) -> View<G>
            + Clone
            + Send
            + Sync
            + 'static,
        S: MakeRx + Serialize + DeserializeOwned + Send + Sync + Clone + 'static,
        S::Rx: MakeUnrx<Unrx = S> + AnyFreeze + Clone + MakeRxRef,
    {
        #[cfg(target_arch = "wasm32")]
        let entity_name = self.get_path();
        #[cfg(target_arch = "wasm32")]
        let fallback_fn = self.fallback.clone(); // `Arc`ed, heaven help us
        self.template = Box::new(
            #[allow(unused_variables)]
            move |app_cx, preload_info, template_state, path| {
                let reactor = Reactor::<G>::from_cx(app_cx);
                if self.is_capsule {
                    reactor.get_widget_view::<S, _>(
                        app_cx,
                        path,
                        #[cfg(target_arch = "wasm32")]
                        entity_name.clone(),
                        template_state,
                        #[cfg(target_arch = "wasm32")]
                        preload_info,
                        val.clone(),
                        #[cfg(target_arch = "wasm32")]
                        fallback_fn.as_ref().unwrap(),
                    )
                } else {
                    // This will handle frozen/active state prioritization, etc.
                    let intermediate_state = reactor.get_page_state::<S>(&path, template_state)?;
                    // Run the user's code in a child scope so any effects they start are killed
                    // when the page ends (otherwise we basically get a series of
                    // continuous pseudo-memory leaks, which can also cause accumulations of
                    // listeners on things like the router state)
                    let mut view = View::empty();
                    let disposer = ::sycamore::reactive::create_child_scope(app_cx, |child_cx| {
                        // Compute suspended states
                        #[cfg(target_arch = "wasm32")]
                        intermediate_state.compute_suspense(child_cx);
                        // let ref_struct = intermediate_state.to_ref_struct(child_cx);
                        view = val(child_cx, intermediate_state.to_ref_struct(child_cx));
                    });
                    Ok((view, disposer))
                }
            },
        );
        self
    }
    /// Sets the template rendering function to use, if the template takes
    /// unreactive state.
    pub fn template_with_unreactive_state<F, S>(mut self, val: F) -> Template<G>
    where
        F: Fn(Scope, S) -> View<G> + Clone + Send + Sync + 'static,
        S: MakeRx + Serialize + DeserializeOwned + UnreactiveState + 'static,
        <S as MakeRx>::Rx: AnyFreeze + Clone + MakeUnrx<Unrx = S>,
    {
        #[cfg(target_arch = "wasm32")]
        let entity_name = self.get_path();
        #[cfg(target_arch = "wasm32")]
        let fallback_fn = self.fallback.clone(); // `Arc`ed, heaven help us
        self.template = Box::new(
            #[allow(unused_variables)]
            move |app_cx, preload_info, template_state, path| {
                let reactor = Reactor::<G>::from_cx(app_cx);
                if self.is_capsule {
                    reactor.get_unreactive_widget_view(
                        app_cx,
                        path,
                        #[cfg(target_arch = "wasm32")]
                        entity_name.clone(),
                        template_state,
                        #[cfg(target_arch = "wasm32")]
                        preload_info,
                        val.clone(),
                        #[cfg(target_arch = "wasm32")]
                        fallback_fn.as_ref().unwrap(),
                    )
                } else {
                    // This will handle frozen/active state prioritization, etc.
                    let intermediate_state = reactor.get_page_state::<S>(&path, template_state)?;
                    let mut view = View::empty();
                    let disposer = create_child_scope(app_cx, |child_cx| {
                        // We go back from the unreactive state type wrapper to the base type (since
                        // it's unreactive)
                        view = val(child_cx, intermediate_state.make_unrx());
                    });
                    Ok((view, disposer))
                }
            },
        );
        self
    }

    /// Sets the template rendering function to use for templates that take no
    /// state. Templates that do take state should use
    /// `.template_with_state()` instead.
    pub fn template<F>(mut self, val: F) -> Template<G>
    where
        F: Fn(Scope) -> View<G> + Send + Sync + 'static,
    {
        self.template = Box::new(move |app_cx, _preload_info, _template_state, path| {
            let reactor = Reactor::<G>::from_cx(app_cx);
            // Declare that this page/widget will never take any state to enable full
            // caching
            reactor.register_no_state(&path, self.is_capsule);

            // Nicely, if this is a widget, this means there need be no network requests
            // at all!
            let mut view = View::empty();
            let disposer = ::sycamore::reactive::create_child_scope(app_cx, |child_cx| {
                view = val(child_cx);
            });
            Ok((view, disposer))
        });
        self
    }

    /// Sets the document `<head>` rendering function to use. The [`View`]
    /// produced by this will only be rendered on the engine-side, and will
    /// *not* be reactive (since it only contains metadata).
    ///
    /// This is for heads that do require state. Those that do not should use
    /// `.head()` instead.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn head_with_state<S, V>(
        mut self,
        val: impl Fn(Scope, S) -> V + Send + Sync + 'static,
    ) -> Template<G>
    where
        S: Serialize + DeserializeOwned + MakeRx + 'static,
        V: Into<GeneratorResult<View<SsrNode>>>,
    {
        let template_name = self.get_path();
        self.head = Box::new(move |cx, template_state| {
            // Make sure now that there is actually state
            if template_state.is_empty() {
                return Err(ClientError::InvariantError(ClientInvariantError::NoState).into());
            }
            // Declare a type on the untyped state (this doesn't perform any conversions,
            // but the type we declare may be invalid)
            let typed_state = template_state.change_type::<S>();

            let state =
                match typed_state.into_concrete() {
                    Ok(state) => state,
                    Err(err) => {
                        return Err(ClientError::InvariantError(
                            ClientInvariantError::InvalidState { source: err },
                        )
                        .into())
                    }
                };

            let template_name = template_name.clone();
            val(cx, state)
                .into()
                .into_server_result("head", template_name)
        });
        self
    }
    /// Sets the document `<head>` rendering function to use. The [`View`]
    /// produced by this will only be rendered on the engine-side, and will
    /// *not* be reactive (since it only contains metadata).
    ///
    /// This is for heads that do require state. Those that do not should use
    /// `.head()` instead.
    #[cfg(target_arch = "wasm32")]
    pub fn head_with_state(self, _val: impl Fn() + 'static) -> Self {
        self
    }

    /// Sets the function to set headers. This will override Perseus' inbuilt
    /// header defaults. This should only be used when your header-setting
    /// requires knowing the state.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_headers_with_state<S, V>(
        mut self,
        val: impl Fn(S) -> V + Send + Sync + 'static,
    ) -> Template<G>
    where
        S: Serialize + DeserializeOwned + MakeRx + 'static,
        V: Into<GeneratorResult<HeaderMap>>,
    {
        let template_name = self.get_path();
        self.set_headers = Box::new(move |template_state| {
            // Make sure now that there is actually state
            if template_state.is_empty() {
                return Err(ClientError::InvariantError(ClientInvariantError::NoState).into());
            }
            // Declare a type on the untyped state (this doesn't perform any conversions,
            // but the type we declare may be invalid)
            let typed_state = template_state.change_type::<S>();

            let state =
                match typed_state.into_concrete() {
                    Ok(state) => state,
                    Err(err) => {
                        return Err(ClientError::InvariantError(
                            ClientInvariantError::InvalidState { source: err },
                        )
                        .into())
                    }
                };

            let template_name = template_name.clone();
            val(state)
                .into()
                .into_server_result("set_headers", template_name)
        });
        self
    }
    /// Sets the function to set headers. This will override Perseus' inbuilt
    /// header defaults. This should only be used when your header-setting
    /// requires knowing the state.
    #[cfg(target_arch = "wasm32")]
    pub fn set_headers_with_state(self, _val: impl Fn() + 'static) -> Template<G> {
        self
    }
}