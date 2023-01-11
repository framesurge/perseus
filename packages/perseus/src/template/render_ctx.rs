#[cfg(any(client, doc))]
use super::BrowserNodeType;
use super::{ArcTemplateMap, TemplateState, TemplateStateWithType};
use crate::{PathMaybeWithLocale, PathWithoutLocale, errors::*};
use crate::router::{RouterLoadState, RouterState};
use crate::state::{
    AnyFreeze, Freeze, FrozenApp, GlobalState, GlobalStateType, MakeRx, MakeRxRef, MakeUnrx,
    PageStateStore, RxRef, ThawPrefs,
};
use crate::stores::ImmutableStore;
use std::cell::RefCell;
use std::rc::Rc;
use serde_json::Value;
use sycamore::prelude::{provide_context, use_context, Scope};
use sycamore::web::{Html, SsrNode};
use sycamore_router::navigate;
#[cfg(engine)]
use std::collections::HashMap;



/// A representation of the render context of the app, constructed from
/// references to a series of `struct`s that mirror context values. This is
/// purely a proxy `struct` for function organization.
#[derive(Debug)]
pub struct RenderCtx {
    // /// A translator for templates to use. This will still be present in non-i18n apps, but it
    // will have no message IDs and support for /// the non-existent locale `xx-XX`. This uses
    // an `Arc<T>` for thread-safety. translator: Translator,
    /// The router's state.
    pub router: RouterState,
    /// The page state store for the app. This is a type map to which pages can
    /// add state that they need to access later. Usually, this will be
    /// interfaced with through the `#[perseus::template_with_rx_state(...
    /// )]` macro, but it can be used manually as well to get the state of one
    /// page from another (provided that the target page has already
    /// been visited).
    pub page_state_store: PageStateStore,
    /// The user-provided global state. This is stored on the heap to avoid a
    /// type parameter that would be needed every time we had to access the
    /// render context (which would be very difficult to pass around inside
    /// Perseus).
    ///
    /// Because we store `dyn Any` in here, we initialize it as `Option::None`,
    /// and then the template macro (which does the heavy lifting for global
    /// state) will find that it can't downcast to the user's global state
    /// type, which will prompt it to deserialize whatever global state it was
    /// given and then write that here.
    ///
    /// This is `pub(crate)` to prevent users accessing this without using the
    /// wrappers in `RenderCtx` that handle thawing. If this direct access were
    /// to occur, the likelihood of a `BorrowMut` panic would be *substantial*!
    /// Essentially, you don't know when you get the global state how it's going
    /// to be initialized (e.g. it might come from the server, or maybe from a
    /// frozen state), so the seemingly read-only method
    /// `.get_global_state()` might actually need to mutate the underlying
    /// state multiple times, which would fail with a panic if there were
    /// any existing borrows of the global state. By making this private, we
    /// prevent this.
    pub(crate) global_state: GlobalState,
    /// A previous state the app was once in, still serialized. This will be
    /// rehydrated gradually by the template macro.
    pub frozen_app: Rc<RefCell<Option<(FrozenApp, ThawPrefs)>>>,
    /// The app's error pages. If you need to render an error, you should use
    /// these!
    ///
    /// **Warning:** these don't exist on the engine-side! But, there, you
    /// should always return a build-time error rather than produce a page
    /// with an error in it.
    #[cfg(any(client, doc))]
    pub error_pages: Rc<crate::error_pages::ErrorPages<BrowserNodeType>>,
    // --- PRIVATE FIELDS ---
    // Any users accessing these are *extremely* likely to shoot themselves in the foot!
    /// Whether or not this page is the very first to have been rendered since
    /// the browser loaded the app. This will be reset on full reloads, and is
    /// used internally to determine whether or not we should look for
    /// stored HSR state.
    #[cfg(any(client, doc))]
    pub(crate) is_first: Rc<std::cell::Cell<bool>>,
    /// The locales, for use in routing.
    #[cfg(any(client, doc))]
    pub(crate) locales: crate::i18n::Locales,
    /// The map of all templates in the app, for use in routing.
    #[cfg(any(client, doc))]
    pub(crate) templates: crate::template::TemplateMap<BrowserNodeType>,
    /// The render configuration, for use in routing.
    #[cfg(any(client, doc))]
    pub(crate) render_cfg: Rc<std::collections::HashMap<String, String>>,
    /// The client-side translations manager.
    #[cfg(any(client, doc))]
    pub(crate) translations_manager: crate::i18n::ClientTranslationsManager,
    /// The mode we're currently rendering in. This will be used primarily in widget rendering.
    ///
    /// While a user could *hypothetically* render in a manner that's dependent on this, that is
    /// never going to be a good idea, since the internals of this could change in any release. Hence,
    /// we keep it private to the crate.
    #[cfg(engine)]
    pub(crate) render_mode: RenderMode<SsrNode>,
}
impl Freeze for RenderCtx {
    /// 'Freezes' the relevant parts of the render configuration to a serialized
    /// `String` that can later be used to re-initialize the app to the same
    /// state at the time of freezing.
    fn freeze(&self) -> String {
        let frozen_app = FrozenApp {
            global_state: self.global_state.0.borrow().freeze(),
            route: match &*self.router.get_load_state_rc().get_untracked() {
                RouterLoadState::Loaded { path, .. } => Some(path.clone()),
                RouterLoadState::Loading { path, .. } => Some(path.clone()),
                // If we encounter this during re-hydration, we won't try to set the URL in the
                // browser
                RouterLoadState::ErrorLoaded { .. } => None,
                RouterLoadState::Server => None,
            },
            page_state_store: self.page_state_store.freeze_to_hash_map(),
        };
        serde_json::to_string(&frozen_app).unwrap()
    }
}
#[cfg(engine)] // To prevent foot-shooting
impl RenderCtx {
    /// Initializes a new `RenderCtx` on the server-side with the given global
    /// state and set of widget states.
    pub(crate) fn server(global_state: TemplateState, mode: RenderMode<SsrNode>) -> Self {
        Self {
            router: RouterState::default(),
            page_state_store: PageStateStore::new(0), /* There will be no need for the PSS on the
                                                       * server-side */
            global_state: if !global_state.is_empty() {
                GlobalState::new(GlobalStateType::Server(global_state))
            } else {
                GlobalState::new(GlobalStateType::None)
            },
            frozen_app: Rc::new(RefCell::new(None)),
            render_mode: mode,
        }
    }
}
impl RenderCtx {
    /// Creates a new instance of the render context, with the given maximum
    /// size for the page state store, and other properties.
    ///
    /// Note: this is designed for client-side usage, use `::server()` on the
    /// engine-side.
    ///
    /// Note also that this will automatically extract global state from page
    /// variables.
    #[cfg(any(client, doc))] // To prevent foot-shooting
    pub(crate) fn new(
        pss_max_size: usize,
        locales: crate::i18n::Locales,
        templates: crate::template::TemplateMap<BrowserNodeType>,
        render_cfg: Rc<std::collections::HashMap<String, String>>,
        error_pages: Rc<crate::error_pages::ErrorPages<BrowserNodeType>>,
    ) -> Self {
        let translations_manager = crate::i18n::ClientTranslationsManager::new(&locales);
        Self {
            router: RouterState::default(),
            page_state_store: PageStateStore::new(pss_max_size),
            global_state: match get_global_state() {
                Some(global_state) => GlobalState::new(GlobalStateType::Server(global_state)),
                None => GlobalState::new(GlobalStateType::None),
            },
            frozen_app: Rc::new(RefCell::new(None)),
            is_first: Rc::new(std::cell::Cell::new(true)),
            error_pages,
            locales,
            templates,
            render_cfg,
            translations_manager,
        }
    }
    // TODO Use a custom, optimized context system instead of Sycamore's? (GIven we
    // only need to store one thing...)
    /// Gets an instance of `RenderCtx` out of Sycamore's context system.
    pub fn from_ctx(cx: Scope) -> &Self {
        use_context::<Self>(cx)
    }
    /// Places this instance of `RenderCtx` into Sycamore's context system,
    /// returning a reference. This assumes no other instances of `RenderCtx`
    /// have been added to context already (or Sycamore will cause a panic).
    /// Once this is done, the render context can be modified safely with
    /// interior mutability.
    pub(crate) fn set_ctx(self, cx: Scope) -> &Self {
        provide_context(cx, self)
    }
    /// Preloads the given URL from the server and caches it, preventing
    /// future network requests to fetch that page.
    ///
    /// This function automatically defers the asynchronous preloading
    /// work to a browser future for convenience. If you would like to
    /// access the underlying future, use `.try_preload()` instead.
    ///
    /// # Panics
    /// This function will panic if any errors occur in preloading, such as
    /// the route being not found, or not localized. If the path you're
    /// preloading is not hardcoded, use `.try_preload()` instead.
    // Conveniently, we can use the lifetime mechanics of knowing that the render context
    // is registered on the given scope to ensure that the future works out
    #[cfg(any(client, doc))]
    pub fn preload<'a, 'b: 'a>(&'b self, cx: Scope<'a>, url: &PathMaybeWithLocale) {
        use fmterr::fmt_err;

