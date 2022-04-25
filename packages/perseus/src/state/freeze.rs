use crate::utils::provide_context_signal_replace;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use sycamore::prelude::{use_context, Scope, Signal};

#[derive(Debug, Clone)]
pub struct FrozenAppStoreState(pub FrozenApp, pub ThawPrefs);

/// A representation of the context store that stores a frozen app and thaw preferences.
#[derive(Debug)]
pub struct FrozenAppStore<'a> {
    state: &'a Signal<Option<FrozenAppStoreState>>,
}
impl<'a> FrozenAppStore<'a> {
    /// Creates a new instance of the frozen app store.
    pub fn new(cx: Scope<'a>) -> Self {
        let state = provide_context_signal_replace(cx, None);

        Self { state }
    }
    /// Creates a new instance of the frozen app store from the context of the given reactive scope. If the required types do not exist in the given scope, this will panic.
    pub fn from_ctx(cx: Scope<'a>) -> Self {
        Self {
            state: use_context(cx),
        }
    }
    /// Gets the inner value.
    pub fn get(&self) -> Rc<Option<FrozenAppStoreState>> {
        self.state.get()
    }
    /// Sets the inner value using a tuple.
    pub fn set_tuple(&self, val: Option<(FrozenApp, ThawPrefs)>) {
        self.state.set(val.map(|(f, t)| FrozenAppStoreState(f, t)))
    }
    /// Sets the inner value using the wrapper type.
    pub fn set(&self, val: Option<FrozenAppStoreState>) {
        self.state.set(val);
    }
    /// Extracts the stored `FrozenAppStoreState`. This should only ever be called if there are no outstanding references to the `Signal`.
    // TODO Check if this is remotely okay to do...
    pub fn take_unwrap(&self) -> Option<FrozenAppStoreState> {
        match Rc::try_unwrap(self.state.take()) {
            Ok(val) => val,
            Err(val_rc) => (*val_rc).clone(),
        }
    }
}

/// A representation of a frozen app.
///
/// This is only `Clone` for fault tolerance. Do NOT ever clone this unless you seriously know what you're doing!
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrozenApp {
    /// The frozen global state. If it was never initialized, this will be `None`.
    pub global_state: String,
    /// The frozen route.
    pub route: String,
    /// The frozen page state store. We store this as a `HashMap` as this level so that we can avoid another deserialization.
    pub page_state_store: HashMap<String, String>,
}

/// The user's preferences on state thawing.
#[derive(Debug, Clone)]
pub struct ThawPrefs {
    /// The preference for page thawing.
    pub page: PageThawPrefs,
    /// Whether or not active global state should be overriden by frozen state.
    pub global_prefer_frozen: bool,
}

/// The user's preferences on page state thawing. Templates have three places they can fetch state from: the page state store (called *active* state), the frozen state, and the server. They're
/// typically prioritized in that order, but if thawing occurs later in an app, it may be desirable to override active state in favor of frozen state. These preferences allow setting an
/// inclusion or exclusion list.
#[derive(Debug, Clone)]
pub enum PageThawPrefs {
    /// Include the attached pages by their URLs (with no leading `/`). Pages listed here will prioritize frozen state over active state, allowing thawing to override the current state of the app.
    Include(Vec<String>),
    /// Includes all pages in the app, making frozen state always override state that's already been initialized.
    IncludeAll,
    /// Exludes the attached pages by their URLs (with no leading `/`). Pages listed here will prioritize active state over frozen state as usual, and any pages not listed here will prioritize
    /// frozen state. `Exclude(Vec::new())` is equivalent to `IncludeAll`.
    Exclude(Vec<String>),
}
impl PageThawPrefs {
    /// Checks whether or not the given URl should prioritize frozen state over active state.
    pub fn should_use_frozen_state(&self, url: &str) -> bool {
        match &self {
            // If we're only including some pages, this page should be on the include list
            Self::Include(pages) => pages.iter().any(|v| v == url),
            // If we're including all pages in frozen state prioritization, then of course this should use frozen state
            Self::IncludeAll => true,
            // If we're excluding some pages, this page shouldn't be on the exclude list
            Self::Exclude(pages) => !pages.iter().any(|v| v == url),
        }
    }
}
