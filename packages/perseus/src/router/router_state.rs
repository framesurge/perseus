use sycamore::prelude::{ReadSignal, Signal};

/// The state for the router.
#[derive(Clone, Debug)]
pub struct RouterState {
    /// The router's current load state.
    load_state: Signal<RouterLoadState>,
}
impl Default for RouterState {
    /// Creates a default instance of the router state intended for server-side usage.
    fn default() -> Self {
        Self {
            load_state: Signal::new(RouterLoadState::Server),
        }
    }
}
impl RouterState {
    /// Gets the load state of the router. You'll still need to call `.get()` after this (this just returns a `ReadSignal` to derive other state from in a `create_memo` or the like).
    pub fn get_load_state(&self) -> ReadSignal<RouterLoadState> {
        self.load_state.handle()
    }
    /// Sets the load state of the router.
    pub fn set_load_state(&self, new: RouterLoadState) {
        self.load_state.set(new);
    }
}

/// The current load state of the router. You can use this to be warned of when a new page is about to be loaded (and display a loading bar or the like, perhaps).
#[derive(Clone, Debug)]
pub enum RouterLoadState {
    /// The page has been loaded.
    Loaded {
        /// The name of the template being loaded (mostly for convenience).
        template_name: String,
        /// The full path to the new page being loaded (including the locale, if we're using i18n).
        path: String,
    },
    /// A new page is being loaded, and will soon replace whatever is currently loaded. The name of the new template is attached.
    Loading {
        /// The name of the template being loaded (mostly for convenience).
        template_name: String,
        /// The full path to the new page being loaded (including the locale, if we're using i18n).
        path: String,
    },
    /// We're on the server, and there is no router. Whatever you render based on this state will appear when the user first loads the page, before it's made interactive.
    Server,
}
