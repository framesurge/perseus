use fluent_bundle::resolver::Scope;
use serde::{Serialize, de::DeserializeOwned};
use sycamore::web::Html;
use crate::{PathMaybeWithLocale, errors::{ClientError, ClientInvariantError, ClientThawError}, state::{AnyFreeze, Freeze, FrozenApp, FrozenGlobalState, GlobalStateType, MakeRx, MakeRxRef, MakeUnrx, RxRef}, template::TemplateState};
use super::Reactor;

// These methods are used for acquiring the global state on both the browser-side and the engine-side
impl<G: Html> Reactor<G> {
    // TODO Fix these type bounds
    /// Gets the global state.
    ///
    /// # Panics
    /// This will panic if the app has no global state. If you don't know
    /// whether or not there is global state, use `.try_global_state()`
    /// instead.
    // This function takes the final ref struct as a type parameter! That
    // complicates everything substantially.
    pub fn get_global_state<'a, R>(
        &self,
        cx: Scope<'a>,
    ) -> R
    where
        R: RxRef,
        R::RxNonRef: MakeUnrx + AnyFreeze + Clone + MakeRxRef<RxRef = R>,
        <<R as RxRef>::RxNonRef as MakeUnrx>::Unrx: MakeRx<Rx = R::RxNonRef>
    {
        // Warn the user about the perils of having no build-time global state handler
        self.try_get_global_state::<R>(cx).unwrap().expect("you requested global state, but none exists for this app (if you;re generating it at request-time, then you can't access it at build-time; try adding a build-time generator too, or target-gating your use of global state for the browser-side only)")
    }
    /// The underlying logic for `.get_global_state()`, except this will return
    /// `None` if the app does not have global state.
    ///
    /// This will return an error if the state from the server was found to be
    /// invalid.
    pub fn try_get_global_state<'a, R>(
        &self,
        cx: Scope<'a>,
    ) -> Result<Option<R>, ClientError>
    where
        R: RxRef,
        R::RxNonRef: MakeUnrx + AnyFreeze + Clone + MakeRxRef<RxRef = R>,
        <<R as RxRef>::RxNonRef as MakeUnrx>::Unrx: MakeRx<Rx = R::RxNonRef>,
    {
        let global_state_ty = self.global_state.0.borrow();
        // Bail early if the app doesn't support global state
        if let GlobalStateType::None = *global_state_ty {
            return Ok(None);
        }

        let intermediate_state = if let Some(held_state) = self.get_held_global_state::<<<R as RxRef>::RxNonRef as MakeUnrx>::Unrx>() {
            held_state
        } else {
            // We'll get the server-given global state
            if let GlobalStateType::Server(server_state) = &*global_state_ty {
                // Fall back to the state we were given, first
                // giving it a type (this just sets a phantom type parameter)
                let typed_state = server_state.change_type::<<<R as RxRef>::RxNonRef as MakeUnrx>::Unrx>();
                // This attempts a deserialization from a `Value`, which could fail
                let unrx = typed_state
                    .to_concrete()
                    .map_err(|err| ClientInvariantError::InvalidState { source: err })?;
                let rx = unrx.make_rx();
                // Set that as the new active global state
                let mut active_global_state = self.global_state.0.borrow_mut();
                *active_global_state = GlobalStateType::Loaded(Box::new(rx.clone()));

                rx
            } else {
                // There are two alternatives: `None` (handled with an early bail above) and `Loaded`,
                // the latter of which would have been handled as the active state above (even if we
                // prioritized frozen state, that would have returned something; if there was an active global state,
                // we would've dealt with it). If we're here it was `Server`.
                unreachable!()
            }
        };

        Ok(intermediate_state.to_ref_struct(cx))
    }

    /// Determines if the global state should use the state given by the server,
    /// or whether it has other state in the frozen/active state systems. If the latter is true,
    /// this will instantiate them appropriately and return them. If this returns `None`, the
    /// server-provided state should be used.
    ///
    /// To understand the exact logic chain this uses, please refer to the flowchart of the
    /// Perseus reactive state platform in the book.
    ///
    /// Note: on the engine-side, there is no such thing as frozen state, and the active state will
    /// always be empty, so this will simply return `None`.
    #[cfg(target_arch = "wasm32")]
    fn get_held_global_state<S>(&self) -> Result<Option<S::Rx>, ClientError>
    where
        S: MakeRx,
        S::Rx: MakeUnrx<Unrx = S> + AnyFreeze + Clone + MakeRxRef,
    {
        // See if we can get both the active and frozen states
        let frozen_app_full = self.frozen_app.borrow();
        if let Some((_, thaw_prefs)) = &*frozen_app_full {
            // Check against the thaw preferences if we should prefer frozen state over
            // active state
            if thaw_prefs.global_prefer_frozen {
                drop(frozen_app_full);
                // We'll fall back to active state if no frozen state is available
                match self.get_frozen_global_state_and_register::<S>()? {
                    Some(state) => Ok(Some(state)),
                    None => Ok(self.get_active_global_state::<S>(url)),
                }
            } else {
                drop(frozen_app_full);
                // We're preferring active state, but we'll fall back to frozen state if none is
                // available
                match self.get_active_global_state::<S>(url) {
                    Some(state) => Ok(Some(state)),
                    None => self.get_frozen_global_state_and_register::<S>(),
                }
            }
        } else {
            // No frozen app exists, so we of course shouldn't prioritize it
            Ok(self.get_active_global_state::<S>())
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn get_held_global_state<S>(&self) -> Result<Option<S::Rx>, ClientError>
    where
        S: MakeRx,
        S::Rx: MakeUnrx<Unrx = S> + AnyFreeze + Clone + MakeRxRef,
    {
        None
    }

    /// Attempts to the get the active global state. Of course, this does not
    /// register anything in the state store. This may return an error on a downcast failure
    /// (which is probably the user's fault for providing the wrong type argument, but it's
    /// still an invariant failure).
    fn get_active_global_state<S>(&self, url: &PathMaybeWithLocale) -> Result<Option<S::Rx>, ClientError>
    where
        S: MakeRx,
        S::Rx: MakeUnrx<Unrx = S> + AnyFreeze + Clone + MakeRxRef,
    {
        self.global_state.0.borrow().parse_active::<S::Rx>()
    }
    /// Attempts to extract the frozen global state from any currently registered frozen
    /// app, registering what it finds. This assumes that the thaw preferences have already been
    /// accounted for.
    ///
    /// This assumes that the app actually supports global state.
    #[cfg(target_arch = "wasm32")]
    fn get_frozen_global_state_and_register<S>(&self) -> Result<Option<S::Rx>, ClientError>
    where
        S: MakeRx + Serialize + DeserializeOwned,
        S::Rx: MakeUnrx<Unrx = S> + AnyFreeze + Clone + MakeRxRef,
    {
        let frozen_app_full = self.frozen_app.borrow();
        if let Some((frozen_app, thaw_prefs, is_hsr)) = &*frozen_app_full {
            #[cfg(not(all(debug_assertions, feature = "hsr")))]
            assert!(!is_hsr, "attempted to invoke hsr-style thaw in non-hsr environment");

            match frozen_app.global_state {
                FrozenGlobalState::Some(state_str) => {
                    // Deserialize into the unreactive version
                    let unrx = match serde_json::from_str::<S>(state_str) {
                        Ok(unrx) => unrx,
                        // A corrupted frozen state should explicitly bubble up to be an error,
                        // *unless* this is HSR, in which case the data model has just been changed,
                        // and we should move on
                        Err(_) if is_hsr => return Ok(None),
                        Err(err) => return Err(ClientThawError::InvalidFrozenGlobalState { source: err }.into())
                    };
                    // This returns the reactive version of the unreactive version of `R`, which
                    // is why we have to make everything else do the same
                    // Then we convince the compiler that that actually is `R` with the
                    // ludicrous trait bound at the beginning of this function
                    let rx = unrx.make_rx();
                    // And we'll register this as the new active global state
                    let mut active_global_state = self.global_state.0.borrow_mut();
                    *active_global_state = GlobalStateType::Loaded(Box::new(rx.clone()));
                    // Now we should remove this from the frozen state so we don't fall back to
                    // it again
                    drop(frozen_app_full);
                    let mut frozen_app_val = self.frozen_app.take().unwrap(); // We're literally in a conditional that checked this
                    frozen_app_val.0.global_state = FrozenGlobalState::Used;
                    let mut frozen_app = self.frozen_app.borrow_mut();
                    *frozen_app = Some(frozen_app_val);

                    Some(rx)
                },
                // The state hadn't been modified from what the server provided, so
                // we'll just use that (note: this really means it hadn't been instantiated
                // yet).
                // We'll handle global state that has already been used in the same way (this
                // is needed because, unlike a page/widget state map, we can't just remove
                // the global state from the frozen app, so this acts as a placeholder).
                FrozenGlobalState::Server | FrozenGlobalState::Used => None,
                // There was no global state last time, but if we're here, we've
                // checked that the app is using global state. If we're using HSR,
                // allow the data model change, otherwise ths frozen state will be considered
                // invalid.
                FrozenGlobalState::None => if is_hsr {
                    return Ok(None)
                } else {
                    return Err(ClientThawError::NoFrozenGlobalState.into())
                },
            }
        } else {
            Ok(None)
        }
    }


}
