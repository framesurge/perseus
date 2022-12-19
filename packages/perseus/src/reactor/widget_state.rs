#[cfg(target_arch = "wasm32")]
use std::sync::Arc;

use super::Reactor;
use crate::{
    errors::{ClientError, ClientInvariantError},
    path::*,
    state::{AnyFreeze, MakeRx, MakeUnrx, PssContains, TemplateState, UnreactiveState},
};
use serde::{de::DeserializeOwned, Serialize};
use sycamore::{
    prelude::{create_child_scope, create_ref, BoundedScope, Scope, ScopeDisposer},
    view::View,
    web::Html,
};

#[cfg(target_arch = "wasm32")]
use crate::template::PreloadInfo;
#[cfg(target_arch = "wasm32")]
use sycamore::prelude::create_signal;
#[cfg(target_arch = "wasm32")]
use sycamore_futures::spawn_local_scoped;

impl<G: Html> Reactor<G> {
    /// Gets the view and disposer for the given widget path. This will perform
    /// asynchronous fetching as needed to fetch state from the server, and
    /// will also handle engine-side state pass-through. This function will
    /// propagate as many errors as it can, though those occurring inside a
    /// `spawn_local_scoped` environment will be resolved to error views.
    ///
    /// This is intended for use with widgets that use reactive state. See
    /// `.get_unreactive_widget_view()` for widgets that use unreactive
    /// state.
    // HRTB explanation: 'a = 'app, but the compiler hates that.
    pub(crate) fn get_widget_view<'a, S, F, P: Clone + 'static>(
        &'a self,
        app_cx: Scope<'a>,
        path: PathMaybeWithLocale,
        #[cfg(target_arch = "wasm32")] capsule_name: String,
        template_state: TemplateState, // Empty on the browser-side
        props: P,
        #[cfg(target_arch = "wasm32")] preload_info: PreloadInfo,
        view_fn: F,
        #[cfg(target_arch = "wasm32")] fallback_fn: &Arc<dyn Fn(Scope, P) -> View<G> + Send + Sync>,
    ) -> Result<(View<G>, ScopeDisposer<'a>), ClientError>
    where
        // Note: these bounds replicate those for `.view_with_state()`, except the app lifetime is
        // known
        F: for<'app, 'child> Fn(BoundedScope<'app, 'child>, &'child S::Rx, P) -> View<G>
            + Send
            + Sync
            + 'static,
        S: MakeRx + Serialize + DeserializeOwned + 'static,
        S::Rx: MakeUnrx<Unrx = S> + AnyFreeze + Clone,
    {
        match self.get_widget_state_no_fetch::<S>(&path, template_state)? {
            Some(intermediate_state) => {
                let mut view = View::empty();
                let disposer = create_child_scope(app_cx, |child_cx| {
                    // We go back from the unreactive state type wrapper to the base type (since
                    // it's unreactive)
                    view = view_fn(child_cx, create_ref(child_cx, intermediate_state), props);
                });
                Ok((view, disposer))
            }
            // We need to asynchronously fetch the state from the server, which doesn't work
            // ergonomically with the rest of the code, so we just break out entirely
            #[cfg(target_arch = "wasm32")]
            None => {
                return {
                    let view = create_signal(app_cx, View::empty());

                    let fallback_fn = fallback_fn.clone();
                    let disposer = create_child_scope(app_cx, |child_cx| {
                        // We'll render the fallback view in the meantime (which `PerseusApp`
                        // guarantees to be defined for capsules)
                        view.set((fallback_fn)(child_cx, props.clone()));
                        // Note: this uses `child_cx`, meaning the fetch will be aborted if the user
                        // goes to another page (when this page is cleaned
                        // up, including all child scopes)
                        let capsule_name = capsule_name.clone();
                        spawn_local_scoped(child_cx, async move {
                            // Any errors that occur in here will be converted into proper error
                            // views using the reactor (it's not the
                            // nicest handling pattern, but in a future
                            // like this, it's the best we can do)
                            let final_view = {
                                let path_without_locale =
                                    PathWithoutLocale(match preload_info.locale.as_str() {
                                        "xx-XX" => path.to_string(),
                                        locale => path
                                            .strip_prefix(&format!("{}/", locale))
                                            .unwrap()
                                            .to_string(),
                                    });
                                // We can simply use the preload system to perform the fetching
                                match self
                                    .state_store
                                    .preload(
                                        &path_without_locale,
                                        &preload_info.locale,
                                        &capsule_name,
                                        preload_info.was_incremental_match,
                                        false, // Don't use the route preloading system
                                        true,  // This is a widget
                                    )
                                    .await
                                {
                                    // If that succeeded, we can use the same logic as before, and
                                    // we know it can't return `Ok(None)`
                                    // this time! We're in the browser, so we can just use an empty
                                    // template state, rather than
                                    // cloning the one we've been given (which is empty anyway).
                                    Ok(()) => match self.get_widget_state_no_fetch::<S>(
                                        &path,
                                        TemplateState::empty(),
                                    ) {
                                        Ok(Some(intermediate_state)) => view_fn(
                                            child_cx,
                                            create_ref(child_cx, intermediate_state),
                                            props,
                                        ),
                                        Ok(None) => unreachable!(),
                                        Err(err) => self.error_views.handle_widget(err, child_cx),
                                    },
                                    Err(err) => self.error_views.handle_widget(err, child_cx),
                                }
                            };

                            view.set(final_view);
                        });
                    });

                    Ok((sycamore::prelude::view! { app_cx, (*view.get()) }, disposer))
                };
            }
            // On the engine-side, this is impossible (we cannot be instructed to fetch)
            #[cfg(not(target_arch = "wasm32"))]
            None => unreachable!(),
        }
    }

    /// Gets the view and disposer for the given widget path. This will perform
    /// asynchronous fetching as needed to fetch state from the server, and
    /// will also handle engine-side state pass-through. This function will
    /// propagate as many errors as it can, though those occurring inside a
    /// `spawn_local_scoped` environment will be resolved to error views.
    ///
    /// This is intended for use with widgets that use unreactive state. See
    /// `.get_widget_view()` for widgets that use reactive state.
    pub(crate) fn get_unreactive_widget_view<'a, F, S, P: Clone + 'static>(
        &'a self,
        app_cx: Scope<'a>,
        path: PathMaybeWithLocale,
        #[cfg(target_arch = "wasm32")] capsule_name: String,
        template_state: TemplateState, // Empty on the browser-side
        props: P,
        #[cfg(target_arch = "wasm32")] preload_info: PreloadInfo,
        view_fn: F,
        #[cfg(target_arch = "wasm32")] fallback_fn: &Arc<dyn Fn(Scope, P) -> View<G> + Send + Sync>,
    ) -> Result<(View<G>, ScopeDisposer<'a>), ClientError>
    where
        F: Fn(Scope, S, P) -> View<G> + Send + Sync + 'static,
        S: MakeRx + Serialize + DeserializeOwned + UnreactiveState + 'static,
        <S as MakeRx>::Rx: AnyFreeze + Clone + MakeUnrx<Unrx = S>,
    {
        match self.get_widget_state_no_fetch::<S>(&path, template_state)? {
            Some(intermediate_state) => {
                let mut view = View::empty();
                let disposer = create_child_scope(app_cx, |child_cx| {
                    // We go back from the unreactive state type wrapper to the base type (since
                    // it's unreactive)
                    view = view_fn(child_cx, intermediate_state.make_unrx(), props);
                });
                Ok((view, disposer))
            }
            // We need to asynchronously fetch the state from the server, which doesn't work
            // ergonomically with the rest of the code, so we just break out entirely
            #[cfg(target_arch = "wasm32")]
            None => {
                return {
                    let view = create_signal(app_cx, View::empty());

                    let fallback_fn = fallback_fn.clone();
                    let disposer = create_child_scope(app_cx, |child_cx| {
                        // We'll render the fallback view in the meantime (which `PerseusApp`
                        // guarantees to be defined for capsules)
                        view.set((fallback_fn)(child_cx, props.clone()));
                        // Note: this uses `child_cx`, meaning the fetch will be aborted if the user
                        // goes to another page (when this page is cleaned
                        // up, including all child scopes)
                        let capsule_name = capsule_name.clone();
                        spawn_local_scoped(child_cx, async move {
                            // Any errors that occur in here will be converted into proper error
                            // views using the reactor (it's not the
                            // nicest handling pattern, but in a future
                            // like this, it's the best we can do)
                            let final_view = {
                                let path_without_locale =
                                    PathWithoutLocale(match preload_info.locale.as_str() {
                                        "xx-XX" => path.to_string(),
                                        locale => path
                                            .strip_prefix(&format!("{}/", locale))
                                            .unwrap()
                                            .to_string(),
                                    });
                                // We can simply use the preload system to perform the fetching
                                match self
                                    .state_store
                                    .preload(
                                        &path_without_locale,
                                        &preload_info.locale,
                                        &capsule_name,
                                        preload_info.was_incremental_match,
                                        false, // Don't use the route preloading system
                                        true,  // This is a widget
                                    )
                                    .await
                                {
                                    // If that succeeded, we can use the same logic as before, and
                                    // we know it can't return `Ok(None)`
                                    // this time! We're in the browser, so we can just use an empty
                                    // template state, rather than
                                    // cloning the one we've been given (which is empty anyway).
                                    Ok(()) => match self.get_widget_state_no_fetch::<S>(
                                        &path,
                                        TemplateState::empty(),
                                    ) {
                                        Ok(Some(intermediate_state)) => {
                                            view_fn(child_cx, intermediate_state.make_unrx(), props)
                                        }
                                        Ok(None) => unreachable!(),
                                        Err(err) => self.error_views.handle_widget(err, child_cx),
                                    },
                                    Err(err) => self.error_views.handle_widget(err, child_cx),
                                }
                            };

                            view.set(final_view);
                        });
                    });

                    Ok((sycamore::prelude::view! { app_cx, (*view.get()) }, disposer))
                };
            }
            // On the engine-side, this is impossible (we cannot be instructed to fetch)
            #[cfg(not(target_arch = "wasm32"))]
            None => unreachable!(),
        }
    }

    /// Gets the state for the given widget. This will return `Ok(None)`, if the
    /// state needs to be fetched from the server.
    ///
    /// This will check against the active and frozen states, but it will
    /// extract state from the preload system on an initial load (as this is
    /// how widget states are loaded in). Note that this also acts as a
    /// general interface with the preload system for widgets, the role
    /// of which is fulfilled for pages by the subsequent load system.
    ///
    /// On the engine-side, this will use the given template state (which will
    /// be passed through, unlike on the browser-side, where it will always
    /// be empty).
    pub(crate) fn get_widget_state_no_fetch<S>(
        &self,
        url: &PathMaybeWithLocale,
        server_state: TemplateState,
    ) -> Result<Option<S::Rx>, ClientError>
    where
        S: MakeRx + Serialize + DeserializeOwned + 'static,
        S::Rx: MakeUnrx<Unrx = S> + AnyFreeze + Clone,
    {
        if let Some(held_state) = self.get_held_state::<S>(url, true)? {
            Ok(Some(held_state))
        } else if cfg!(target_arch = "wasm32") {
            // On the browser-side, the given server state is empty, and we need to check
            // the preload
            match self.state_store.contains(url) {
                // This implies either user preloading, or initial load automatic preloading
                // from `__PERSEUS_INITIAL_WIDGET_STATES`
                PssContains::Preloaded => {
                    let page_data = self.state_store.get_preloaded(url).unwrap();
                    // Register an empty head
                    self.state_store.add_head(url, String::new(), true);
                    // And reactivize the state for registration
                    let typed_state = TemplateState::from_value(page_data.state).change_type::<S>();
                    // This attempts a deserialization from a `Value`, which could fail
                    let unrx = typed_state
                        .into_concrete()
                        .map_err(|err| ClientInvariantError::InvalidState { source: err })?;
                    let rx = unrx.make_rx();
                    // Add that to the state store as the new active state
                    self.state_store.add_state(url, rx.clone(), false)?;

                    Ok(Some(rx))
                }
                // We need to fetch the state from the server, which will require
                // asynchronicity, so bail out of this function, which is
                // not equipped for that
                PssContains::None => Ok(None),
                // Widgets have no heads, and must always be registered with a state
                PssContains::Head | PssContains::HeadNoState => {
                    Err(ClientInvariantError::InvalidWidgetPssEntry.into())
                }
                // These would have been caught by `get_held_state()` above
                PssContains::All | PssContains::State => unreachable!(),
            }
        }
        // On the engine-side, the given server state is correct, and `get_held_state()`
        // will definitionally return `Ok(None)`
        else if server_state.is_empty() {
            // This would be quite concerning...
            Err(ClientInvariantError::NoState.into())
        } else {
            // Fall back to the state we were given, first
            // giving it a type (this just sets a phantom type parameter)
            let typed_state = server_state.change_type::<S>();
            // This attempts a deserialization from a `Value`, which could fail
            let unrx = typed_state
                .into_concrete()
                .map_err(|err| ClientInvariantError::InvalidState { source: err })?;
            let rx = unrx.make_rx();
            // Add that to the state store as the new active state
            self.state_store.add_state(url, rx.clone(), false)?;

            Ok(Some(rx))
        }
    }
}
