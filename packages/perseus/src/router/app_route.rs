use super::match_route;
use super::RouteVerdict;
use crate::reactor::Reactor;

use sycamore_router::Route;

/// The Perseus route system, which implements Sycamore `Route`, but adds
/// additional data for Perseus' processing system.
pub(crate) struct PerseusRoute {
    /// The current route verdict. The initialization value of this is
    /// completely irrelevant (it will be overridden immediately by the internal
    /// routing logic).
    pub verdict: RouteVerdict,
}
// Sycamore would only use this if we were processing dynamic routes, which
// we're not
// In other words, it's fine that these values would break everything
// if they were used, they're just compiler appeasement
impl Default for PerseusRoute {
    fn default() -> Self {
        Self {
            verdict: RouteVerdict::NotFound {
                locale: "xx-XX".to_string(),
            },
        }
    }
}
impl PerseusRoute {
    /// Gets the current route verdict.
    pub fn get_verdict(&self) -> &RouteVerdict {
        &self.verdict
    }
}
impl Route for PerseusRoute {
    fn match_route(&self, path: &[&str]) -> Self {
        // Decode the path (we can't do this in `match_route` because of how it's called
        // by initial view, and we don't want to double-decode!)
        let path = path.join("/");
        let path = js_sys::decode_uri_component(&path)
            .unwrap()
            .as_string()
            .unwrap();
        let path_segments = path
            .split('/')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>(); // This parsing is identical to the Sycamore router's

        let reactor = Reactor::from_cx(); // We know the context will always exist
        let verdict = match_route(
            &path_segments,
            &reactor.render_cfg,
            &reactor.entities,
            &reactor.locales,
        );
        Self { verdict }
    }
}
