#[cfg(target_arch = "wasm32")]
mod start;
#[cfg(target_arch = "wasm32")]
mod error;
mod state;
mod global_state;
#[cfg(not(target_arch = "wasm32"))]
mod render_mode;
#[cfg(target_arch = "wasm32")]
mod initial_load;
#[cfg(target_arch = "wasm32")]
mod widget_disposers;

pub(crate) use render_mode::{RenderMode, RenderStatus};
#[cfg(target_arch = "wasm32")]
pub(crate) use initial_load::InitialView;

// --- Common imports ---
use std::{cell::{Cell, RefCell}, collections::HashMap, rc::Rc};
use sycamore::{prelude::{Scope, provide_context, use_context}, web::Html};
use crate::{router::RouterState, state::{GlobalState, GlobalStateType, PageStateStore, TemplateState}};

// --- Engine-side imports ---
#[cfg(not(target_arch = "wasm32"))]
use crate::i18n::Translator;

// --- Browser-side imports ---
#[cfg(target_arch = "wasm32")]
use crate::{
    i18n::{ClientTranslationsManager, Locales, TranslationsManager},
    router::PageDisposer,
    error_views::ErrorViews,
    init::PerseusAppBase,
    errors::{ClientInvariantError, PluginError},
    plugins::Plugins,
    state::{FrozenApp, ThawPrefs},
    stores::MutableStore,
    template::TemplateMap,
};
#[cfg(target_arch = "wasm32")]
use self::widget_disposers::WidgetDisposers;
#[cfg(target_arch = "wasm32")]
use sycamore::reactive::{RcSignal, create_rc_signal};
#[cfg(target_arch = "wasm32")]
use serde::{Serialize, de::DeserializeOwned};
#[cfg(target_arch = "wasm32")]
use serde_json::Value;

/// The core of Perseus' browser-side systems. This forms a central point for all the Perseus state and rendering logic
/// to operate from. In your own code, this will always be available in the Sycamore context system.
///
/// Note that this is also used on the engine-side for rendering.
pub struct Reactor<G: Html> {
    /// The state store, which is used to hold all reactive states, along with preloads.
    state_store: PageStateStore,
    /// The router state.
    router_state: RouterState,
    /// The user-provided global state, stored with similar mechanics to the state store,
    /// although optimised.
    global_state: GlobalState,

    // --- Browser-side only ---
    /// A previous state the app was once in, still serialized. This will be
    /// rehydrated gradually by the template closures.
    ///
    /// The `bool` in here will be set to `true` if this was created through HSR,
    /// which has slightly more lenient thawing procedures to allow for data model
    /// changes.
    #[cfg(target_arch = "wasm32")]
    frozen_app: Rc<RefCell<Option<(FrozenApp, ThawPrefs, bool)>>>,
    /// Whether or not this page is the very first to have been rendered since
    /// the browser loaded the app. This will be reset on full reloads, and is
    /// used internally to determine whether or not we should look for
    /// stored HSR state.
    #[cfg(target_arch = "wasm32")]
    pub(crate) is_first: Cell<bool>,
    /// The app's *full* render configuration. Note that a subset of this
    /// is contained in the [`RenderMode`] on the engine-side for widget
    /// rendering.
    #[cfg(target_arch = "wasm32")]
    render_cfg: HashMap<String, String>,
    /// The app's templates for use in routing.
    #[cfg(target_arch = "wasm32")]
    templates: TemplateMap<G>,
    /// The app's locales.
    #[cfg(target_arch = "wasm32")]
    locales: Locales,
    /// The app's plugins.
    #[cfg(target_arch = "wasm32")]
    plugins: Rc<Plugins>,
    /// The scope disposers for widgets rendered for the current page. These will all be disposed of
    /// simultaneously.
    ///
    /// On the engine-side, scopes are simply ignored, because they're all ephemeral, so we don't
    /// need this.
    #[cfg(target_arch = "wasm32")]
    current_widget_disposers: WidgetDisposers,
    /// The scope disposers for widgets rendered for the next page. These will all be disposed of
    /// simultaneously. We need this to be separate because, during the render of the next page,
    /// durng which these will be accumulated, the current page has not yet been disposed of,
    /// so adding the next disposers to that old list would lead to their all being disposed of
    /// when we switched formally to the new page!
    ///
    /// On the engine-side, scopes are simply ignored, because they're all ephemeral, so we don't
    /// need this.
    #[cfg(target_arch = "wasm32")]
    next_widget_disposers: WidgetDisposers,
    /// The browser-side translations manager.
    #[cfg(target_arch = "wasm32")]
    translations_manager: ClientTranslationsManager,
    /// The app's error views.
    #[cfg(target_arch = "wasm32")]
    error_views: ErrorViews<G>,
    /// A reactive container for the current page-wide view. This will usually contain the contents of
    /// the current page, but it may also contain a page-wide error. This will be wrapped in a router.
    #[cfg(target_arch = "wasm32")]
    current_view: RcSignal<View<G>>,
    /// A reactive container for any popup errors.
    #[cfg(target_arch = "wasm32")]
    popup_error_view: RcSignal<View<G>>,
    /// The disposer for the current page (separate from the widget disposers). This does
    /// not need a distinction between current/next (as the widgets do), because it will instantiated
    /// from code in `Self`.
    #[cfg(target_arch = "wasm32")]
    page_disposer: PageDisposer,
    /// The app's root div ID.
    #[cfg(target_arch = "wasm32")]
    root: String,

