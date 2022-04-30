use super::RouteVerdict;
use crate::templates::TemplateNodeType;
use std::cell::RefCell;
use std::rc::Rc;
use sycamore::prelude::{create_rc_signal, RcSignal};

/// The state for the router. This makes use of `RcSignal`s internally, and can be cheaply cloned.
#[derive(Debug, Clone)]
pub struct RouterState {
    /// The router's current load state. This is in an `RcSignal` because users need to be able to create derived state from it.
    load_state: RcSignal<RouterLoadState>,
    /// The last route verdict. We can come back to this if we need to reload the current page without losing context etc.
    last_verdict: Rc<RefCell<Option<RouteVerdict<TemplateNodeType>>>>,
    /// A flip-flop `RcSignal`. Whenever this is changed, the router will reload the current page in the SPA style. As a user,
    /// you should rarely ever need to do this, but it's used internally in the thawing process.
    pub(crate) reload_commander: RcSignal<bool>,
}
impl Default for RouterState {
    /// Creates a default instance of the router state intended for server-side usage.
    fn default() -> Self {
        Self {
            load_state: create_rc_signal(RouterLoadState::Server),
            last_verdict: Rc::new(RefCell::new(None)),
            // It doesn't matter what we initialize this as, it's just for signalling
            reload_commander: create_rc_signal(true),
        }
    }
}
impl RouterState {
    /// Gets the load state of the router. You'll still need to call `.get()` after this (this just returns a `ReadSignal` to derive other state from in a `create_memo` or the like).
    pub fn get_load_state(&self) -> RcSignal<RouterLoadState> {
        self.load_state.clone() // TODO Better approach than cloning here?
    }
    /// Sets the load state of the router.
    pub fn set_load_state(&self, new: RouterLoadState) {
        self.load_state.set(new);
    }
    /// Gets the last verdict.
    pub fn get_last_verdict(&self) -> Option<RouteVerdict<TemplateNodeType>> {
        (*self.last_verdict.borrow()).clone()
    }
    /// Sets the last verdict.
    pub fn set_last_verdict(&mut self, new: RouteVerdict<TemplateNodeType>) {
        let mut last_verdict = self.last_verdict.borrow_mut();
        *last_verdict = Some(new);
    }
    /// Orders the router to reload the current page as if you'd called `navigate()` to it (but that would do nothing). This
    /// enables reloading in an SPA style (but you should almost never need it).
    ///
    /// Warning: if you're trying to rest your app, do NOT use this! Instead, reload the page fully through `web_sys`.
    pub fn reload(&self) {
        self.reload_commander
            .set(!*self.reload_commander.get_untracked())
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
