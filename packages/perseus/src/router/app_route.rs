use super::{match_route, RouteVerdict};
use crate::{
    i18n::Locales,
    template::{RenderCtx, TemplateMap, TemplateNodeType},
};
use std::collections::HashMap;
use std::rc::Rc;
use sycamore::prelude::Scope;
use sycamore_router::Route;

/// The Perseus route system, which implements Sycamore `Route`, but adds
/// additional data for Perseus' processing system.
pub(crate) struct PerseusRoute<'cx> {
    /// The current route verdict. The initialization value of this is
    /// completely irrelevant (it will be overridden immediately by the internal
    /// routing logic).
    pub verdict: RouteVerdict<TemplateNodeType>,
    /// The Sycamore scope that allows us to access the render context.
    ///
    /// This will *always* be `Some(_)` in actual applications.
    pub cx: Option<Scope<'cx>>,
}
// Sycamore would only use this if we were processing dynamic routes, which
// we're not
// In other words, it's fine that these values would break everything
// if they were used, they're just compiler appeasement
impl<'cx> Default for PerseusRoute<'cx> {
    fn default() -> Self {
        Self {
            verdict: RouteVerdict::NotFound,
            // Again, this will never be accessed
            cx: None,
        }
    }
}
impl<'cx> PerseusRoute<'cx> {
    /// Gets the current route verdict.
    pub fn get_verdict(&self) -> &RouteVerdict<TemplateNodeType> {
        &self.verdict
    }
}
impl<'cx> Route for PerseusRoute<'cx> {
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

        let render_ctx = RenderCtx::from_ctx(self.cx.unwrap()); // We know the scope will always exist
        let verdict = match_route(
            &path_segments,
            &render_ctx.render_cfg,
            &render_ctx.templates,
            &render_ctx.locales,
        );
        Self {
            verdict,
            cx: self.cx,
        }
    }
}
