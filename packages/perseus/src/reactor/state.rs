use serde::{Serialize, de::DeserializeOwned};
use sycamore::web::Html;
use crate::{PathMaybeWithLocale, errors::{ClientError, ClientInvariantError, ClientThawError}, state::{AnyFreeze, Freeze, FrozenApp, FrozenGlobalState, GlobalStateType, MakeRx, MakeRxRef, MakeUnrx}, template::TemplateState};
use super::Reactor;

// Explicitly prevent the user from trying to freeze on the engine-side
#[cfg(target_arch = "wasm32")]
impl<G: Html> Freeze for Reactor<G> {
    fn freeze(&self) -> String {
        // This constructs a `FrozenApp`, which has everything the thawing reactor will need
        let frozen_app = FrozenApp {
            // `GlobalStateType` -> `FrozenGlobalState`
            global_state: self.global_state.0.borrow().into(),
            route: match &*self.router.get_load_state_rc().get_untracked() {
                RouterLoadState::Loaded { path, .. } => Some(path.clone()),
                // It would be impressive to manage this timing, but it's fine to go to the route we were
                // in the middle of loading when we thaw
                RouterLoadState::Loading { path, .. } => Some(path.clone()),
                // If we encounter this during re-hydration, we won't try to set the URL in the
                // browser
                RouterLoadState::ErrorLoaded { .. } => None,
                RouterLoadState::Server => None,
            },
            state_store: self.state_store.freeze_to_hash_map(),
        };
        serde_json::to_string(&frozen_app).unwrap()
    }
}

#[cfg(target_arch = "wasm32")]
impl<G: Html> Reactor<G> {
    /// Commands Perseus to 'thaw' the app from the given frozen state. You'll
    /// also need to provide preferences for thawing, which allow you to control
    /// how different pages should prioritize frozen state over existing (or
    /// *active*) state. Once you call this, assume that any following logic
    /// will not run, as this may navigate to a different route in your app. How
    /// you get the frozen state to supply to this is up to you.
    ///
    /// If the app has already been thawed from a previous frozen state, any
    /// state used from that will be considered *active* for this thawing.
    ///
    /// This will return an error if the frozen state provided is invalid.
    /// However, if the frozen state for an individual page is invalid, it will
    /// be silently ignored in favor of either the active state or the
    /// server-provided state.
    ///
    /// Note that any existing frozen app will be overriden by this.
    ///
    /// If the app was last frozen while on an error page, this will not attempt
    /// to change the current route.
    pub fn thaw(&self, new_frozen_app: &str, thaw_prefs: ThawPrefs) -> Result<(), ClientError> {
        let new_frozen_app: FrozenApp = serde_json::from_str(new_frozen_app)
            .map_err(|err| ClientError::ThawFailed { source: err })?;
        let route = new_frozen_app.route.clone();
        // Update our current frozen app
        let mut frozen_app = self.frozen_app.borrow_mut();
        *frozen_app = Some((new_frozen_app, thaw_prefs));
        // Better safe than sorry
        drop(frozen_app);

        // Check if we're on the same page now as we were at freeze-time
        let curr_route = match &*self.router.get_load_state_rc().get_untracked() {
                RouterLoadState::Loaded { path, .. } => path.clone(),
                RouterLoadState::Loading { path, .. } => path.clone(),
                // TODO
                RouterLoadState::ErrorLoaded { location } => todo!("thawing while in an error state is not yet implemented"),
                // Since this function is only defined on the browser-side, this should
                // be completely impossible (note that the user can't change the router
                // state manually)
                RouterLoadState::Server => unreachable!(),
            };
        // We handle the possibility that the page tried to reload before it had been
        // made interactive here (we'll just reload wherever we are)
        if let Some(route) = route {
            // If we're on the same page, just reload, otherwise go to the frozen route
            if curr_route == route {
                // We need to do this to get the new frozen state (dependent on thaw prefs)
                self.router.reload();
            } else {
                navigate(&route);
            }
        } else {
            // The page froze before hydration, so we'll just reload to get the new state
            self.router.reload();
        }

        Ok(())
    }
}

// These methods are used for acquiring the state of pages on both the browser-side and the engine-side
impl<G: Html> Reactor<G> {
    /// Gets the intermediate state type for the given page by evaluating active and frozen
    /// state to see if anything else is available, reverting to the provided state from
    /// the server if necessary.
    ///
    /// This will return an invariant error if the provided server state is invalid, since
    /// it's assumed to have actually come from the server.
    ///
    /// This should not be used for capsules!
    pub(crate) fn get_page_state<S>(
        &self,
        url: &PathMaybeWithLocale,
        server_state: TemplateState,
    ) -> Result<S::Rx, ClientError>
    where
        S: MakeRx,
        S::Rx: MakeUnrx<Unrx = S> + AnyFreeze + Clone + MakeRxRef,
    {
        if let Some(held_state) = self.get_held_state::<S>(url, false) {
            held_state
        } else {
            // Fall back to the state we were given, first
            // giving it a type (this just sets a phantom type parameter)
            let typed_state = server_state.change_type::<S>();
            // This attempts a deserialization from a `Value`, which could fail
            let unrx = typed_state
                .to_concrete()
                .map_err(|err| ClientInvariantError::InvalidState { source: err })?;
            let rx = unrx.make_rx();
            // Add that to the state store as the new active state
            self.state_store.add_state(url, rx, false)?;

            Ok(rx)
        }
    }
    // TODO Version of the above for widgets
    /// Registers a page/widget as definitely taking no state, which allows it to be
    /// cached fully, preventing unnecessary network requests. Any future
    /// attempt to set state will lead to errors (with logical exceptions for HSR).
    pub fn register_no_state(&self, url: &PathMaybeWithLocale, is_widget: bool) {
        self.page_state_store.set_state_never(url, is_widget);
    }

