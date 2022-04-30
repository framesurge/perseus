use crate::errors::*;
use crate::router::{RouterLoadState, RouterState};
#[cfg(all(feature = "live-reload", debug_assertions))]
use crate::state::LiveReloadIndicator;
use crate::state::{
    AnyFreeze, Freeze, FrozenApp, FrozenAppStore, GlobalState, MakeRx, MakeUnrx, PageStateStore,
    ThawPrefs,
};
use crate::utils::provide_context_signal_replace;
use sycamore::prelude::{use_context, Scope, Signal};
use sycamore_router::navigate;

#[derive(Debug)]
struct IsFirstInner(bool);

/// A representation of a state that stores whether or not the current page is the very first page to have been rendered since the browser loaded the app. This will be reset naturally
/// on full reloads, and is used internally to determine whether or not we should look for any stored frozen state.
#[derive(Debug)]
pub struct IsFirst<'a>(&'a Signal<IsFirstInner>);
impl<'a> IsFirst<'a> {
    /// Creates a new instance of the first page state, registering with the context of the given reactive scope and mirroring it. This starts assuming that the inner value is `true`.
    pub fn new(cx: Scope<'a>) -> Self {
        let inner = provide_context_signal_replace(cx, IsFirstInner(true));

        Self(inner)
    }
    /// Creates a new instance of the first page state from the context of the given reactive scope. If the required types do not exist in the given scope, this will panic.
    pub fn from_ctx(cx: Scope<'a>) -> Self {
        Self(use_context(cx))
    }
    /// Gets the inner value.
    pub fn get(&self) -> bool {
        (*self.0.get()).0
    }
    /// Sets the inner value.
    pub fn set(&self, val: bool) {
        self.0.set(IsFirstInner(val))
    }
}

