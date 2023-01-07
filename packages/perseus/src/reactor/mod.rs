#[cfg(client)]
mod error;
mod global_state;
#[cfg(all(feature = "hsr", debug_assertions, client))]
mod hsr;
#[cfg(client)]
mod initial_load;
#[cfg(engine)]
mod render_mode;
#[cfg(client)]
mod start;
mod state;
#[cfg(client)]
mod subsequent_load;
mod widget_state;

#[cfg(client)]
pub(crate) use initial_load::InitialView;
#[cfg(engine)]
pub(crate) use render_mode::{RenderMode, RenderStatus};

// --- Common imports ---
#[cfg(client)]
use crate::template::{BrowserNodeType, EntityMap};
use crate::{
    i18n::Translator,
    state::{GlobalState, GlobalStateType, PageStateStore, TemplateState},
};
use sycamore::{
    prelude::{provide_context, use_context, Scope},
    web::Html,
};

// --- Engine-side imports ---

// --- Browser-side imports ---
#[cfg(client)]
use crate::{
    error_views::ErrorViews,
    errors::ClientError,
    errors::ClientInvariantError,
    i18n::{ClientTranslationsManager, Locales, TranslationsManager},
    init::PerseusAppBase,
    plugins::PluginAction,
    router::RouterState,
    state::{FrozenApp, ThawPrefs},
    stores::MutableStore,
};
#[cfg(client)]
use serde::{de::DeserializeOwned, Serialize};
#[cfg(client)]
use serde_json::Value;
#[cfg(client)]
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};
#[cfg(client)]
use sycamore::{
    reactive::{create_rc_signal, RcSignal},
    view::View,
};

/// The core of Perseus' browser-side systems. This forms a central point for
/// all the Perseus state and rendering logic to operate from. In your own code,
/// this will always be available in the Sycamore context system.
///
/// Note that this is also used on the engine-side for rendering.
#[derive(Debug)]
pub struct Reactor<G: Html> {
    /// The state store, which is used to hold all reactive states, along with
    /// preloads.
    pub(crate) state_store: PageStateStore,
    /// The router state.
    #[cfg(client)]
    pub router_state: RouterState,
    /// The user-provided global state, stored with similar mechanics to the
    /// state store, although optimised.
    global_state: GlobalState,

    // --- Browser-side only ---
    /// A previous state the app was once in, still serialized. This will be
    /// rehydrated gradually by the template closures.
    ///
    /// The `bool` in here will be set to `true` if this was created through
    /// HSR, which has slightly more lenient thawing procedures to allow for
    /// data model changes.
    #[cfg(client)]
    frozen_app: Rc<RefCell<Option<(FrozenApp, ThawPrefs, bool)>>>,
    /// Whether or not this page is the very first to have been rendered since
    /// the browser loaded the app. This will be reset on full reloads, and is
    /// used internally to determine whether or not we should look for
    /// stored HSR state.
    #[cfg(client)]
    pub(crate) is_first: Cell<bool>,
    /// The app's *full* render configuration. Note that a subset of this
    /// is contained in the [`RenderMode`] on the engine-side for widget
    /// rendering.
    #[cfg(client)]
    pub(crate) render_cfg: HashMap<String, String>,
    /// The app's templates and capsules for use in routing.
    #[cfg(client)]
    pub(crate) entities: EntityMap<G>,
    /// The app's locales.
    #[cfg(client)]
    pub(crate) locales: Locales,
    /// The browser-side translations manager.
    #[cfg(client)]
    translations_manager: ClientTranslationsManager,
    /// The app's error views.
    #[cfg(client)]
    pub(crate) error_views: Rc<ErrorViews<G>>,
    /// A reactive container for the current page-wide view. This will usually
    /// contain the contents of the current page, but it may also contain a
    /// page-wide error. This will be wrapped in a router.
    #[cfg(client)]
    current_view: RcSignal<View<BrowserNodeType>>,
    /// A reactive container for any popup errors.
    #[cfg(client)]
    popup_error_view: RcSignal<View<BrowserNodeType>>,
    /// The app's root div ID.
    #[cfg(client)]
    root: String,