    /// Determines if the given path (page or capsule) should use the state given by the server,
    /// or whether it has other state in the frozen/active state systems. If the latter is true,
    /// this will instantiate them appropriately and return them. If this returns `None`, the
    /// server-provided state should be used.
    ///
    /// This needs to know if it's a widget or a page so the state can be appropriately registered
    /// in the state store if necessary.
    ///
    /// To understand the exact logic chain this uses, please refer to the flowchart of the
    /// Perseus reactive state platform in the book.
    ///
    /// Note: on the engine-side, there is no such thing as frozen state, and the active state will
    /// always be empty, so this will simply return `None`.
    #[cfg(target_arch = "wasm32")]
    fn get_held_state<S>(&self, url: &PathMaybeWithLocale, is_widget: bool) -> Result<Option<S::Rx>, ClientError>
    where
        S: MakeRx,
        S::Rx: MakeUnrx<Unrx = S> + AnyFreeze + Clone + MakeRxRef,
    {
        // See if we can get both the active and frozen states
        let frozen_app_full = self.frozen_app.borrow();
        if let Some((_, thaw_prefs)) = &*frozen_app_full {
            // Check against the thaw preferences if we should prefer frozen state over
            // active state
            if thaw_prefs.page.should_prefer_frozen_state(url) {
                drop(frozen_app_full);
                // We'll fall back to active state if no frozen state is available
                match self.get_frozen_state_and_register::<S>(url, is_widget)? {
                    Some(state) => Ok(Some(state)),
                    None => Ok(self.get_active_state::<S>(url)),
                }
            } else {
                drop(frozen_app_full);
                // We're preferring active state, but we'll fall back to frozen state if none is
                // available
                match self.get_active_state::<S>(url) {
                    Some(state) => Ok(Some(state)),
                    None => self.get_frozen_state_and_register::<S>(url, is_widget),
                }
            }
        } else {
            // No frozen app exists, so we of course shouldn't prioritize it
            Ok(self.get_active_state::<S>(url))
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn get_held_state<S>(&self, _url: &PathMaybeWithLocale, _is_widget: bool) -> Result<Option<S::Rx>, ClientError>
    where
        S: MakeRx,
        S::Rx: MakeUnrx<Unrx = S> + AnyFreeze + Clone + MakeRxRef,
    {
        None
    }

    /// Attempts to the get the active state for a page or widget. Of course, this does not
    /// register anything in the state store.
    fn get_active_state<S>(&self, url: &PathMaybeWithLocale) -> Option<S::Rx>
    where
        S: MakeRx,
        S::Rx: MakeUnrx<Unrx = S> + AnyFreeze + Clone + MakeRxRef,
    {
        self.state_store
            .get_state::<S::Rx>(url)
    }
    /// Attempts to extract the frozen state for the given page from any currently registered frozen
    /// app, registering what it finds. This assumes that the thaw preferences have already been
    /// accounted for.
    #[cfg(target_arch = "wasm32")]
    fn get_frozen_state_and_register<S>(&self, url: &PathMaybeWithLocale, is_widget: bool) -> Result<Option<S::Rx>, ClientError>
    where
        S: MakeRx + Serialize + DeserializeOwned,
        S::Rx: MakeUnrx<Unrx = S> + AnyFreeze + Clone + MakeRxRef,
    {
        let frozen_app_full = self.frozen_app.borrow();
        if let Some((frozen_app, thaw_prefs, is_hsr)) = &*frozen_app_full {
            #[cfg(not(all(debug_assertions, feature = "hsr")))]
            assert!(!is_hsr, "attempted to invoke hsr-style thaw in non-hsr environment");
            // Get the serialized and unreactive frozen state from the store
            match frozen_app.page_state_store.get(&url) {
                Some(state_str) => {
                    // Deserialize into the unreactive version
                    let unrx = match serde_json::from_str::<S>(state_str) {
                        Ok(unrx) => unrx,
                        // A corrupted frozen state should explicitly bubble up to be an error,
                        // *unless* this is HSR, in which case the data model has just been changed,
                        // and we should move on
                        Err(_) if is_hsr => return Ok(None),
                        Err(err) => return Err(ClientThawError::InvalidFrozenState { source: err }.into()),
                    };
                    // This returns the reactive version of the unreactive version of `R`, which
                    // is why we have to make everything else do the same
                    // Then we convince the compiler that that actually is `R` with the
                    // ludicrous trait bound at the beginning of this function
                    let rx = unrx.make_rx();
                    // Now add the reactive version to the state store (see the documentation
                    // for this method for HSR caveats, and why we ignore the error in HSR mode)
                    match self.state_store.add_state(url, rx.clone(), is_widget) {
                        Ok(_) => (),
                        // This means the user has removed state from an entity that previously had it,
                        // and that's fine
                        Err(_) if is_hsr => return Ok(None),
                        Err(err) => return Err(err)
                    };

                    // Now we should remove this from the frozen state so we don't fall back to
                    // it again
                    drop(frozen_app_full);
                    let mut frozen_app_val = self.frozen_app.take().unwrap(); // We're literally in a conditional that checked this
                    frozen_app_val.0.page_state_store.remove(url);
                    let mut frozen_app = self.frozen_app.borrow_mut();
                    *frozen_app = Some(frozen_app_val);

                    Ok(Some(rx))
                }
                None => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}
