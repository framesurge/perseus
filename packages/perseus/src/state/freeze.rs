use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A representation of a frozen app.
///
/// This is only `Clone` for fault tolerance. Do NOT ever clone this unless you
/// seriously know what you're doing!
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrozenApp {
    /// The frozen global state. If it was never initialized, this will be
    /// `None`.
    pub global_state: String,
    /// The frozen route.
    pub route: String,
    /// The frozen page state store. We store this as a `HashMap` as this level
    /// so that we can avoid another deserialization.
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

/// The user's preferences on page state thawing. Templates have three places
/// they can fetch state from: the page state store (called *active* state), the
/// frozen state, and the server. They're typically prioritized in that order,
/// but if thawing occurs later in an app, it may be desirable to override
/// active state in favor of frozen state. These preferences allow setting an
/// inclusion or exclusion list.
#[derive(Debug, Clone)]
pub enum PageThawPrefs {
    /// Include the attached pages by their URLs (with no leading `/`). Pages
    /// listed here will prioritize frozen state over active state, allowing
    /// thawing to override the current state of the app.
    Include(Vec<String>),
    /// Includes all pages in the app, making frozen state always override state
    /// that's already been initialized.
    IncludeAll,
    /// Exludes the attached pages by their URLs (with no leading `/`). Pages
    /// listed here will prioritize active state over frozen state as usual, and
    /// any pages not listed here will prioritize frozen state.
    /// `Exclude(Vec::new())` is equivalent to `IncludeAll`.
    Exclude(Vec<String>),
}
impl PageThawPrefs {
    /// Checks whether or not the given URl should prioritize frozen state over
    /// active state.
    pub fn should_use_frozen_state(&self, url: &str) -> bool {
        match &self {
            // If we're only including some pages, this page should be on the include list
            Self::Include(pages) => pages.iter().any(|v| v == url),
            // If we're including all pages in frozen state prioritization, then of course this
            // should use frozen state
            Self::IncludeAll => true,
            // If we're excluding some pages, this page shouldn't be on the exclude list
            Self::Exclude(pages) => !pages.iter().any(|v| v == url),
        }
    }
}