    // --- Engine-side only ---
    #[cfg(engine)]
    pub(crate) render_mode: RenderMode<G>,
    /// The currently active translator. On the browser-side, this is handled by
    /// the more fully-fledged `ClientTranslationsManager` type.
    ///
    /// This is provided to the engine-side reactor on instantiation. This can
    /// be `None` in certain error view renders.
    #[cfg(engine)]
    translator: Option<Translator>,
}

// This uses window variables set by the HTML shell, so it should never be used
// on the engine-side
#[cfg(client)]
impl<G: Html, M: MutableStore, T: TranslationsManager> TryFrom<PerseusAppBase<G, M, T>>
    for Reactor<G>
{
    type Error = ClientError;

    fn try_from(app: PerseusAppBase<G, M, T>) -> Result<Self, Self::Error> {
        let locales = app.get_locales()?;
        let root = app.get_root()?;
        let plugins = &app.plugins;

        plugins
            .functional_actions
            .client_actions
            .start
            .run((), plugins.get_plugin_data())?;

        // We need to fetch some things from window variables
        let render_cfg =
            match WindowVariable::<HashMap<String, String>>::new_obj("__PERSEUS_RENDER_CFG") {
                WindowVariable::Some(render_cfg) => render_cfg,
                WindowVariable::None | WindowVariable::Malformed => {
                    return Err(ClientInvariantError::RenderCfg.into())
                }
            };
        let global_state_ty = match WindowVariable::<Value>::new_obj("__PERSEUS_GLOBAL_STATE") {
            WindowVariable::Some(val) => {
                let state = TemplateState::from_value(val);
                if state.is_empty() {
                    // TODO Since we have it to hand, just make sure the global state creator really
                    // wasn't going to create anything (otherwise fail
                    // immediately)
                    GlobalStateType::None
                } else {
                    GlobalStateType::Server(state)
                }
            }
            WindowVariable::None => GlobalStateType::None,
            WindowVariable::Malformed => return Err(ClientInvariantError::GlobalState.into()),
        };

        Ok(Self {
            // This instantiates as if for the engine-side, but it will rapidly be changed
            router_state: RouterState::default(),
            state_store: PageStateStore::new(app.pss_max_size),
            global_state: GlobalState::new(global_state_ty),
            translations_manager: ClientTranslationsManager::new(&locales),
            // This will be filled out by a `.thaw()` call or HSR
            frozen_app: Rc::new(RefCell::new(None)),
            is_first: Cell::new(true),
            current_view: create_rc_signal(View::empty()),
            popup_error_view: create_rc_signal(View::empty()),
            entities: app.entities,
            locales,
            render_cfg,
            #[cfg(debug_assertions)]
            error_views: app.error_views.unwrap_or_default(),
            #[cfg(not(debug_assertions))]
            error_views: app
                .error_views
                .expect("you must provide your own error views in production"),
            root,
        })
    }
}

impl<G: Html> Reactor<G> {
    /// Adds `self` to the given Sycamore scope as context.
    ///
    /// # Panics
    /// This will panic if any other reactor is found in the context.
    pub(crate) fn add_self_to_cx(self, cx: Scope) {
        provide_context(cx, self);
    }
    /// Gets a [`Reactor`] out of the given Sycamore scope's context.
    ///
    /// You should never need to worry about this function panicking, since
    /// your code will only ever run if a reactor is present.
    pub fn from_cx(cx: Scope) -> &Self {
        use_context::<Self>(cx)
    }
    /// Gets the currently active translator.
    ///
    /// On the browser-side, this will return `None` under some error
    /// conditions, or before the initial load.
    ///
    /// On the engine-side, this will return `None` under certain error
    /// conditions.
    #[cfg(client)]
    pub fn try_get_translator(&self) -> Option<Translator> {
        self.translations_manager.get_translator()
    }
    /// Gets the currently active translator.
    ///
    /// On the browser-side, this will return `None` under some error
    /// conditions, or before the initial load.
    ///
    /// On the engine-side, this will return `None` under certain error
    /// conditions.
    #[cfg(engine)]
    pub fn try_get_translator(&self) -> Option<Translator> {
        self.translator.clone()
    }
    /// Gets the currently active translator. Under some conditions, this will
    /// panic: `.try_get_translator()` is available as a non-panicking
    /// alternative.
    ///
    /// # Panics
    /// Panics if used before the initial load on the browser, when there isn't
    /// a translator yet, or if used on the engine-side when a translator is
    /// not available (which will be inside certain error views). Note that
    /// an engine-side panic would occur as the server is serving a request,
    /// which will lead to the request not being fulfilled.
    pub fn get_translator(&self) -> Translator {
        self.try_get_translator().expect("translator not available")
    }
}

