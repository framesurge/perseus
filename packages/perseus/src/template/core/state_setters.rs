#[cfg(engine)]
use super::super::fn_types::*;
use super::TemplateInner;
#[cfg(engine)]
use crate::errors::*;
use crate::{
    reactor::Reactor,
    state::{AnyFreeze, MakeRx, MakeUnrx, UnreactiveState},
};
#[cfg(engine)]
use http::HeaderMap;
use serde::{de::DeserializeOwned, Serialize};
use sycamore::reactive::create_child_scope;
#[cfg(engine)]
use sycamore::web::SsrNode;
use sycamore::prelude::*;

impl TemplateInner {
    // The view functions below are shadowed for widgets, and therefore these
    // definitions only apply to templates, not capsules!

    /// Sets the template rendering function to use, if the template takes
    /// state. Templates that do not take state should use `.template()`
    /// instead.
    ///
    /// The closure wrapping this performs will automatically handle suspense
    /// state.
    // Generics are swapped here for nicer manual specification
    pub fn view_with_state<I, F>(mut self, val: F) -> Self
    where
        // The state is made reactive on the child
        // In Sycamore 0.9.2, reactive state is usually Copy (contains Copy signals),
        // but for nested/suspense state it might not be, so we only require Clone
        F: Fn(I) -> View + Send + Sync + 'static,
        I: MakeUnrx + AnyFreeze + Clone,
        I::Unrx: MakeRx<Rx = I> + Serialize + DeserializeOwned + Send + Sync + Clone + 'static,
    {
        self.view = Box::new(
            #[allow(unused_variables)]
            move |preload_info, template_state, path| {
                let reactor = Reactor::from_cx();
                // This will handle frozen/active state prioritization, etc.
                let intermediate_state = reactor.get_page_state::<I::Unrx>(&path, template_state)?;
                // Run the user's code in a child scope so any effects they start are killed
                // when the page ends (otherwise we basically get a series of
                // continuous pseudo-memory leaks, which can also cause accumulations of
                // listeners on things like the router state)
                let mut view = View::new();
                let disposer = create_child_scope(|| {
                    // Compute suspended states
                    #[cfg(any(client, doc))]
                    intermediate_state.compute_suspense();

                    // In Sycamore 0.9.2, reactive state is usually Copy (contains only Copy signals)
                    // If it's Copy, we pass by value; if not Copy, we clone it
                    // The compiler will optimize away the clone for Copy types
                    view = val(intermediate_state.clone());
                });
                Ok((view, disposer))
            },
        );
        self
    }
    /// Sets the template rendering function to use, if the template takes
    /// unreactive state.
    pub fn view_with_unreactive_state<F, S>(mut self, val: F) -> Self
    where
        F: Fn(S) -> View + Send + Sync + 'static,
        S: MakeRx + Serialize + DeserializeOwned + UnreactiveState + 'static,
        <S as MakeRx>::Rx: AnyFreeze + Clone + MakeUnrx<Unrx = S>,
    {
        self.view = Box::new(
            #[allow(unused_variables)]
            move |preload_info, template_state, path| {
                let reactor = Reactor::from_cx();
                // This will handle frozen/active state prioritization, etc.
                let intermediate_state = reactor.get_page_state::<S>(&path, template_state)?;
                let mut view = View::new();
                let disposer = create_child_scope(|| {
                    // We go back from the unreactive state type wrapper to the base type (since
                    // it's unreactive)
                    view = val(intermediate_state.make_unrx());
                });
                Ok((view, disposer))
            },
        );
        self
    }

    /// Sets the template rendering function to use for templates that take no
    /// state. Templates that do take state should use
    /// `.template_with_state()` instead.
    pub fn view<F>(mut self, val: F) -> Self
    where
        F: Fn() -> View + Send + Sync + 'static,
    {
        self.view = Box::new(move |_preload_info, _template_state, path| {
            let reactor = Reactor::from_cx();
            // Declare that this page/widget will never take any state to enable full
            // caching
            reactor.register_no_state(&path, false);

            // Nicely, if this is a widget, this means there need be no network requests
            // at all!
            let mut view = View::new();
            let disposer = create_child_scope(|| {
                view = val();
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
    #[cfg(engine)]
    pub fn head_with_state<S, V>(
        mut self,
        val: impl Fn(S) -> V + Send + Sync + 'static,
    ) -> Self
    where
        S: Serialize + DeserializeOwned + MakeRx + 'static,
        V: Into<GeneratorResult<View>>,
    {
        let template_name = self.get_path();
        self.head = Some(Box::new(move |template_state| {
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
                .into_server_result("head", template_name)
        }));
        self
    }
    /// Sets the document `<head>` rendering function to use. The [`View`]
    /// produced by this will only be rendered on the engine-side, and will
    /// *not* be reactive (since it only contains metadata).
    ///
    /// This is for heads that do require state. Those that do not should use
    /// `.head()` instead.
    #[cfg(any(client, doc))]
    pub fn head_with_state(self, _val: impl Fn() + 'static) -> Self {
        self
    }

    /// Sets the function to set headers. This will override Perseus' inbuilt
    /// header defaults. This should only be used when your header-setting
    /// requires knowing the state.
    #[cfg(engine)]
    pub fn set_headers_with_state<S, V>(
        mut self,
        val: impl Fn(S) -> V + Send + Sync + 'static,
    ) -> Self
    where
        S: Serialize + DeserializeOwned + MakeRx + 'static,
        V: Into<GeneratorResult<HeaderMap>>,
    {
        let template_name = self.get_path();
        self.set_headers = Some(Box::new(move |template_state| {
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
        }));
        self
    }
    /// Sets the function to set headers. This will override Perseus' inbuilt
    /// header defaults. This should only be used when your header-setting
    /// requires knowing the state.
    #[cfg(any(client, doc))]
    pub fn set_headers_with_state(self, _val: impl Fn() + 'static) -> Self {
        self
    }
}
