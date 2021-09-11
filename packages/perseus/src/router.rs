use crate::Locales;
use crate::Template;
use std::rc::Rc;
use sycamore::prelude::GenericNode;
use sycamore::rx::use_context;
use sycamore_router::{Route, RoutePath, Segment};

/// A representation of routes in a Perseus app. This is used internally to match routes. Because this can't be passed directly to
/// the `RouteVerdict`'s `match_route` function, it should be provided in context instead (through an `Rc<T>`).
pub struct Routes<G: GenericNode> {
    /// The routes in the app, stored as an *ordered* list of key-value pairs, mapping routing path (e.g. `/post/<slug..>`) to template.
    /// These will be matched by a loop, so more specific routes should go first in the vector. Even if we're using i18n, this still
    /// stores a routing path without the locale, which is added in during parsing as necessary.
    routes: Vec<(Vec<Segment>, Template<G>)>,
    /// Whether or not the user is using i18n, which significantly impacts how we match routes (will there be a locale in front of
    /// everything).
    locales: Locales,
}
impl<G: GenericNode> Routes<G> {
    /// Creates a new instance of the routes. This takes a vector of key-value pairs of routing path to template functions.
    pub fn new(raw_routes: Vec<(String, Template<G>)>, locales: Locales) -> Self {
        let routes: Vec<(Vec<Segment>, Template<G>)> = raw_routes
            .iter()
            .map(|(router_path_str_raw, template_fn)| {
                // Handle the landing page (because match systems don't tolerate empty strings well)
                if router_path_str_raw == "/" {
                    return (Vec::new(), template_fn.clone());
                }

                // Remove leading/trailing `/`s to avoid empty elements (which stuff up path matching)
                let mut router_path_str = router_path_str_raw.clone();
                if router_path_str.starts_with('/') {
                    router_path_str.remove(0);
                }
                if router_path_str.ends_with('/') {
                    router_path_str.remove(router_path_str.len() - 1);
                }

                let router_path_parts = router_path_str.split('/');
                let router_path: Vec<Segment> = router_path_parts
                    .map(|part| {
                        // TODO possibly use Actix Web like syntax here instead and propose to @lukechu10?
                        // We need to create a segment out of this part, we'll parse Sycamore's syntax
                        // We don't actually need Regex here, so we don't bloat with it
                        // If you're familiar with Sycamore's routing system, we don't need to worry about capturing these segments in Perseus because we just return the actual path directly
                        /* Variants (in tested order):
                            - <stuff..>     segment that captures many parameters
                            - <stuff>       parameter that captures a single element
                            - stuff         verbatim stuff
                        */
                        if part.starts_with('<') && part.ends_with("..>") {
                            Segment::DynSegments
                        } else if part.starts_with('<') && part.ends_with('>') {
                            Segment::DynParam
                        } else {
                            Segment::Param(part.to_string())
                        }
                    })
                    .collect();
                // Turn the router path into a vector of `Segment`s
                (router_path, template_fn.clone())
            })
            .collect();

        Self { routes, locales }
    }
    /// Matches the given route to an instance of `RouteVerdict`.
    pub fn match_route(&self, raw_path: &[&str]) -> RouteVerdict<G> {
        let path: Vec<&str> = raw_path.to_vec();
        let path_joined = path.join("/"); // This should not have a leading forward slash, it's used for asset fetching by the app shell

        let mut verdict = RouteVerdict::NotFound;
        // There are different logic chains if we're using i18n, so we fork out early
        if self.locales.using_i18n {
            for (segments, template_fn) in &self.routes {
                let route_path_without_locale = RoutePath::new(segments.to_vec());
                let route_path_with_locale = RoutePath::new({
                    let mut vec = vec![Segment::DynParam];
                    vec.extend(segments.to_vec());
                    vec
                });

                // First, we'll see if the path matches a translated route
                // If that fails, we'll see if it matches an untranslated route, which becomes a locale detector
                if route_path_with_locale.match_path(&path).is_some() {
                    verdict = RouteVerdict::Found(RouteInfo {
                        // The asset fetching process deals with the locale separately, and doesn't need a leading `/`
                        path: path[1..].to_vec().join("/"),
                        template_fn: template_fn.clone(),
                        locale: path[0].to_string(),
                    });
                    break;
                } else if route_path_without_locale.match_path(&path).is_some() {
                    // We've now matched that it fits without the locale, which means the user is trying to
                    verdict = RouteVerdict::LocaleDetection(path_joined);
                    break;
                }
            }
        } else {
            for (segments, template_fn) in &self.routes {
                let route_path = RoutePath::new(segments.to_vec());

                // We're not using i18n, so we can just match the path directly
                if route_path.match_path(&path).is_some() {
                    verdict = RouteVerdict::Found(RouteInfo {
                        path: path_joined,
                        template_fn: template_fn.clone(),
                        // Every page uses the default locale if we aren't using i18n (translators won't be used anyway)
                        locale: self.locales.default.to_string(),
                    });
                    break;
                }
            }
        }

        verdict
    }
}

/// Information about a route, which, combined with error pages and a client-side translations manager, allows the initialization of
/// the app shell and the rendering of a page.
pub struct RouteInfo<G: GenericNode> {
    /// The actual path of the route.
    pub path: String,
    /// The template that will render the template. The app shell will derive pros and a translator to pass to the template function.
    pub template_fn: Template<G>,
    /// The locale for the template to be rendered in.
    pub locale: String,
}

/// The possible outcomes of matching a route. This is an alternative implementation of Sycamore's `Route` trait to enable greater
/// control and tighter integration of routing with templates. This can only be used if `Routes` has been defined in context (done
/// automatically by the CLI).
pub enum RouteVerdict<G: GenericNode> {
    /// The given route was found, and route information is attached.
    Found(RouteInfo<G>),
    /// The given route was not found, and a `404 Not Found` page should be shown.
    NotFound,
    /// The given route maps to the locale detector, which will redirect the user to the attached path (in the appropriate locale).
    LocaleDetection(String),
}
impl<G: GenericNode> Route for RouteVerdict<G> {
    fn match_route(path: &[&str]) -> Self {
        // Get an instance of `Routes` by context
        let routes = use_context::<Rc<Routes<G>>>();
        // Match the path using that
        routes.match_route(path)
    }
}