/// A representation of the render context of the app, constructed from references to a series of `struct`s that mirror context values. This is purely a proxy `struct` for function
/// organization.
#[derive(Debug)]
pub struct RenderCtx<'a> {
    // /// A translator for templates to use. This will still be present in non-i18n apps, but it will have no message IDs and support for
    // /// the non-existent locale `xx-XX`. This uses an `Arc<T>` for thread-safety.
    // translator: Translator,
    /// The router's state.
    pub router: RouterState<'a>,
    /// The page state store for the app. This is a type map to which pages can add state that they need to access later. Usually, this will be interfaced with through
    /// the `#[perseus::template_with_rx_state(...)]` macro, but it can be used manually as well to get the state of one page from another (provided that the target page has already
    /// been visited).
    pub page_state_store: PageStateStore<'a>,
    /// The user-provided global state. This is stored on the heap to avoid a type parameter that would be needed every time we had to access the render context (which would be very difficult
    /// to pass around inside Perseus).
    ///
    /// Because we store `dyn Any` in here, we initialize it as `Option::None`, and then the template macro (which does the heavy lifting for global state) will find that it can't downcast
    /// to the user's global state type, which will prompt it to deserialize whatever global state it was given and then write that here.
    pub global_state: GlobalState<'a>,
    /// A previous state the app was once in, still serialized. This will be rehydrated gradually by the template macro.
    pub frozen_app: FrozenAppStore<'a>,
    /// Whether or not this page is the very first to have been rendered since the browser loaded the app. This will be reset on full reloads, and is used internally to determine whether or
    /// not we should look for stored HSR state.
    pub is_first: IsFirst<'a>,
    #[cfg(all(feature = "live-reload", debug_assertions))]
    /// An indicator `Signal` used to allow the root to instruct the app that we're about to reload because of an instruction from the live reloading server. Hooking into this to run code
    /// before live reloading takes place is NOT supported, as no guarantee can be made that your code will run before Perseus reloads the page fully (at which point no more code will run).
    pub live_reload_indicator: LiveReloadIndicator,
}
impl<'a> RenderCtx<'a> {
    /// Creates a new instance of the render context from the given scope, creating all necessary mirror `struct`s. This is cheap because it entirely mirrors the context with references.
    /// If any of the necessary data aren't present in the context, this will lead to a panic.
    pub fn from_ctx(cx: Scope<'a>) -> Self {
        Self {
            router: RouterState::from_ctx(cx),
            page_state_store: PageStateStore::from_ctx(cx),
            global_state: GlobalState::from_ctx(cx),
            frozen_app: FrozenAppStore::from_ctx(cx),
            is_first: IsFirst::from_ctx(cx),
            #[cfg(all(feature = "live-reload", debug_assertions))]
            live_reload_indicator: LiveReloadIndicator::from_ctx(cx),
        }
    }
    /// Creates a new instance of the render context for the server-side. Do NOT execute this on the client-side, as it will override all currently-set context values, leading to state loss!
    pub fn server(cx: Scope<'a>) -> Self {
        Self {
            router: RouterState::new(cx),
            page_state_store: PageStateStore::new(cx),
            global_state: GlobalState::new(cx),
            frozen_app: FrozenAppStore::new(cx),
            is_first: IsFirst::new(cx),
            #[cfg(all(feature = "live-reload", debug_assertions))]
            live_reload_indicator: LiveReloadIndicator::new(cx),
        }
    }
}
impl<'a> Freeze for RenderCtx<'a> {
    /// 'Freezes' the relevant parts of the render configuration to a serialized `String` that can later be used to re-initialize the app to the same state at the time of freezing.
    fn freeze(&self) -> String {
        let frozen_app = FrozenApp {
            global_state: self.global_state.get().0.freeze(),
            route: match &*self.router.get_load_state().get_untracked() {
                RouterLoadState::Loaded { path, .. } => path,
                RouterLoadState::Loading { path, .. } => path,
                // If we encounter this during re-hydration, we won't try to set the URL in the browser
                RouterLoadState::Server => "SERVER",
            }
            .to_string(),
            page_state_store: self.page_state_store.freeze_to_hash_map(),
        };
        serde_json::to_string(&frozen_app).unwrap()
    }
}
impl<'a> RenderCtx<'a> {
    /// Commands Perseus to 'thaw' the app from the given frozen state. You'll also need to provide preferences for thawing, which allow you to control how different pages should prioritize
    /// frozen state over existing (or *active*) state. Once you call this, assume that any following logic will not run, as this may navigate to a different route in your app. How you get
    /// the frozen state to supply to this is up to you.
    ///
    /// If the app has already been thawed from a previous frozen state, any state used from that will be considered *active* for this thawing.
    ///
    /// This will return an error if the frozen state provided is invalid. However, if the frozen state for an individual page is invalid, it will be silently ignored in favor of either the
    /// active state or the server-provided state.
    pub fn thaw(&self, new_frozen_app: &str, thaw_prefs: ThawPrefs) -> Result<(), ClientError> {
        let new_frozen_app: FrozenApp = serde_json::from_str(new_frozen_app)
            .map_err(|err| ClientError::ThawFailed { source: err })?;
        let route = new_frozen_app.route.clone();
        // Set everything in the render context
        self.frozen_app
            .set_tuple(Some((new_frozen_app, thaw_prefs)));

        // Check if we're on the same page now as we were at freeze-time
        let curr_route = match &*self.router.get_load_state().get_untracked() {
                RouterLoadState::Loaded { path, .. } => path.to_string(),
                RouterLoadState::Loading { path, .. } => path.to_string(),
                // The user is trying to thaw on the server, which is an absolutely horrific idea (we should be generating state, and loops could happen)
                RouterLoadState::Server => panic!("attempted to thaw frozen state on server-side (you can only do this in the browser)"),
            };
        if curr_route == route {
            // We'll need to imperatively instruct the router to reload the current page (Sycamore can't do this yet)
            // We know the last verdict will be available because the only way we can be here is if we have a page
            self.router.reload();
        } else {
            // We aren't, navigate to the old route as usual
            navigate(&route);
        }

        Ok(())
    }
    /// An internal getter for the frozen state for the given page. When this is called, it will also add any frozen state
    /// it finds to the page state store, overriding what was already there.
    fn get_frozen_page_state_and_register<R>(
        &mut self,
        url: &str,
    ) -> Option<<R::Unrx as MakeRx>::Rx>
    where
        R: Clone + AnyFreeze + MakeUnrx,
        // We need this so that the compiler understands that the reactive version of the unreactive version of `R` has the same properties as `R` itself
        <<R as MakeUnrx>::Unrx as MakeRx>::Rx: Clone + AnyFreeze + MakeUnrx,
    {
        let frozen_app_full = self.frozen_app.get();
        // We don't use an `if let` here because that would require access to the `FrozenAppStoreState` wrapper type, which is private
        if frozen_app_full.is_some() {
            let frozen_app_full_ref = frozen_app_full.as_ref().as_ref().unwrap();
            let (frozen_app, thaw_prefs) = (&frozen_app_full_ref.0, &frozen_app_full_ref.1);
            // Check against the thaw preferences if we should prefer frozen state over active state
            if thaw_prefs.page.should_use_frozen_state(url) {
                // Get the serialized and unreactive frozen state from the store
                match frozen_app.page_state_store.get(url) {
                    Some(state_str) => {
                        // Deserialize into the unreactive version
                        let unrx = match serde_json::from_str::<R::Unrx>(state_str) {
                            Ok(unrx) => unrx,
                            // The frozen state could easily be corrupted, so we'll fall back to the active state (which is already reactive)
                            // We break out here to avoid double-storing this and trying to make a reactive thing reactive
                            Err(_) => return None,
                        };
                        // This returns the reactive version of the unreactive version of `R`, which is why we have to make everything else do the same
                        // Then we convince the compiler that that actually is `R` with the ludicrous trait bound at the beginning of this function
                        let rx = unrx.make_rx();
                        // And we do want to add this to the page state store
                        self.page_state_store.add(url, rx.clone());
                        // We have to drop this now so that we can `.take()` from the `Signal<T>` without a panic (this is the only borrow because the `.get()` call is public to this crate only)
                        drop(frozen_app_full);
                        // Now we should remove this from the frozen state so we don't fall back to it again
                        // We do this by taking out the `Rc<T>` from the `Signal`'s internal `RefCell<Rc<T>>`, which we can do because of the above `drop(...)`
                        // Then, we unwrap the `Rc`, which is fine to do because again of the above `drop(...)`
                        let mut frozen_app_val = self.frozen_app.take_unwrap().unwrap(); // We're literally in a conditional that checked this
                        frozen_app_val.0.page_state_store.remove(url);
                        self.frozen_app.set(Some(frozen_app_val));

                        Some(rx)
                    }
                    // If there's nothing in the frozen state, we'll fall back to the active state
                    None => self.page_state_store.get::<<R::Unrx as MakeRx>::Rx>(url),
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    /// An internal getter for the active (already registered) state for the given page.
    fn get_active_page_state<R>(&self, url: &str) -> Option<<R::Unrx as MakeRx>::Rx>
    where
        R: Clone + AnyFreeze + MakeUnrx,
        // We need this so that the compiler understands that the reactive version of the unreactive version of `R` has the same properties as `R` itself
        <<R as MakeUnrx>::Unrx as MakeRx>::Rx: Clone + AnyFreeze + MakeUnrx,
    {
        self.page_state_store.get::<<R::Unrx as MakeRx>::Rx>(url)
    }
    /// Gets either the active state or the frozen state for the given page. If `.thaw()` has been called, thaw preferences will be registered, which this will use to decide whether to use
    /// frozen or active state. If neither is available, the caller should use generated state instead.
    ///
    /// This takes a single type parameter for the reactive state type, from which the unreactive state type can be derived.
    pub fn get_active_or_frozen_page_state<R>(
        &mut self,
        url: &str,
    ) -> Option<<R::Unrx as MakeRx>::Rx>
    where
        R: Clone + AnyFreeze + MakeUnrx,
        // We need this so that the compiler understands that the reactive version of the unreactive version of `R` has the same properties as `R` itself
        <<R as MakeUnrx>::Unrx as MakeRx>::Rx: Clone + AnyFreeze + MakeUnrx,
    {
        let frozen_app_full = self.frozen_app.get();
        // We don't use an `if let` here because that would require access to the `FrozenAppStoreState` wrapper type, which is private
        if frozen_app_full.is_some() {
            let frozen_app_full_ref = frozen_app_full.as_ref().as_ref().unwrap();
            let thaw_prefs = &frozen_app_full_ref.1;
            // Check against the thaw preferences if we should prefer frozen state over active state
            if thaw_prefs.page.should_use_frozen_state(url) {
                drop(frozen_app_full);
                // We'll fall back to active state if no frozen state is available
                match self.get_frozen_page_state_and_register::<R>(url) {
                    Some(state) => Some(state),
                    None => self.get_active_page_state::<R>(url),
                }
            } else {
                drop(frozen_app_full);
                // We're preferring active state, but we'll fall back to frozen state if none is available
                match self.get_active_page_state::<R>(url) {
                    Some(state) => Some(state),
                    None => self.get_frozen_page_state_and_register::<R>(url),
                }
            }
        } else {
            // No frozen state exists, so we of course shouldn't prioritize it
            self.get_active_page_state::<R>(url)
        }
    }
    /// An internal getter for the frozen global state. When this is called, it will also add any frozen state to the registered
    /// global state, removing whatever was there before.
    fn get_frozen_global_state_and_register<R>(&mut self) -> Option<<R::Unrx as MakeRx>::Rx>
    where
        R: Clone + AnyFreeze + MakeUnrx,
        // We need this so that the compiler understands that the reactive version of the unreactive version of `R` has the same properties as `R` itself
        <<R as MakeUnrx>::Unrx as MakeRx>::Rx: Clone + AnyFreeze + MakeUnrx,
    {
        let frozen_app_full = self.frozen_app.get();
        if frozen_app_full.is_some() {
            let frozen_app_full_ref = frozen_app_full.as_ref().as_ref().unwrap();
            let (frozen_app, thaw_prefs) = (&frozen_app_full_ref.0, &frozen_app_full_ref.1);
            // Check against the thaw preferences if we should prefer frozen state over active state
            if thaw_prefs.global_prefer_frozen {
                // Get the serialized and unreactive frozen state from the store
                match frozen_app.global_state.as_str() {
                    // See `rx_state.rs` for why this would be the default value
                    "None" => None,
                    state_str => {
                        // Deserialize into the unreactive version
                        let unrx = match serde_json::from_str::<R::Unrx>(state_str) {
                            Ok(unrx) => unrx,
                            // The frozen state could easily be corrupted
                            Err(_) => return None,
                        };
                        // This returns the reactive version of the unreactive version of `R`, which is why we have to make everything else do the same
                        // Then we convince the compiler that that actually is `R` with the ludicrous trait bound at the beginning of this function
                        let rx = unrx.make_rx();
                        // And we'll register this as the new active global state
                        // This just references the `Signal` inside `GlobalState`
                        self.global_state.set(Box::new(rx.clone()));
                        // We have to drop this now so that we can `.take()` from the `Signal<T>` without a panic (this is the only borrow because the `.get()` call is public to this crate only)
                        drop(frozen_app_full);
                        // Now we should remove this from the frozen state so we don't fall back to it again
                        // We do this by taking out the `Rc<T>` from the `Signal`'s internal `RefCell<Rc<T>>`, which we can do because of the above `drop(...)`
                        // Then, we unwrap the `Rc`, which is fine to do because again of the above `drop(...)`
                        let mut frozen_app_val = self.frozen_app.take_unwrap().unwrap(); // We're literally in a conditional that checked this
                        frozen_app_val.0.global_state = "None".to_string();
                        self.frozen_app.set(Some(frozen_app_val));

                        Some(rx)
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    /// An internal getter for the active (already registered) global state.
    fn get_active_global_state<R>(&self) -> Option<<R::Unrx as MakeRx>::Rx>
    where
        R: Clone + AnyFreeze + MakeUnrx,
        // We need this so that the compiler understands that the reactive version of the unreactive version of `R` has the same properties as `R` itself
        <<R as MakeUnrx>::Unrx as MakeRx>::Rx: Clone + AnyFreeze + MakeUnrx,
    {
        self.global_state
            .get()
            .0
            .as_any()
            .downcast_ref::<<R::Unrx as MakeRx>::Rx>()
            .cloned()
    }
    /// Gets either the active or the frozen global state, depending on thaw preferences. Otherwise, this is exactly the same as `.get_active_or_frozen_state()`.
    pub fn get_active_or_frozen_global_state<R>(&mut self) -> Option<<R::Unrx as MakeRx>::Rx>
    where
        R: Clone + AnyFreeze + MakeUnrx,
        // We need this so that the compiler understands that the reactive version of the unreactive version of `R` has the same properties as `R` itself
        <<R as MakeUnrx>::Unrx as MakeRx>::Rx: Clone + AnyFreeze + MakeUnrx,
    {
        let frozen_app_full = self.frozen_app.get();
        if frozen_app_full.is_some() {
            let frozen_app_full_ref = frozen_app_full.as_ref().as_ref().unwrap();
            let thaw_prefs = &frozen_app_full_ref.1;
            // Check against the thaw preferences if we should prefer frozen state over active state
            if thaw_prefs.global_prefer_frozen {
                drop(frozen_app_full);
                // We'll fall back to the active state if there's no frozen state
                match self.get_frozen_global_state_and_register::<R>() {
                    Some(state) => Some(state),
                    None => self.get_active_global_state::<R>(),
                }
            } else {
                drop(frozen_app_full);
                // We'll fall back to the frozen state there's no active state available
                match self.get_active_global_state::<R>() {
                    Some(state) => Some(state),
                    None => self.get_frozen_global_state_and_register::<R>(),
                }
            }
        } else {
            // No frozen state exists, so we of course shouldn't prioritize it
            self.get_active_global_state::<R>()
        }
    }
    /// Registers a serialized and unreactive state string to the page state store, returning a fully reactive version.
    pub fn register_page_state_str<R>(
        &mut self,
        url: &str,
        state_str: &str,
    ) -> Result<<R::Unrx as MakeRx>::Rx, ClientError>
    where
        R: Clone + AnyFreeze + MakeUnrx,
        // We need this so that the compiler understands that the reactive version of the unreactive version of `R` has the same properties as `R` itself
        <<R as MakeUnrx>::Unrx as MakeRx>::Rx: Clone + AnyFreeze + MakeUnrx,
    {
        // Deserialize it (we know nothing about the calling situation, so we assume it could be invalid, hence the fallible return type)
        let unrx = serde_json::from_str::<R::Unrx>(state_str)
            .map_err(|err| ClientError::StateInvalid { source: err })?;
        let rx = unrx.make_rx();
        self.page_state_store.add(url, rx.clone());

        Ok(rx)
    }
    /// Registers a serialized and unreactive state string as the new active global state, returning a fully reactive version.
    pub fn register_global_state_str<R>(
        &mut self,
        state_str: &str,
    ) -> Result<<R::Unrx as MakeRx>::Rx, ClientError>
    where
        R: Clone + AnyFreeze + MakeUnrx,
        // We need this so that the compiler understands that the reactive version of the unreactive version of `R` has the same properties as `R` itself
        <<R as MakeUnrx>::Unrx as MakeRx>::Rx: Clone + AnyFreeze + MakeUnrx,
    {
        // Deserialize it (we know nothing about the calling situation, so we assume it could be invalid, hence the fallible return type)
        let unrx = serde_json::from_str::<R::Unrx>(state_str)
            .map_err(|err| ClientError::StateInvalid { source: err })?;
        let rx = unrx.make_rx();
        self.global_state.set(Box::new(rx.clone()));

        Ok(rx)
    }
}

/// Gets the `RenderCtx` efficiently.
#[macro_export]
macro_rules! get_render_ctx {
    ($cx:expr) => {
        ::perseus::templates::RenderCtx::from_ctx($cx)
    };
}
