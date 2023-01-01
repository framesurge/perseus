use super::Reactor;
use crate::{
    errors::*,
    path::*,
    state::{AnyFreeze, MakeRx, MakeUnrx, TemplateState},
};
#[cfg(client)]
use crate::{
    router::RouterLoadState,
    state::{Freeze, FrozenApp, ThawPrefs},
};
use serde::{de::DeserializeOwned, Serialize};
#[cfg(client)]
use sycamore::prelude::Scope;
use sycamore::web::Html;
#[cfg(client)]
use sycamore_router::navigate;

// Explicitly prevent the user from trying to freeze on the engine-side
#[cfg(client)]
impl<G: Html> Freeze for Reactor<G> {
    fn freeze(&self) -> String {
        // This constructs a `FrozenApp`, which has everything the thawing reactor will
        // need
        let frozen_app = FrozenApp {
            // `GlobalStateType` -> `FrozenGlobalState`
            global_state: (&*self.global_state.0.borrow()).into(),
            route: match &*self.router_state.get_load_state_rc().get_untracked() {
                RouterLoadState::Loaded { path, .. } => Some(path.clone()),
                // It would be impressive to manage this timing, but it's fine to go to the route we
                // were in the middle of loading when we thaw
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

#[cfg(client)]
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
        self._thaw(new_frozen_app, thaw_prefs, false)
    }
    /// Internal underlying thaw logic (generic over HSR).
    pub(super) fn _thaw(
        &self,
        new_frozen_app: &str,
        thaw_prefs: ThawPrefs,
        is_hsr: bool,
    ) -> Result<(), ClientError> {
        // This won't check the data model, just that it is valid in some Perseus app
        // that could exist (therefore fine with HSR)
        let new_frozen_app: FrozenApp = serde_json::from_str(new_frozen_app)
            .map_err(|err| ClientThawError::InvalidFrozenApp { source: err })?;
        let route = new_frozen_app.route.clone();
        // Update our current frozen app
        let mut frozen_app = self.frozen_app.borrow_mut();
        *frozen_app = Some((new_frozen_app, thaw_prefs, is_hsr));
        // Better safe than sorry
        drop(frozen_app);

        if let Some(frozen_route) = route {
            let curr_route = match &*self.router_state.get_load_state_rc().get_untracked() {
                // If we've loaded a page, or we're about to, only change the route if necessary
                RouterLoadState::Loaded { path, .. }
                | RouterLoadState::Loading { path, .. }
                | RouterLoadState::ErrorLoaded { path } => path.clone(),
                // Since this function is only defined on the browser-side, this should
                // be completely impossible (note that the user can't change the router
                // state manually)
                RouterLoadState::Server => unreachable!(),
            };
            // If we're on the same page, just reload, otherwise go to the frozen route
            if curr_route == frozen_route {
                // We need to do this to get the new frozen state (dependent on thaw prefs)
                self.router_state.reload();
            } else {
                navigate(&frozen_route);
            }
        } else {
            self.router_state.reload();
        }

        Ok(())
    }

    /// Preloads the given URL from the server and caches it, preventing
    /// future network requests to fetch that page. Localization will be
    /// handled automatically.
    ///
    /// This function automatically defers the asynchronous preloading
    /// work to a browser future for convenience. If you would like to
    /// access the underlying future, use `.try_preload()` instead.
    ///
    /// To preload a widget, you must prefix its path with `__capsule/`.
    ///
    /// # Panics
    /// This function will panic if any errors occur in preloading, such as
    /// the route being not found, or not localized. If the path you're
    /// preloading is not hardcoded, use `.try_preload()` instead.
    // Conveniently, we can use the lifetime mechanics of knowing that the render
    // context is registered on the given scope to ensure that the future works
    // out
    pub fn preload<'a, 'b: 'a>(&'b self, cx: Scope<'a>, url: &str) {
        use fmterr::fmt_err;
        let url = url.to_string();

        sycamore_futures::spawn_local_scoped(cx, async move {
            if let Err(err) = self.try_preload(&url).await {
                panic!("{}", fmt_err(&err));
            }
        });
    }
    /// Preloads the given URL from the server and caches it for the current
    /// route, preventing future network requests to fetch that page. On a
    /// route transition, this will be removed. Localization will be
    /// handled automatically.
    ///
    /// WARNING: the route preloading system is under heavy construction at
    /// present!
    ///
    /// This function automatically defers the asynchronous preloading
    /// work to a browser future for convenience. If you would like to
    /// access the underlying future, use `.try_route_preload()` instead.
    ///
    /// To preload a widget, you must prefix its path with `__capsule/`.
    ///
    /// # Panics
    /// This function will panic if any errors occur in preloading, such as
    /// the route being not found, or not localized. If the path you're
    /// preloading is not hardcoded, use `.try_route_preload()` instead.
    // Conveniently, we can use the lifetime mechanics of knowing that the render
    // context is registered on the given scope to ensure that the future works
    // out
    pub fn route_preload<'a, 'b: 'a>(&'b self, cx: Scope<'a>, url: &str) {
        use fmterr::fmt_err;
        let url = url.to_string();

        sycamore_futures::spawn_local_scoped(cx, async move {
            if let Err(err) = self.try_route_preload(&url).await {
                panic!("{}", fmt_err(&err));
            }
        });
    }
    /// A version of `.preload()` that returns a future that can resolve to an
    /// error. If the path you're preloading is not hardcoded, you should
    /// use this. Localization will be
    /// handled automatically.
    ///
    /// To preload a widget, you must prefix its path with `__capsule/`.
    pub async fn try_preload(&self, url: &str) -> Result<(), ClientError> {
        self._preload(url, false).await
    }
    /// A version of `.route_preload()` that returns a future that can resolve
    /// to an error. If the path you're preloading is not hardcoded, you
    /// should use this. Localization will be
    /// handled automatically.
    ///
    /// To preload a widget, you must prefix its path with `__capsule/`.
    pub async fn try_route_preload(&self, url: &str) -> Result<(), ClientError> {
        self._preload(url, true).await
    }
    /// Preloads the given URL from the server and caches it, preventing
    /// future network requests to fetch that page. Localization will be
    /// handled automatically.
    ///
    /// To preload a widget, you must prefix its path with `__capsule/`.
    async fn _preload(&self, path: &str, is_route_preload: bool) -> Result<(), ClientError> {
        use crate::router::{match_route, FullRouteVerdict};

        // It is reasonable to assume that this function will not be called before the
        // instantiation of a translator
        let locale = self.get_translator().get_locale();
        let full_path = PathMaybeWithLocale::new(&PathWithoutLocale(path.to_string()), &locale);

        let path_segments = full_path
            .split('/')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>(); // This parsing is identical to the Sycamore router's
                                     // Get a route verdict on this so we know where we're going (this doesn't modify
                                     // the router state)
        let verdict = match_route(
            &path_segments,
            &self.render_cfg,
            &self.entities,
            &self.locales,
        );
        // Make sure we've got a valid verdict (otherwise the user should be told there
        // was an error)
        let route_info = match verdict.into_full(&self.entities) {
            FullRouteVerdict::Found(info) => info,
            FullRouteVerdict::NotFound { .. } => {
                return Err(ClientPreloadError::PreloadNotFound {
                    path: path.to_string(),
                }
                .into())
            }
            FullRouteVerdict::LocaleDetection(_) => {
                return Err(ClientPreloadError::PreloadLocaleDetection {
                    path: path.to_string(),
                }
                .into())
            }
        };

        // We just needed to acquire the arguments to this function
        self.state_store
            .preload(
                // We want an unlocalized path, which will be amalgamated with the locale for the
                // key
                &route_info.path,
                &route_info.locale,
                &route_info.entity.get_path(),
                route_info.was_incremental_match,
                is_route_preload,
                // While we might be preloading a widget, this just controls asset types, and,
                // since this function is intended for end user abstractions, this should always
                // use `AssetType::Preload`
                false,
            )
            .await
    }
}