#[cfg(engine)]
impl<G: Html> Reactor<G> {
    /// Initializes a new [`Reactor`] on the engine-side.
    pub(crate) fn engine(
        global_state: TemplateState,
        mode: RenderMode<G>,
        translator: Option<&Translator>,
    ) -> Self {
        Self {
            state_store: PageStateStore::new(0), /* There will be no need for the state store on
                                                  * the
                                                  * server-side (but is still has to be
                                                  * accessible) */
            global_state: if !global_state.is_empty() {
                GlobalState::new(GlobalStateType::Server(global_state))
            } else {
                GlobalState::new(GlobalStateType::None)
            },
            render_mode: mode,
            translator: translator.cloned(),
        }
    }
}

/// The possible states a window variable injected by the server/export process
/// can be found in.
#[cfg(client)]
pub(crate) enum WindowVariable<T: Serialize + DeserializeOwned> {
    /// It existed and coudl be deserialized into the correct type.
    Some(T),
    /// It was not present.
    None,
    /// It could not be deserialized into the correct type, or it was not
    /// instantiated as the correct serialized type (e.g. expected to find a
    /// string to be deserialized, found a boolean instead).
    Malformed,
}
#[cfg(client)]
impl<T: Serialize + DeserializeOwned> WindowVariable<T> {
    /// Gets the window variable of the given name, attempting to fetch it as
    /// the given type. This will only work with window variables that have
    /// been serialized to strings from the given type `T`.
    fn new_obj(name: &str) -> Self {
        let val_opt = web_sys::window().unwrap().get(name);
        let js_obj = match val_opt {
            Some(js_obj) => js_obj,
            None => return Self::None,
        };
        // The object should only actually contain the string value that was injected
        let val_str = match js_obj.as_string() {
            Some(val_str) => val_str,
            None => return Self::Malformed,
        };
        let val_typed = match serde_json::from_str::<T>(&val_str) {
            Ok(typed) => typed,
            Err(_) => return Self::Malformed,
        };

        Self::Some(val_typed)
    }
}
#[cfg(client)]
impl WindowVariable<bool> {
    /// Gets the window variable of the given name, attempting to fetch it as
    /// the given type. This will only work with boolean window variables.
    ///
    /// While it may seem that a boolean cannot be 'malformed', it most
    /// certainly can be if you think it is boolean, but it actually isn't!
    ///
    /// This is generally used internally for managing flags.
    pub(crate) fn new_bool(name: &str) -> Self {
        let val_opt = web_sys::window().unwrap().get(name);
        let js_bool = match val_opt {
            Some(js_bool) => js_bool,
            None => return Self::None,
        };
        // The object should only actually contain the boolean value that was injected
        match js_bool.as_bool() {
            Some(val) => Self::Some(val),
            None => Self::Malformed,
        }
    }
}
#[cfg(client)]
impl WindowVariable<String> {
    /// Gets the window variable of the given name, attempting to fetch it as
    /// the given type. This will only work with `String` window variables.
    fn new_str(name: &str) -> Self {
        let val_opt = web_sys::window().unwrap().get(name);
        let js_str = match val_opt {
            Some(js_str) => js_str,
            None => return Self::None,
        };
        // The object should only actually contain the boolean value that was injected
        match js_str.as_string() {
            Some(val) => Self::Some(val),
            None => Self::Malformed,
        }
    }
}