    // --- Engine-side only ---
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) render_mode: RenderMode<G>,
    /// The currently active translator. On the browser-side, this is handled by the more fully-fledged
    /// [`ClientTranslationsManager`].
    ///
    /// This is provided to the engine-side reactor on instantiation. This can be `None` in certain
    /// error view renders.
    #[cfg(not(target_arch = "wasm32"))]
    translator: Option<Translator>,
}

// This uses window variables set by the HTML shell, so it should never be used on the engine-side
#[cfg(target_arch = "wasm32")]
impl<G: Html, M: MutableStore, T: TranslationsManager> TryFrom<PerseusAppBase<G, M, T>> for Reactor<G> {
    fn try_from(app: PerseusAppBase<G, M, T>) -> Result<Self, ClientError> {
        let pss_max_size = app.get_pss_max_size();
        let templates = app.get_templates_map();
        let locales = app.get_locales()?;
        let plugins = app.get_plugins();
        let error_views = app.get_error_views();
        let root = app.get_root()?;

        plugins
            .functional_actions
            .client_actions
            .start
            .run((), plugins.get_plugin_data())?;

        // We need to fetch some things from window variables
        let render_cfg = match WindowVariable::<HashMap<String, String>>::new_obj("__PERSEUS_RENDER_CFG") {
            WindowVariable::Some(render_cfg) => render_cfg,
            WindowVariable::None | WindowVariable::Malformed => return Err(ClientInvariantError::RenderCfg).into(),
        };
        let global_state = match WindowVariable::<Value>::new_obj("__PERSEUS_GLOBAL_STATE") {
            WindowVariable::Some(val) => {
                let state = TemplateState::from_value(val);
                if state.is_empty() {
                    // TODO Since we have it to hand, just make sure the global state creator really wasn't
                    // going to create anything (otherwise fail immediately)
                    GlobalStateType::None
                } else {
                    GlobalStateType::Server(state)
                }
            },
            WindowVariable::None => GlobalStateType::None,
            WindowVariable::Malformed => err,
        };

        Ok(Self {
            // This instantiates as if for the engine-side, but it will rapidly be changed
            router_state: RouterState::default(),
            state_store: PageStateStore::new(pss_max_size),
            global_state,
            translations_manager: ClientTranslationsManager::new(&locales),
            // This will be filled out by a `.thaw()` call or HSR
            frozen_app: Rc::new(RefCell::new(None)),
            is_first: Cell::new(true),
            current_view: create_rc_signal(View::empty()),
            popup_error_view: create_rc_signal(View::empty()),
            current_widget_disposers: WidgetDisposers::default(),
            // Yes, the initial load puts things in here
            next_widget_disposers: WidgetDisposers::default(),
            page_disposer: PageDisposer::default(),
            templates,
            locales,
            render_cfg,
            plugins,
            error_views,
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
    /// On the browser-side, this will return `None` under some error conditions,
    /// or before the initial load.
    ///
    /// On the engine-side, this will return `None` under certain error conditions.
    #[cfg(target_arch = "wasm32")]
    pub fn try_get_translator(&self) -> Option<Translator> {
        self.translations_manager.get_translator()
    }
    /// Gets the currently active translator.
    ///
    /// On the browser-side, this will return `None` under some error conditions,
    /// or before the initial load.
    ///
    /// On the engine-side, this will return `None` under certain error conditions.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn try_get_translator(&self) -> Option<Translator> {
        self.translator.clone()
    }
    /// Gets the currently active translator. Under some conditions, this will panic:
    /// `.try_get_translator()` is available as a non-panicking alternative.
    ///
    /// # Panics
    /// Panics if used before the initial load on the browser, when there isn't a translator
    /// yet, or if used on the engine-side when a translator is not available (which will be
    /// inside certain error views). Note that an engine-side panic would occur as the server
    /// is serving a request, which will lead to the request not being fulfilled.
    pub fn get_translator(&self) -> Translator {
        self.try_get_translator().expect("translator not available")
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<G: Html> Reactor<G> {
    /// Initializes a new [`Reactor`] on the engine-side.
    pub(crate) fn engine(global_state: TemplateState, mode: RenderMode<G>, translator: Option<&Translator>) -> Self {
        Self {
            router_state: RouterState::default(),
            state_store: PageStateStore::new(0), /* There will be no need for the state store on the
                                                       * server-side (but is still has to be accessible) */
            global_state: if !global_state.is_empty() {
                GlobalState::new(GlobalStateType::Server(global_state))
            } else {
                GlobalState::new(GlobalStateType::None)
            },
            render_mode: mode,
            translator: translator.cloned()
        }
    }
}

/// The possible states a window variable injected by the server/export process can be found in.
#[cfg(target_arch = "wasm32")]
enum WindowVariable<T: Serialize + DeserializeOwned> {
    /// It existed and coudl be deserialized into the correct type.
    Some(T),
    /// It was not present.
    None,
    /// It could not be deserialized into the correct type, or it was not instantiated
    /// as the correct serialized type (e.g. expected to find a string to be deserialized,
    /// found a boolean instead).
    Malformed,
}
#[cfg(target_arch = "wasm32")]
impl<T: Serialize + DeserializeOwned> WindowVariable<T> {
    /// Gets the window variable of the given name, attempting to fetch it as the given type. This
    /// will only work with window variables that have been serialized to strings from the given
    /// type `T`.
    fn new_obj(name: &str) -> Self {
        let val_opt = web_sys::window().unwrap().get(name);
        let js_obj = match val_opt {
            Some(js_obj) => js_obj,
            None => return Self::None,
        };
        // The object should only actually contain the string value that was injected
        let val_str = match js_obj.as_string() {
            Some(cfg_str) => cfg_str,
            None => return Self::Malformed,
        };
        let val_typed = match serde_json::from_str::<T>(&cfg_str) {
            Ok(typed) => typed,
            Err(_) => return Self::Malformed,
        };

        Self::Some(val_typed)
    }
}
#[cfg(target_arch = "wasm32")]
impl WindowVariable<bool> {
    /// Gets the window variable of the given name, attempting to fetch it as the given type. This
    /// will only work with boolean window variables.
    fn new_bool(name: &str) -> Self {
        let val_opt = web_sys::window().unwrap().get(name);
        let js_bool = match val_opt {
            Some(js_bool) => js_bool,
            None => return Self::None,
        };
        // The object should only actually contain the boolean value that was injected
        match js_obj.as_bool() {
            Some(val) => Self::Some(val),
            None => Self::Malformed,
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl WindowVariable<String> {
    /// Gets the window variable of the given name, attempting to fetch it as the given type. This
    /// will only work with `String` window variables.
    fn new_str(name: &str) -> Self {
        let val_opt = web_sys::window().unwrap().get(name);
        let js_bool = match val_opt {
            Some(js_bool) => js_bool,
            None => return Self::None,
        };
        // The object should only actually contain the boolean value that was injected
        match js_obj.as_string() {
            Some(val) => Self::Some(val),
            None => Self::Malformed,
        }
    }
}
