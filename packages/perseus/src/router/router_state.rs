use super::RouteVerdict;
use crate::templates::TemplateNodeType;
use crate::utils::provide_context_signal_replace;
use std::rc::Rc;
use sycamore::prelude::{use_context, Scope, Signal};

#[derive(Debug)]
pub struct LastVerdict(pub Option<RouteVerdict<TemplateNodeType>>);
#[derive(Debug)]
pub struct ReloadCommander(bool);

/// The state for the router. This makes use of `Signal`s internally, and can be cheaply cloned.
#[derive(Debug, Clone)]
pub struct RouterState<'a> {
    /// The router's current load state.
    load_state: &'a Signal<RouterLoadState>,
    /// The last route verdict. We can come back to this if we need to reload the current page without losing context etc.
    last_verdict: &'a Signal<LastVerdict>,
    /// A flip-flop `Signal`. Whenever this is changed, the router will reload the current page in the SPA style. As a user,
    /// you should rarely ever need to do this, but it's used internally in the thawing process.
    pub(crate) reload_commander: &'a Signal<ReloadCommander>,
}
impl<'a> RouterState<'a> {
    /// Creates a default instance of the router state intended for server-side usage. This will allocate its properties to the given scope's context and then mirror them.
    pub fn new(cx: Scope<'a>) -> Self {
        let load_state = provide_context_signal_replace(cx, RouterLoadState::Server);
        let last_verdict = provide_context_signal_replace(cx, LastVerdict(None));
        // It doesn't matter what we initialize this as, it's just for signalling
        let reload_commander = provide_context_signal_replace(cx, ReloadCommander(true));

        Self {
            load_state,
            last_verdict,
            reload_commander,
        }
    }
    /// Creates a new instance of the router state from the context of the given reactive scope. If the required types do not exist in the given scope, this will panic.
    pub fn from_ctx(cx: Scope<'a>) -> Self {
        Self {
            load_state: use_context(cx),
            last_verdict: use_context(cx),
            reload_commander: use_context(cx),
        }
    }
    /// Gets the load state of the router. You'll still need to call `.get()` after this (this just returns an `RcSignal` to derive other state from in a `create_memo` or the like).
    pub fn get_load_state(&self) -> &'a Signal<RouterLoadState> {
        self.load_state
    }
    /// Sets the load state of the router.
    pub fn set_load_state(&self, new: RouterLoadState) {
        self.load_state.set(new);
    }
    /// Gets the last verdict.
    pub fn get_last_verdict(&self) -> Rc<LastVerdict> {
        self.last_verdict.get()
    }
    /// Sets the last verdict.
    pub fn set_last_verdict(&mut self, new: RouteVerdict<TemplateNodeType>) {
        self.last_verdict.set(LastVerdict(Some(new)));
    }
    /// Orders the router to reload the current page as if you'd called `navigate()` to it (but that would do nothing). This
    /// enables reloading in an SPA style (but you should almost never need it).
    ///
    /// Warning: if you're trying to reset your app, do NOT use this! Instead, reload the page fully through `web_sys`.
    pub fn reload(&self) {
        self.reload_commander
            .set(ReloadCommander(!self.reload_commander.get_untracked().0))
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