// These methods are used for acquiring the state of pages on both the
// browser-side and the engine-side
impl<G: Html> Reactor<G> {
    /// Gets the intermediate state type for the given page by evaluating active
    /// and frozen state to see if anything else is available, reverting to
    /// the provided state from the server if necessary.
    ///
    /// This will return an invariant error if the provided server state is
    /// invalid, since it's assumed to have actually come from the server.
    /// It is also expected that the given path does actually take state!
    ///
    /// This should not be used for capsules!
    pub(crate) fn get_page_state<S>(
        &self,
        url: &PathMaybeWithLocale,
        server_state: TemplateState,
    ) -> Result<S::Rx, ClientError>
    where
        S: MakeRx + Serialize + DeserializeOwned + 'static,
        S::Rx: MakeUnrx<Unrx = S> + AnyFreeze + Clone,
    {
        if let Some(held_state) = self.get_held_state::<S>(url, false)? {
            Ok(held_state)
        } else if server_state.is_empty() {
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

            Ok(rx)
        }
    }
    /// Registers a page/widget as definitely taking no state, which allows it
    /// to be cached fully, preventing unnecessary network requests. Any
    /// future attempt to set state will lead to errors (with exceptions for
    /// HSR).
    pub fn register_no_state(&self, url: &PathMaybeWithLocale, is_widget: bool) {
        self.state_store.set_state_never(url, is_widget);
    }

