use super::global_state::FrozenGlobalState;
use crate::path::{PathMaybeWithLocale, PathWithoutLocale};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A representation of a frozen app.
///
/// This is only `Clone` for fault tolerance. Do NOT ever clone this unless you
/// seriously know what you're doing!
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrozenApp {
    /// The frozen global state. This will serialize to a `TemplateState`,
    /// unless it is set to
    pub global_state: FrozenGlobalState,
    /// The frozen route. This will be `None` if the app hadn't properly
    /// hydrated when it was frozen.
    pub route: Option<PathMaybeWithLocale>,
    /// The frozen state store. We store this as a `HashMap` as this level
    /// so that we can avoid another deserialization.
    ///
    /// Note that this only contains the active state store, preloads are *not*
    /// preserved to save space (and because they can always be
    /// re-instantiated )
    pub state_store: HashMap<PathMaybeWithLocale, String>,
}

/// The user's preferences on state thawing.
#[derive(Debug, Clone)]
pub struct ThawPrefs {
    /// The preference for page thawing.
    pub page: PageThawPrefs,
    /// Whether or not active global state should be overridden by frozen state.
    pub global_prefer_frozen: bool,
}

/// The user's preferences on page state thawing. Templates have three places
/// they can fetch state from: the page state store (called *active* state), the
/// frozen state, and the server. They're typically prioritized in that order,
/// but if thawing occurs later in an app, it may be desirable to override
/// active state in favor of frozen state. These preferences allow setting an
/// inclusion or exclusion list.
///
/// In apps using internationalization, locales should not be provided here,
/// they will be inferred.
#[derive(Debug, Clone)]
pub enum PageThawPrefs {
    /// Include the attached pages by their URLs (with no leading `/`). Pages
    /// listed here will prioritize frozen state over active state, allowing
    /// thawing to override the current state of the app.
    Include(Vec<String>),
    /// Includes all pages in the app, making frozen state always override state
    /// that's already been initialized.
    IncludeAll,
    /// Excludes the attached pages by their URLs (with no leading `/`). Pages
    /// listed here will prioritize active state over frozen state as usual, and
    /// any pages not listed here will prioritize frozen state.
    /// `Exclude(Vec::new())` is equivalent to `IncludeAll`.
    Exclude(Vec<String>),
}
impl PageThawPrefs {
    /// Checks whether or not the given URL should prioritize frozen state over
    /// active state.
    pub(crate) fn should_prefer_frozen_state(&self, url: &PathWithoutLocale) -> bool {
        match &self {
            // If we're only including some pages, this page should be on the include list
            Self::Include(pages) => pages.iter().any(|v| v == &**url),
            // If we're including all pages in frozen state prioritization, then of course this
            // should use frozen state
            Self::IncludeAll => true,
            // If we're excluding some pages, this page shouldn't be on the exclude list
            Self::Exclude(pages) => !pages.iter().any(|v| v == &**url),
        }
    }
}
