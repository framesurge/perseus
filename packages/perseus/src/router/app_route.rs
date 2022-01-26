use crate::Html;
use sycamore_router::Route;

use super::RouteVerdict;

/// Creates an app-specific routing `struct`. Sycamore expects an `enum` to do this, so we create a `struct` that behaves similarly. If
/// we don't do this, we can't get the information necessary for routing into the `enum` at all (context and global variables don't suit
/// this particular case).
#[macro_export]
macro_rules! create_app_route {
    {
        name => $name:ident,
        render_cfg => $render_cfg:expr,
        templates => $templates:expr,
        locales => $locales:expr
    } => {
        /// The route type for the app, with all routing logic inbuilt through the generation macro.
        struct $name<G: $crate::Html>($crate::internal::router::RouteVerdict<G>);
        impl<G: $crate::Html> $crate::internal::router::PerseusRoute<G> for $name<G> {
            fn get_verdict(&self) -> &$crate::internal::router::RouteVerdict<G> {
                &self.0
            }
        }
        impl<G: $crate::Html> ::sycamore_router::Route for $name<G> {
            fn match_route(path: &[&str]) -> Self {
                let verdict = $crate::internal::router::match_route(path, $render_cfg, $templates, $locales);
                Self(verdict)
            }
        }
    };
}

/// A trait for the routes in Perseus apps. This should be used almost exclusively internally, and you should never need to touch
/// it unless you're building a custom engine.
pub trait PerseusRoute<G: Html>: Route {
    /// Gets the route verdict for the current route.
    fn get_verdict(&self) -> &RouteVerdict<G>;
}
