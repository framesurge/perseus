use super::{match_route, RouteVerdict};
use crate::{i18n::Locales, templates::TemplateMap, Html};
use std::collections::HashMap;
use sycamore_router::Route;

// /// Creates an app-specific routing `struct`. Sycamore expects an `enum` to do this, so we create a `struct` that behaves similarly. If
// /// we don't do this, we can't get the information necessary for routing into the `enum` at all (context and global variables don't suit
// /// this particular case).
// #[macro_export]
// macro_rules! create_app_route {
//     {
//         name => $name:ident,
//         render_cfg => $render_cfg:expr,
//         templates => $templates:expr,
//         locales => $locales:expr
//     } => {
//         /// The route type for the app, with all routing logic inbuilt through the generation macro.
//         #[derive(::std::clone::Clone)]
//         struct $name<G: $crate::Html>($crate::internal::router::RouteVerdict<G>);
//         impl<G: $crate::Html> $crate::internal::router::PerseusRoute<G> for $name<G> {
//             fn get_verdict(&self) -> &$crate::internal::router::RouteVerdict<G> {
//                 &self.0
//             }
//         }
//         impl<G: $crate::Html> ::sycamore_router::Route for $name<G> {
//             fn match_route(path: &[&str]) -> Self {
//                 let verdict = $crate::internal::router::match_route(path, $render_cfg, $templates, $locales);
//                 Self(verdict)
//             }
//         }
//     };
// }

/// The Perseus route system, which implements Sycamore `Route`, but adds additional data for Perseus' processing system.
pub struct PerseusRoute<G: Html> {
    verdict: RouteVerdict<G>,
    render_cfg: HashMap<String, String>,
    templates: TemplateMap<G>,
    locales: Locales,
}
// Sycamore would only use this if we were processing dynamic routes, which we're not
// In other words, it's fine that these values would break everything if they were used, they're just compiler appeasement
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
