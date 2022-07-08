use super::{match_route, RouteVerdict};
use crate::{i18n::Locales, template::TemplateMap, Html};
use std::collections::HashMap;
use sycamore_router::Route;

/// The Perseus route system, which implements Sycamore `Route`, but adds
/// additional data for Perseus' processing system.
pub(crate) struct PerseusRoute<G: Html> {
    /// The current route verdict. The initialization value of this is
    /// completely irrelevant (it will be overriden immediately by the internal
    /// routing logic).
    pub verdict: RouteVerdict<G>,
    /// The app's render configuration.
    pub render_cfg: HashMap<String, String>,
    /// The templates the app is using.
    pub templates: TemplateMap<G>,
    /// The app's i18n configuration.
    pub locales: Locales,
}
// Sycamore would only use this if we were processing dynamic routes, which
// we're not In other words, it's fine that these values would break everything
// if they were used, they're just compiler appeasement
impl<G: Html> Default for PerseusRoute<G> {
    fn default() -> Self {
        Self {
            verdict: RouteVerdict::NotFound,
            render_cfg: HashMap::default(),
            templates: TemplateMap::default(),
            locales: Locales {
                default: String::default(),
                other: Vec::default(),
                using_i18n: bool::default(),
            },
        }
    }
}
impl<G: Html> PerseusRoute<G> {
    /// Gets the current route verdict.
    pub fn get_verdict(&self) -> &RouteVerdict<G> {
        &self.verdict
    }
}
impl<G: Html> Route for PerseusRoute<G> {
    fn match_route(&self, path: &[&str]) -> Self {
        let verdict = match_route(path, &self.render_cfg, &self.templates, &self.locales);
        Self {
            verdict,
            render_cfg: self.render_cfg.clone(),
            templates: self.templates.clone(),
            locales: self.locales.clone(),
        }
    }
}
