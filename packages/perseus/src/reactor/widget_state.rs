#[cfg(any(client, doc))]
use std::cell::RefCell;
#[cfg(any(client, doc))]
use std::rc::Rc;
#[cfg(any(client, doc))]
use std::sync::Arc;

use super::Reactor;
use crate::{
    error_views::ServerErrorData,
    errors::{ClientError, ClientInvariantError},
    path::*,
    state::{AnyFreeze, MakeRx, MakeUnrx, PssContains, TemplateState, UnreactiveState},
};
use serde::{de::DeserializeOwned, Serialize};
use sycamore::{
    reactive::{create_child_scope, NodeHandle},
    web::View,
};

#[cfg(any(client, doc))]
use crate::template::PreloadInfo;
#[cfg(any(client, doc))]
use sycamore::prelude::create_signal;
#[cfg(any(client, doc))]
use sycamore_futures::spawn_local_scoped;

impl Reactor {
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
    #[allow(clippy::type_complexity)]
    #[allow(clippy::too_many_arguments)] // Internal function
    pub(crate) fn get_widget_view<'a, S, F, P: Clone + 'static>(
        &'a self,
        path: PathMaybeWithLocale,
        #[allow(unused_variables)] caller_path: PathMaybeWithLocale,
        #[cfg(any(client, doc))] capsule_name: String,
        template_state: TemplateState, // Empty on the browser-side
        props: P,
        #[cfg(any(client, doc))] preload_info: PreloadInfo,
        view_fn: F,
        #[cfg(any(client, doc))] fallback_fn: &Arc<dyn Fn(P) -> View + Send + Sync>,
    ) -> Result<(View, NodeHandle), ClientError>
    where
        // Note: these bounds replicate those for `.view_with_state()`, except the app lifetime is
        // known
        // In Sycamore 0.9.2, reactive state is usually Copy, but may not be for nested/suspense
        F: Fn(S::Rx, P) -> View + Send + Sync + 'static,
        S: MakeRx + Serialize + DeserializeOwned + 'static,
        S::Rx: MakeUnrx<Unrx = S> + AnyFreeze + Clone,
    {
        match self.get_widget_state_no_fetch::<S>(&path, template_state)? {
            Some(intermediate_state) => {
                let mut view = View::new();
                let disposer = create_child_scope(|| {
                    // In Sycamore 0.9.2, we pass state by value (or clone if not Copy)
                    view = view_fn(intermediate_state.clone(), props);
                });
                Ok((view, disposer))
            }
            // We need to asynchronously fetch the state from the server, which doesn't work
            // ergonomically with the rest of the code, so we just break out entirely
            #[cfg(any(client, doc))]
            None => {
                return {
                    // Use version counter pattern to avoid Rc::try_unwrap issues
                    let view_holder = Rc::new(RefCell::new(Option::<View>::None));
                    let view_version = create_signal(0u64);

                    let fallback_fn = fallback_fn.clone();
                    let view_holder_inner = view_holder.clone();
                    let disposer = create_child_scope(|| {
                        // We'll render the fallback view in the meantime (which `PerseusApp`
                        // guarantees to be defined for capsules)
                        *view_holder_inner.borrow_mut() = Some((fallback_fn)(props.clone()));
                        view_version.set(view_version.get_untracked() + 1);
                        // Note: this uses child scope, meaning the fetch will be aborted if the user
                        // goes to another page (when this page is cleaned
                        // up, including all child scopes)
                        let capsule_name = capsule_name.clone();
                        let view_holder_async = view_holder_inner.clone();
                        spawn_local_scoped(async move {
                            // Get reactor from context for async block
                            let reactor = Reactor::from_cx();
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
                                match reactor
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
                                    Ok(()) => match reactor.get_widget_state_no_fetch::<S>(
                                        &path,
                                        TemplateState::empty(),
                                    ) {
                                        Ok(Some(intermediate_state)) => {
                                            // Declare the relationship between the widget and its
                                            // caller
                                            reactor
                                                .state_store
                                                .declare_dependency(&path, &caller_path);

                                            // In Sycamore 0.9.2, pass state by value (or clone if not Copy)
                                            view_fn(intermediate_state.clone(), props)
                                        }
                                        Ok(None) => unreachable!(),
                                        Err(err) => reactor.error_views.handle_widget(err),
                                    },
                                    Err(err) => reactor.error_views.handle_widget(err),
                                }
                            };

                            *view_holder_async.borrow_mut() = Some(final_view);
                            view_version.set(view_version.get_untracked() + 1);
                        });
                    });

                    Ok((
                        sycamore::prelude::view! {
                            (move || {
                                // Track the version counter - triggers re-runs when view changes
                                view_version.track();
                                // Take the view from holder - doesn't trigger reactive updates
                                view_holder.borrow_mut().take().unwrap_or_else(View::new)
                            })
                        },
                        disposer,
                    ))
                };
            }
            // On the engine-side, this is impossible (we cannot be instructed to fetch)
            #[cfg(engine)]
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
    #[allow(clippy::type_complexity)]
    #[allow(clippy::too_many_arguments)] // Internal function
    pub(crate) fn get_unreactive_widget_view<'a, F, S, P: Clone + 'static>(
        &'a self,
        path: PathMaybeWithLocale,
        #[allow(unused_variables)] caller_path: PathMaybeWithLocale,
        #[cfg(any(client, doc))] capsule_name: String,
        template_state: TemplateState, // Empty on the browser-side
        props: P,
        #[cfg(any(client, doc))] preload_info: PreloadInfo,
        view_fn: F,
        #[cfg(any(client, doc))] fallback_fn: &Arc<dyn Fn(P) -> View + Send + Sync>,
    ) -> Result<(View, NodeHandle), ClientError>
    where
        F: Fn(S, P) -> View + Send + Sync + 'static,
        S: MakeRx + Serialize + DeserializeOwned + UnreactiveState + 'static,
        <S as MakeRx>::Rx: AnyFreeze + Clone + MakeUnrx<Unrx = S>,
    {
        match self.get_widget_state_no_fetch::<S>(&path, template_state)? {
            Some(intermediate_state) => {
                let mut view = View::new();
                let disposer = create_child_scope(|| {
                    // We go back from the unreactive state type wrapper to the base type (since
                    // it's unreactive)
                    view = view_fn(intermediate_state.make_unrx(), props);
                });
                Ok((view, disposer))
            }
            // We need to asynchronously fetch the state from the server, which doesn't work
            // ergonomically with the rest of the code, so we just break out entirely
            #[cfg(any(client, doc))]
            None => {
                return {
                    // Use version counter pattern to avoid Rc::try_unwrap issues
                    let view_holder = Rc::new(RefCell::new(Option::<View>::None));
                    let view_version = create_signal(0u64);

                    let fallback_fn = fallback_fn.clone();
                    let view_holder_inner = view_holder.clone();
                    let disposer = create_child_scope(|| {
                        // We'll render the fallback view in the meantime (which `PerseusApp`
                        // guarantees to be defined for capsules)
                        *view_holder_inner.borrow_mut() = Some((fallback_fn)(props.clone()));
                        view_version.set(view_version.get_untracked() + 1);
                        // Note: this uses child scope, meaning the fetch will be aborted if the user
                        // goes to another page (when this page is cleaned
                        // up, including all child scopes)
                        let capsule_name = capsule_name.clone();
                        let view_holder_async = view_holder_inner.clone();
                        spawn_local_scoped(async move {
                            // Get reactor from context for async block
                            let reactor = Reactor::from_cx();
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
                                match reactor
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
                                    Ok(()) => match reactor.get_widget_state_no_fetch::<S>(
                                        &path,
                                        TemplateState::empty(),
                                    ) {
                                        Ok(Some(intermediate_state)) => {
                                            // Declare the relationship between the widget and its
                                            // caller
                                            reactor
                                                .state_store
                                                .declare_dependency(&path, &caller_path);

                                            view_fn(intermediate_state.make_unrx(), props)
                                        }
                                        Ok(None) => unreachable!(),
                                        Err(err) => reactor.error_views.handle_widget(err),
                                    },
                                    Err(err) => reactor.error_views.handle_widget(err),
                                }
                            };

                            *view_holder_async.borrow_mut() = Some(final_view);
                            view_version.set(view_version.get_untracked() + 1);
                        });
                    });

                    Ok((
                        sycamore::prelude::view! {
                            (move || {
                                // Track the version counter - triggers re-runs when view changes
                                view_version.track();
                                // Take the view from holder - doesn't trigger reactive updates
                                view_holder.borrow_mut().take().unwrap_or_else(View::new)
                            })
                        },
                        disposer,
                    ))
                };
            }
            // On the engine-side, this is impossible (we cannot be instructed to fetch)
            #[cfg(engine)]
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
        } else if cfg!(client) {
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
                    let typed_state = TemplateState::from_value(page_data.state)
                        .change_type::<Result<S, ServerErrorData>>();
                    // This attempts a deserialization from a `Value`, which could fail
                    let unrx_res = typed_state
                        .into_concrete()
                        .map_err(|err| ClientInvariantError::InvalidState { source: err })?;
                    match unrx_res {
                        Ok(unrx) => {
                            let rx = unrx.make_rx();
                            // Add that to the state store as the new active state
                            self.state_store.add_state(url, rx.clone(), false)?;

                            Ok(Some(rx))
                        }
                        // This would occur if there were an error in the widget that were
                        // transmitted to us
                        Err(ServerErrorData { status, msg }) => Err(ClientError::ServerError {
                            status,
                            message: msg,
                        }),
                    }
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