    /// Determines if the given path (page or capsule) should use the state
    /// given by the server, or whether it has other state in the
    /// frozen/active state systems. If the latter is true,
    /// this will instantiate them appropriately and return them. If this
    /// returns `None`, the server-provided state should be used.
    ///
    /// This needs to know if it's a widget or a page so the state can be
    /// appropriately registered in the state store if necessary.
    ///
    /// To understand the exact logic chain this uses, please refer to the
    /// flowchart of the Perseus reactive state platform in the book.
    ///
    /// Note: on the engine-side, there is no such thing as frozen state, and
    /// the active state will always be empty, so this will simply return
    /// `None`.
    #[cfg(client)]
    pub(super) fn get_held_state<S>(
        &self,
        url: &PathMaybeWithLocale,
        is_widget: bool,
    ) -> Result<Option<S::Rx>, ClientError>
    where
        S: MakeRx + Serialize + DeserializeOwned,
        S::Rx: MakeUnrx<Unrx = S> + AnyFreeze + Clone,
    {
        // See if we can get both the active and frozen states
        let frozen_app_full = self.frozen_app.borrow();
        if let Some((_, thaw_prefs, _)) = &*frozen_app_full {
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
    #[cfg(engine)]
    pub(super) fn get_held_state<S>(
        &self,
        _url: &PathMaybeWithLocale,
        _is_widget: bool,
    ) -> Result<Option<S::Rx>, ClientError>
    where
        S: MakeRx,
        S::Rx: MakeUnrx<Unrx = S> + AnyFreeze + Clone,
    {
        Ok(None)
    }

    /// Attempts to the get the active state for a page or widget. Of course,
    /// this does not register anything in the state store.
    #[cfg(client)]
    fn get_active_state<S>(&self, url: &PathMaybeWithLocale) -> Option<S::Rx>
    where
        S: MakeRx,
        S::Rx: MakeUnrx<Unrx = S> + AnyFreeze + Clone,
    {
        self.state_store.get_state::<S::Rx>(url)
    }
    /// Attempts to extract the frozen state for the given page from any
    /// currently registered frozen app, registering what it finds. This
    /// assumes that the thaw preferences have already been accounted for.
    #[cfg(client)]
    fn get_frozen_state_and_register<S>(
        &self,
        url: &PathMaybeWithLocale,
        is_widget: bool,
    ) -> Result<Option<S::Rx>, ClientError>
    where
        S: MakeRx + Serialize + DeserializeOwned,
        S::Rx: MakeUnrx<Unrx = S> + AnyFreeze + Clone,
    {
        let frozen_app_full = self.frozen_app.borrow();
        if let Some((frozen_app, _, is_hsr)) = &*frozen_app_full {
            #[cfg(not(all(debug_assertions, feature = "hsr")))]
            assert!(
                !is_hsr,
                "attempted to invoke hsr-style thaw in non-hsr environment"
            );
            // Get the serialized and unreactive frozen state from the store
            match frozen_app.state_store.get(&url) {
                Some(state_str) => {
                    // Deserialize into the unreactive version
                    let unrx = match serde_json::from_str::<S>(state_str) {
                        Ok(unrx) => unrx,
                        // A corrupted frozen state should explicitly bubble up to be an error,
                        // *unless* this is HSR, in which case the data model has just been changed,
                        // and we should move on
                        Err(_) if *is_hsr => return Ok(None),
                        Err(err) => {
                            return Err(ClientThawError::InvalidFrozenState { source: err }.into())
                        }
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
                        // This means the user has removed state from an entity that previously had
                        // it, and that's fine
                        Err(_) if *is_hsr => return Ok(None),
                        Err(err) => return Err(err),
                    };

                    // Now we should remove this from the frozen state so we don't fall back to
                    // it again
                    drop(frozen_app_full);
                    let mut frozen_app_val = self.frozen_app.take().unwrap(); // We're literally in a conditional that checked this
                    frozen_app_val.0.state_store.remove(url);
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
