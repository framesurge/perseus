use super::super::fn_types::*;
use super::Template;
use crate::errors::ClientError;
use crate::state::{StateGeneratorInfo, TemplateState, UnknownStateType};
use crate::utils::PerseusDuration;
use crate::{
    reactor::Reactor,
    state::{
        AnyFreeze, MakeRx, MakeRxRef, MakeUnrx, PssContains, TemplateStateWithType, UnreactiveState,
    },
};
#[cfg(not(target_arch = "wasm32"))]
use http::HeaderMap;
use serde::{de::DeserializeOwned, Serialize};
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
    pub fn template_with_state<F, S, I>(mut self, val: F) -> Template<G>
    where
        F: Fn(Scope, I) -> View<G> + Send + Sync + 'static,
        S: MakeRx<Rx = I> + Serialize + DeserializeOwned + 'static,
        I: MakeUnrx<Unrx = S> + AnyFreeze + Clone + MakeRxRef,
        // IDEA: We might be able to fix these type bounds by having `R` link *directly* to `S`!
        // R: RxRef<RxNonRef = <S as MakeRx>::Rx>
    {
        self.template = Box::new(move |app_cx, preload_info, template_state, path| {
            let reactor = Reactor::<G>::from_cx(app_cx);
            let intermediate_state = if self.is_capsule {
                todo!()
            } else {
                // This will handle frozen/active state prioritization, etc.
                reactor.get_page_state::<S>(&path, template_state)?
            };

            //         // If this is a capsule though, the state we've been given is a
            // dummy, and         // we'll need to manually request it,
            // rendering a `Suspense` in the meantime.         None if
            // self.is_capsule => {             let pss =
            // &render_ctx.page_state_store;             // If this is an
            // initial load, the state will have been preloaded for us already. If it's
            //             // subsequent or if we're a delayed widget, it won't have been.
            //             match pss.contains(&path) {
            //                 // This indicates either that the widget was used by a
            // previous page,                 // or that this is an initial load
            //                 PssContains::Preloaded => {
            //                     let page_data = pss.get_preloaded(&path).unwrap();
            //                     // Register the head, otherwise it will never be
            // registered and the page will                     // never
            // properly show up in the PSS (meaning future preload
            // // calls will go through, creating unnecessary network requests). Note that
            //                     // this is guaranteed to be empty for a widget.
            //                     assert!(page_data.head.is_empty(), "widget had defined
            // head");                     pss.add_head(&path,
            // page_data.head.to_string(), self.is_capsule);
            // let typed_state = TemplateStateWithType::<S>::from_value(page_data.state);
            //                     // Register the state properly
            //                     match render_ctx
            //                         .register_page_state_value::<<S as
            // MakeRx>::Rx>(&path, typed_state, self.is_capsule)
            // {                         Ok(state) => state,
            //                         Err(err) => panic!(
            //                             "unrecoverable error in widget state derivation:
            // {:#?}",                             err
            //                         ),
            //                     }
            //                 },
            //                 PssContains::None => {
            //                     use sycamore::suspense::Suspense;

            //                     // We need to manually fetch the state, which involves
            // wrapping the user's                     // function in a Sycamore
            // `Suspense`. To do that, we'll do everything manually,
            // // and return directly.                     let disposer =
            // ::sycamore::reactive::create_child_scope(app_cx, |child_cx| {
            //                         let suspended_view = {
            //                             // Use the preload info that's been passed
            // through to preload the page (when                             //
            // this future is done, if it was successful, the widget will have been
            // preloaded)                             let path_without_locale =
            // match preload_info.locale.as_str() {
            // "xx-XX" => path.to_string(),
            // locale => path.strip_prefix(&format!("{}/", locale)).unwrap().to_string()
            //                             };
            //                             // pss.preload(
            //                             //     &path_without_locale,
            //                             //     &preload_info.locale,
            //                             //     &self.get_path(),
            //                             //     preload_info.was_incremental_match,
            //                             //     false, // This is not a route-specific
            // preload (it will be cleared in a moment anyway)
            // // );

            //                             // The preload has completed, so the state is in
            // the PSS                             let page_data =
            // pss.get_preloaded(&path).unwrap();                             //
            // Register the head, otherwise it will never be registered and the page will
            //                             // never properly show up in the PSS (meaning
            // future preload                             // calls will go
            // through, creating unnecessary network requests). Note that
            //                             // this is guaranteed to be empty for a widget.
            //                             assert!(page_data.head.is_empty(), "widget had
            // defined head");                             pss.add_head(&path,
            // page_data.head.to_string(), self.is_capsule);
            // let typed_state = TemplateStateWithType::<S>::from_value(page_data.state);
            //                             // Register the state properly
            //                             let intermediate_state = match render_ctx
            //                                 .register_page_state_value::<<S as
            // MakeRx>::Rx>(&path, typed_state, self.is_capsule)
            // {                                 Ok(state) => state,
            //                                 Err(err) => panic!(
            //                                     "unrecoverable error in widget state
            // derivation: {:#?}",                                     err
            //                                 ),
            //                             };

            //                             // Compute suspended states
            //                             #[cfg(target_arch = "wasm32")]
            //                             intermediate_state.compute_suspense(child_cx);

            //                             val(child_cx, intermediate_state)
            //                         };

            //                         // let view = sycamore::prelude::view! { child_cx,
            //                         //     Suspense(
            //                         //         fallback = todo!("capsule fallback view")
            //                         //     ) {
            //                         //         (suspended_view)
            //                         //     }
            //                         // };

            //                         // route_manager.update_view(view);
            //                     });
            //                     route_manager.update_disposer(disposer);

            //                     // We've done everything manually, so return to prevent
            // the default                     return;
            //                 }
            //                 // Widgets have no head, so only have a head is impossible.
            // They're also always registered                 // with empty
            // heads, so having just a state is impossible. Finally, there can't be
            // everything,                 // or
            // `.get_active_or_frozen_page_state()` would have returned active state.
            //                 PssContains::Head | PssContains::HeadNoState |
            // PssContains::State | PssContains::All => unreachable!(),
            //             }
            //         },
            //         _ => unreachable!()
            //     }
            // };

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
                view = val(child_cx, intermediate_state);
            });
            Ok((view, disposer))
        });
        self
    }
    /// Sets the template rendering function to use, if the template takes
    /// unreactive state.
    pub fn template_with_unreactive_state<F, S>(mut self, val: F) -> Template<G>
    where
        F: Fn(Scope, S) -> View<G> + Send + Sync + 'static,
        S: MakeRx + Serialize + DeserializeOwned + UnreactiveState + 'static,
        <S as MakeRx>::Rx: AnyFreeze + Clone + MakeUnrx<Unrx = S>,
    {
        self.template = Box::new(move |app_cx, preload_info, template_state, path| {
            let reactor = Reactor::<G>::from_cx(app_cx);
            let intermediate_state = if self.is_capsule {
                todo!()
            } else {
                // This will handle frozen/active state prioritization, etc.
                reactor.get_page_state::<S>(&path, template_state)?
            };

            let mut view = View::empty();
            let disposer = ::sycamore::reactive::create_child_scope(app_cx, |child_cx| {
                // We go back from the unreactive state type wrapper to the base type (since
                // it's unreactive)
                view = val(child_cx, intermediate_state.make_unrx());
            });
            Ok((view, disposer))
        });
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
            // Declare that this page will never take any state to enable full caching
            reactor.register_no_state(&path, self.is_capsule);

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
    pub fn head_with_state<S>(
        mut self,
        val: impl Fn(Scope, S) -> View<SsrNode> + Send + Sync + 'static,
    ) -> Template<G>
    where
        S: Serialize + DeserializeOwned + MakeRx + 'static,
    {
        use crate::errors::ClientInvariantError;

        let template_name = self.get_path();
        self.head = Box::new(move |cx, template_state| {
            // Make sure now that there is actually state
            if template_state.is_empty() {
                return Err(ClientInvariantError::NoState.into());
            }
            // Declare a type on the untyped state (this doesn't perform any conversions,
            // but the type we declare may be invalid)
            let typed_state = template_state.change_type::<S>();

            let state = match typed_state.to_concrete() {
                Ok(state) => state,
                Err(err) => return Err(ClientInvariantError::InvalidState { source: err }.into()),
            };
            Ok(val(cx, state))
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
    pub fn set_headers_with_state<S>(
        mut self,
        val: impl Fn(S) -> HeaderMap + Send + Sync + 'static,
    ) -> Template<G>
    where
        S: Serialize + DeserializeOwned + MakeRx + 'static,
    {
        use crate::errors::ClientInvariantError;

        let template_name = self.get_path();
        self.set_headers = Box::new(move |template_state| {
            // Make sure now that there is actually state
            if template_state.is_empty() {
                return Err(ClientInvariantError::NoState.into());
            }
            // Declare a type on the untyped state (this doesn't perform any conversions,
            // but the type we declare may be invalid)
            let typed_state = template_state.change_type::<S>();

            let state = match typed_state.to_concrete() {
                Ok(state) => state,
                Err(err) => return Err(ClientInvariantError::InvalidState { source: err }.into()),
            };
            Ok(val(state))
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