        crate::spawn_local_scoped(cx, async move {
            if let Err(err) = self.try_preload(&url).await {
                panic!("{}", fmt_err(&err));
            }
        });
    }
    /// Preloads the given URL from the server and caches it for the current
    /// route, preventing future network requests to fetch that page. On a
    /// route transition, this will be removed.
    ///
    /// WARNING: the route preloading system is under heavy construction at
    /// present!
    ///
    /// This function automatically defers the asynchronous preloading
    /// work to a browser future for convenience. If you would like to
    /// access the underlying future, use `.try_route_preload()` instead.
    ///
    /// # Panics
    /// This function will panic if any errors occur in preloading, such as
    /// the route being not found, or not localized. If the path you're
    /// preloading is not hardcoded, use `.try_route_preload()` instead.
    // Conveniently, we can use the lifetime mechanics of knowing that the render context
    // is registered on the given scope to ensure that the future works out
    #[cfg(any(client, doc))]
    pub fn route_preload<'a, 'b: 'a>(&'b self, cx: Scope<'a>, url: &PathMaybeWithLocale) {
        use fmterr::fmt_err;

        crate::spawn_local_scoped(cx, async move {
            if let Err(err) = self.try_route_preload(&url).await {
                panic!("{}", fmt_err(&err));
            }
        });
    }
    /// A version of `.preload()` that returns a future that can resolve to an
    /// error. If the path you're preloading is not hardcoded, you should
    /// use this.
    #[cfg(any(client, doc))]
    pub async fn try_preload(&self, url: &PathMaybeWithLocale) -> Result<(), ClientError> {
        self._preload(url, false).await
    }
    /// A version of `.route_preload()` that returns a future that can resolve
    /// to an error. If the path you're preloading is not hardcoded, you
    /// should use this.
    #[cfg(any(client, doc))]
    pub async fn try_route_preload(&self, url: &PathMaybeWithLocale) -> Result<(), ClientError> {
        self._preload(url, true).await
    }
    /// Preloads the given URL from the server and caches it, preventing
    /// future network requests to fetch that page.
    #[cfg(any(client, doc))]
    async fn _preload(&self, path: &PathMaybeWithLocale, is_route_preload: bool) -> Result<(), ClientError> {
        use crate::router::{match_route, RouteVerdict};

        let path_segments = path
            .split('/')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>(); // This parsing is identical to the Sycamore router's
                                     // Get a route verdict on this so we know where we're going (this doesn't modify
                                     // the router state)
        let verdict = match_route(
            &path_segments,
            &self.render_cfg,
            &self.templates,
            &self.locales,
        );
        // Make sure we've got a valid verdict (otherwise the user should be told there
        // was an error)
        let route_info = match verdict {
            RouteVerdict::Found(info) => info,
            RouteVerdict::NotFound => return Err(ClientError::PreloadNotFound),
            RouteVerdict::LocaleDetection(_) => return Err(ClientError::PreloadLocaleDetection),
        };

        // We just needed to acquire the arguments to this function
        self.page_state_store
            .preload(
                path,
                &route_info.locale,
                &route_info.template.get_path(),
                route_info.was_incremental_match,
                is_route_preload,
            )
            .await
    }
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
    /// If the app was last frozen while on an error page, this will not attempt
    /// to change the current route.
    pub fn thaw(&self, new_frozen_app: &str, thaw_prefs: ThawPrefs) -> Result<(), ClientError> {
        let new_frozen_app: FrozenApp = serde_json::from_str(new_frozen_app)
            .map_err(|err| ClientError::ThawFailed { source: err })?;
        let route = new_frozen_app.route.clone();
        // Set everything in the render context
        let mut frozen_app = self.frozen_app.borrow_mut();
        *frozen_app = Some((new_frozen_app, thaw_prefs));
        // I'm not absolutely certain about destructor behavior with navigation or how
        // that could change with the new primitives, so better to be safe than sorry
        drop(frozen_app);

        // Check if we're on the same page now as we were at freeze-time
        let curr_route = match &*self.router.get_load_state_rc().get_untracked() {
                RouterLoadState::Loaded { path, .. } => path.clone(),
                RouterLoadState::Loading { path, .. } => path.clone(),
                // We're in an error state, so we have no choice but to go to the old route
                RouterLoadState::ErrorLoaded { location } => todo!("thawing while in an error state is not yet implemented"),
                // The user is trying to thaw on the server, which is an absolutely horrific idea (we should be generating state, and loops could happen)
                RouterLoadState::Server => panic!("attempted to thaw frozen state on server-side (you can only do this in the browser)"),
            };
        // We handle the possibility that the page tried to reload before it had been
        // made interactive here (we'll just reload wherever we are)
        if let Some(route) = route {
            // If we're on the same page, just reload, otherwise go to the frozen route
            if curr_route == route {
                self.router.reload();
            } else {
                navigate(&route);
            }
        } else {
            // The page froze before hydration, so we'll jsut reload
            self.router.reload();
        }

        Ok(())
    }
    /// An internal getter for the frozen state for the given page. When this is
    /// called, it will also add any frozen state it finds to the page state
    /// store, overriding what was already there.
    ///
    /// **Warning:** if the page has already been registered in the page state
    /// store as not being able to receive state, this will silently fail.
    /// If this occurs, something has gone horribly wrong, and panics will
    /// almost certainly follow. (Basically, this should *never* happen. If
    /// you're not using the macros, you may need to be careful of this.)
    fn get_frozen_page_state_and_register<R>(&self, url: &PathMaybeWithLocale, is_widget: bool) -> Option<<R::Unrx as MakeRx>::Rx>
    where
        R: Clone + AnyFreeze + MakeUnrx,
        // We need this so that the compiler understands that the reactive version of the
        // unreactive version of `R` has the same properties as `R` itself
        <<R as MakeUnrx>::Unrx as MakeRx>::Rx: Clone + AnyFreeze + MakeUnrx,
    {
        let frozen_app_full = self.frozen_app.borrow();
        if let Some((frozen_app, thaw_prefs)) = &*frozen_app_full {
            // Check against the thaw preferences if we should prefer frozen state over
            // active state
            if thaw_prefs.page.should_use_frozen_state(url) {
                // Get the serialized and unreactive frozen state from the store
                match frozen_app.page_state_store.get(&url) {
                    Some(state_str) => {
                        // Deserialize into the unreactive version
                        let unrx = match serde_json::from_str::<R::Unrx>(state_str) {
                            Ok(unrx) => unrx,
                            // The frozen state could easily be corrupted, so we'll fall back to the
                            // active state (which is already reactive)
                            // We break out here to avoid double-storing this and trying to make a
                            // reactive thing reactive
                            Err(_) => return None,
                        };
                        // This returns the reactive version of the unreactive version of `R`, which
                        // is why we have to make everything else do the same
                        // Then we convince the compiler that that actually is `R` with the
                        // ludicrous trait bound at the beginning of this function
                        let rx = unrx.make_rx();
                        // And we do want to add this to the page state store (if this returns
                        // false, then this page was never supposed to receive state)
                        if !self.page_state_store.add_state(url, rx.clone(), is_widget) {
                            return None;
                        }
                        // Now we should remove this from the frozen state so we don't fall back to
                        // it again
                        drop(frozen_app_full);
                        let mut frozen_app_val = self.frozen_app.take().unwrap(); // We're literally in a conditional that checked this
                        frozen_app_val.0.page_state_store.remove(url);
                        let mut frozen_app = self.frozen_app.borrow_mut();
                        *frozen_app = Some(frozen_app_val);

                        Some(rx)
                    }
                    // If there's nothing in the frozen state, we'll fall back to the active state
                    None => self
                        .page_state_store
                        .get_state::<<R::Unrx as MakeRx>::Rx>(url),
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    /// An internal getter for the active (already registered) state for the
    /// given page.
    fn get_active_page_state<R>(&self, url: &PathMaybeWithLocale) -> Option<<R::Unrx as MakeRx>::Rx>
    where
        R: Clone + AnyFreeze + MakeUnrx,
        // We need this so that the compiler understands that the reactive version of the
        // unreactive version of `R` has the same properties as `R` itself
        <<R as MakeUnrx>::Unrx as MakeRx>::Rx: Clone + AnyFreeze + MakeUnrx,
    {
        self.page_state_store
            .get_state::<<R::Unrx as MakeRx>::Rx>(url)
    }
    /// Gets either the active state or the frozen state for the given page. If
    /// `.thaw()` has been called, thaw preferences will be registered, which
    /// this will use to decide whether to use frozen or active state. If
    /// neither is available, the caller should use generated state instead.
    ///
    /// This takes a single type parameter for the reactive state type, from
    /// which the unreactive state type can be derived.
    pub fn get_active_or_frozen_page_state<R>(&self, url: &PathMaybeWithLocale, is_widget: bool) -> Option<<R::Unrx as MakeRx>::Rx>
    where
        R: Clone + AnyFreeze + MakeUnrx,
        // We need this so that the compiler understands that the reactive version of the
        // unreactive version of `R` has the same properties as `R` itself
        <<R as MakeUnrx>::Unrx as MakeRx>::Rx: Clone + AnyFreeze + MakeUnrx,
    {
        let frozen_app_full = self.frozen_app.borrow();
        if let Some((_, thaw_prefs)) = &*frozen_app_full {
            // Check against the thaw preferences if we should prefer frozen state over
            // active state
            if thaw_prefs.page.should_use_frozen_state(url) {
                drop(frozen_app_full);
                // We'll fall back to active state if no frozen state is available
                match self.get_frozen_page_state_and_register::<R>(url, is_widget) {
                    Some(state) => Some(state),
                    None => self.get_active_page_state::<R>(url),
                }
            } else {
                drop(frozen_app_full);
                // We're preferring active state, but we'll fall back to frozen state if none is
                // available
                match self.get_active_page_state::<R>(url) {
                    Some(state) => Some(state),
                    None => self.get_frozen_page_state_and_register::<R>(url, is_widget),
                }
            }
        } else {
            // No frozen state exists, so we of course shouldn't prioritize it
            self.get_active_page_state::<R>(url)
        }
    }
    /// An internal getter for the frozen global state. When this is called, it
    /// will also add any frozen state to the registered global state,
    /// removing whatever was there before.
    ///
    /// If the app was frozen before the global state from the server had been
    /// parsed, this will return `None` (i.e. there will only be frozen
    /// global state if the app had handled it before it froze).
    fn get_frozen_global_state_and_register<R>(&self) -> Option<<R::Unrx as MakeRx>::Rx>
    where
        R: Clone + AnyFreeze + MakeUnrx,
        // We need this so that the compiler understands that the reactive version of the
        // unreactive version of `R` has the same properties as `R` itself
        <<R as MakeUnrx>::Unrx as MakeRx>::Rx: Clone + AnyFreeze + MakeUnrx,
    {
        let frozen_app_full = self.frozen_app.borrow();
        if let Some((frozen_app, thaw_prefs)) = &*frozen_app_full {
            // Check against the thaw preferences if we should prefer frozen state over
            // active state
            if thaw_prefs.global_prefer_frozen {
                // Get the serialized and unreactive frozen state from the store
                match frozen_app.global_state.as_str() {
                    // There was no global state (hypothetically, a bundle change could mean there's
                    // some now...)
                    "None" => None,
                    // There was some global state from the server that hadn't been dealt with yet
                    // (it will be extracted)
                    "Server" => None,
                    state_str => {
                        // Deserialize into the unreactive version
                        let unrx = match serde_json::from_str::<R::Unrx>(state_str) {
                            Ok(unrx) => unrx,
                            // The frozen state could easily be corrupted
                            Err(_) => return None,
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
                        frozen_app_val.0.global_state = "None".to_string();
                        let mut frozen_app = self.frozen_app.borrow_mut();
                        *frozen_app = Some(frozen_app_val);

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
    ///
    /// If the global state from the server hasn't been parsed yet, this will
    /// return `None`.
    fn get_active_global_state<R>(&self) -> Option<<R::Unrx as MakeRx>::Rx>
    where
        R: Clone + AnyFreeze + MakeUnrx,
        // We need this so that the compiler understands that the reactive version of the
        // unreactive version of `R` has the same properties as `R` itself
        <<R as MakeUnrx>::Unrx as MakeRx>::Rx: Clone + AnyFreeze + MakeUnrx,
    {
        self.global_state.0.borrow().parse_active::<R>()
    }
    /// Gets either the active or the frozen global state, depending on thaw
    /// preferences. Otherwise, this is exactly the same as
    /// `.get_active_or_frozen_state()`.
    ///
    /// If the state from the server hadn't been handled by the previous freeze,
    /// and it hasn't been handled yet, this will return `None`.
    fn get_active_or_frozen_global_state<R>(&self) -> Option<<R::Unrx as MakeRx>::Rx>
    where
        R: Clone + AnyFreeze + MakeUnrx,
        // We need this so that the compiler understands that the reactive version of the
        // unreactive version of `R` has the same properties as `R` itself
        <<R as MakeUnrx>::Unrx as MakeRx>::Rx: Clone + AnyFreeze + MakeUnrx,
    {
        let frozen_app_full = self.frozen_app.borrow();
        if let Some((_, thaw_prefs)) = &*frozen_app_full {
            // Check against the thaw preferences if we should prefer frozen state over
            // active state
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
    /// Registers a deserialized and unreactive [`TemplateState`] to the page
    /// state store, returning a fully reactive version.
    ///
    /// **Warning:** if the page has already been registered in the page state
    /// store as not being able to receive state, this will silently fail
    /// (i.e. the state will be returned, but it won't be registered). If this
    /// occurs, something has gone horribly wrong, and panics will almost
    /// certainly follow. (Basically, this should *never* happen. If you're
    /// not using the macros, you may need to be careful of this.)
    pub fn register_page_state_value<R>(
        &self,
        url: &PathMaybeWithLocale,
        state: TemplateStateWithType<R::Unrx>,
        is_widget: bool,
    ) -> Result<<R::Unrx as MakeRx>::Rx, ClientError>
    where
        R: Clone + AnyFreeze + MakeUnrx,
        // We need this so that the compiler understands that the reactive version of the
        // unreactive version of `R` has the same properties as `R` itself
        <<R as MakeUnrx>::Unrx as MakeRx>::Rx: Clone + AnyFreeze + MakeUnrx,
    {
        // Deserialize it (we know nothing about the calling situation, so we assume it
        // could be invalid, hence the fallible return type)
        let unrx = state
            .to_concrete()
            .map_err(|err| ClientError::StateInvalid { source: err })?;
        let rx = unrx.make_rx();
        // Potential silent failure (see above)
        let _ = self.page_state_store.add_state(url, rx.clone(), is_widget);

        Ok(rx)
    }
    /// Registers a serialized and unreactive state string as the new active
    /// global state, returning a fully reactive version.
    fn register_global_state_value<R>(
        &self,
        state: TemplateStateWithType<R::Unrx>,
    ) -> Result<<R::Unrx as MakeRx>::Rx, ClientError>
    where
        R: Clone + AnyFreeze + MakeUnrx,
        // We need this so that the compiler understands that the reactive version of the
        // unreactive version of `R` has the same properties as `R` itself
        <<R as MakeUnrx>::Unrx as MakeRx>::Rx: Clone + AnyFreeze + MakeUnrx,
    {
        // Deserialize it (we know nothing about the calling situation, so we assume it
        // could be invalid, hence the fallible return type)
        let unrx = state
            .to_concrete()
            .map_err(|err| ClientError::StateInvalid { source: err })?;
        let rx = unrx.make_rx();
        let mut active_global_state = self.global_state.0.borrow_mut();
        *active_global_state = GlobalStateType::Loaded(Box::new(rx.clone()));

        Ok(rx)
    }
    /// Registers a page as definitely taking no state, which allows it to be
    /// cached fully, preventing unnecessary network requests. Any future
    /// attempt to set state will lead to silent failures and/or panics.
    pub fn register_page_no_state(&self, url: &PathMaybeWithLocale, is_widget: bool) {
        self.page_state_store.set_state_never(url, is_widget);
    }

    /// Gets the global state.
    ///
    /// This is operating on a render context that has already defined its
    /// thawing preferences, so we can intelligently choose whether or not
    /// to use any frozen global state here if nothing has been initialized.
    ///
    /// # Panics
    ///
    /// This will panic if the app has no global state. If you don't know
    /// whether or not there is global state, use `.try_global_state()`
    /// instead.
    // This function takes the final ref struct as a type parameter! That
    // complicates everything substantially.
    pub fn get_global_state<'a, R>(
        &self,
        cx: Scope<'a>,
    ) -> <<<<R as RxRef>::RxNonRef as MakeUnrx>::Unrx as MakeRx>::Rx as MakeRxRef>::RxRef<'a>
    where
        R: RxRef,
        // We need this so that the compiler understands that the reactive version of the
        // unreactive version of `R` has the same properties as `R` itself
        <<<R as RxRef>::RxNonRef as MakeUnrx>::Unrx as MakeRx>::Rx:
            Clone + AnyFreeze + MakeUnrx + MakeRxRef,
        <R as RxRef>::RxNonRef: MakeUnrx + AnyFreeze + Clone,
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
    ) -> Result<
        Option<
            // Note: I am sorry.
            <<<<R as RxRef>::RxNonRef as MakeUnrx>::Unrx as MakeRx>::Rx as MakeRxRef>::RxRef<'a>,
        >,
        ClientError,
    >
    where
        R: RxRef,
        // We need this so that the compiler understands that the reactive version of the
        // unreactive version of `R` has the same properties as `R` itself
        <<<R as RxRef>::RxNonRef as MakeUnrx>::Unrx as MakeRx>::Rx:
            Clone + AnyFreeze + MakeUnrx + MakeRxRef,
        <R as RxRef>::RxNonRef: MakeUnrx + AnyFreeze + Clone,
    {
        let global_state_ty = self.global_state.0.borrow();
        // Bail early if the app doesn't support global state
        if let GlobalStateType::None = *global_state_ty {
            return Ok(None);
        }
        // Check if there's an actively loaded state or a frozen state (note
        // that this will set up global state if there was something in the frozen data,
        // hence it needs any other borrows to be dismissed)
        drop(global_state_ty);
        let rx_state = if let Some(rx_state) =
            self.get_active_or_frozen_global_state::<<R as RxRef>::RxNonRef>()
        {
            rx_state
        } else {
            // There was nothing, so we'll load from the server
            let global_state_ty = self.global_state.0.borrow();
            if let GlobalStateType::Server(server_state) = &*global_state_ty {
                let server_state = server_state.clone();
                let server_state =
                    server_state.change_type::<<<R as RxRef>::RxNonRef as MakeUnrx>::Unrx>();
                // The registration borrows mutably, so we have to drop here
                drop(global_state_ty);
                self.register_global_state_value::<<R as RxRef>::RxNonRef>(server_state)?
            } else {
                // We bailed on `None` earlier, and `.get_active_or_frozen_global_state()`
                // would've caught `Loaded`
                unreachable!()
            }
        };

        // Now use the context we have to convert that to a reference struct
        let ref_rx_state = rx_state.to_ref_struct(cx);

        Ok(Some(ref_rx_state))
    }
}

/// Gets the global state injected by the server, if there was any. If there are
/// errors in this, we can return `None` and not worry about it, they'll be
/// handled by the initial state.
///
/// Note that apps without global state will get `Some(..)` of an empty template
/// state here.
#[cfg(any(client, doc))]
fn get_global_state() -> Option<TemplateState> {
    let val_opt = web_sys::window().unwrap().get("__PERSEUS_GLOBAL_STATE");
    let js_obj = match val_opt {
        Some(js_obj) => js_obj,
        None => return None,
    };
    // The object should only actually contain the string value that was injected
    let state_str = match js_obj.as_string() {
        Some(state_str) => state_str,
        None => return None,
    };
    match TemplateState::from_str(&state_str) {
        Ok(state) => Some(state),
        // TODO Is this really valid now?
        Err(_) => None,
    }
}
