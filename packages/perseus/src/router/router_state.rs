use super::RouteVerdict;
use crate::{PathMaybeWithLocale, PathWithLocale, router::RouteInfo, template::TemplateNodeType};
use std::cell::RefCell;
use std::rc::Rc;
use sycamore::prelude::{create_rc_signal, create_ref, RcSignal, Scope};

/// The state for the router. This makes use of `RcSignal`s internally, and can
/// be cheaply cloned.
#[derive(Debug, Clone)]
pub struct RouterState {
    /// The router's current load state. This is in an `RcSignal` because users
    /// need to be able to create derived state from it.
    load_state: RcSignal<RouterLoadState>,
    /// The last route verdict. We can come back to this if we need to reload
    /// the current page without losing context etc.
    last_verdict: Rc<RefCell<Option<RouteVerdict<TemplateNodeType>>>>,
    /// A flip-flop `RcSignal`. Whenever this is changed, the router will reload
    /// the current page in the SPA style (maintaining state). As a user, you
    /// should rarely ever need to do this, but it's used internally in the
    /// thawing process.
    pub(crate) reload_commander: RcSignal<bool>,
}
impl Default for RouterState {
    /// Creates a default instance of the router state intended for server-side
    /// usage.
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
    /// Gets the load state of the router. You'll still need to call `.get()`
    /// after this (this just returns a `&'a RcSignal` to derive other state
    /// from in a `create_memo` or the like).
    pub fn get_load_state<'a>(&self, cx: Scope<'a>) -> &'a RcSignal<RouterLoadState> {
        create_ref(cx, self.load_state.clone())
    }
    /// Gets the load state of the router. You'll still need to call `.get()`
    /// after this (this just returns a `RcSignal` to derive other state from in
    /// a `create_memo` or the like).
    ///
    /// This is designed for internal use only. End users should get a reference
    /// with `.get_load_state()`.
    pub(crate) fn get_load_state_rc(&self) -> RcSignal<RouterLoadState> {
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
    pub fn set_last_verdict(&self, new: RouteVerdict<TemplateNodeType>) {
        let mut last_verdict = self.last_verdict.borrow_mut();
        *last_verdict = Some(new);
    }
    /// Orders the router to reload the current page as if you'd called
    /// `navigate()` to it (but that would do nothing). This
    /// enables reloading in an SPA style (but you should almost never need it).
    ///
    /// Warning: if you're trying to rest your app, do NOT use this! Instead,
    /// reload the page fully through `web_sys`.
    pub fn reload(&self) {
        self.reload_commander
            .set(!*self.reload_commander.get_untracked())
    }
    /// Gets the current path within the app, including the locale if the app is using i18n.
    /// This will not have a leading/trailing forward slash.
    ///
    /// If you're executing this from within a page, it will always be `Some(..)`.
    /// `None` will be returned if no page has been rendered yet (if you managed
    /// to call this from a plugin...), or, more likely, if an error occurred (i.e.
    /// this will probably be `None` in error pages, which are given the path anyway),
    /// or if we're diverting to a localized version of the current path (in which case
    /// your code should not be running).
    pub fn get_path(&self) -> Option<PathMaybeWithLocale> {
        let verdict = self.last_verdict.borrow();
        if let Some(RouteVerdict::Found(RouteInfo { path, locale, .. })) = &*verdict {
            Some(PathMaybeWithLocale::new(path, locale))
        } else {
            None
        }
    }
}

/// The current load state of the router. You can use this to be warned of when
/// a new page is about to be loaded (and display a loading bar or the like,
/// perhaps).
#[derive(Clone, Debug)]
pub enum RouterLoadState {
    /// The page has been loaded.
    Loaded {
        /// The name of the template being loaded (mostly for convenience).
        template_name: String,
        /// The full path to the new page being loaded (including the locale, if
        /// we're using i18n).
        path: String,
    },
    /// An error page has been loaded.
    ErrorLoaded {
        /// The full path to the page we intended to load, on which the error
        /// occurred (including the locale, if we're using i18n).
        path: String,
    },
    /// A new page is being loaded, and will soon replace whatever is currently
    /// loaded. The name of the new template is attached.
    Loading {
        /// The name of the template being loaded (mostly for convenience).
        template_name: String,
        /// The full path to the new page being loaded (including the locale, if
        /// we're using i18n).
        path: String,
    },
    /// We're on the server, and there is no router. Whatever you render based
    /// on this state will appear when the user first loads the page, before
    /// it's made interactive.
    Server,
}
