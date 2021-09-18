use crate::Locales;
use crate::Template;
use crate::TemplateMap;
use std::collections::HashMap;
use sycamore::prelude::GenericNode;

/// Determines the template to use for the given path by checking against the render configuration. This houses the central routing
/// algorithm of Perseus, which is based fully on the fact that we know about every single page except those rendered with ISR, and we
/// can infer about them based on template root path domains. If that domain system is violated, this routing algorithm will not
/// behave as expected whatsoever (as far as routing goes, it's undefined behaviour)!
pub fn get_template_for_path<'a, G: GenericNode>(
    raw_path: &str,
    render_cfg: &HashMap<String, String>,
    templates: &'a TemplateMap<G>,
) -> Option<&'a Template<G>> {
    let mut path = raw_path;
    // If the path is empty, we're looking for the special `index` page
    if path.is_empty() {
        path = "index";
    }

    // Match the path to one of the templates
    let mut template_name = String::new();
    // We'll try a direct match first
    if let Some(template_root_path) = render_cfg.get(path) {
        template_name = template_root_path.to_string();
    }
    // Next, an ISR match (more complex), which we only want to run if we didn't get an exact match above
    if template_name.is_empty() {
        // We progressively look for more and more specificity of the path, adding each segment
        // That way, we're searching forwards rather than backwards, which is more efficient
        let path_segments: Vec<&str> = path.split('/').collect();
        for (idx, _) in path_segments.iter().enumerate() {
            // Make a path out of this and all the previous segments
            let path_to_try = path_segments[0..(idx + 1)].join("/") + "/*";

            // If we find something, keep going until we don't (maximise specificity)
            if let Some(template_root_path) = render_cfg.get(&path_to_try) {
                template_name = template_root_path.to_string();
            } else {
                break;
            }
        }
    }
    // If we still have nothing, then the page doesn't exist
    if template_name.is_empty() {
        return None;
    }

    // Get the template to use (the `Option<T>` this returns is perfect) if it exists
    templates.get(&template_name)
}

/// Matches the given path to a `RouteVerdict`. This takes a `TemplateMap` to match against, the render configuration to index, and it
/// needs to know if i18n is being used. The path this takes should be raw, it may or may not have a locale, but should be split into
/// segments by `/`, with empty ones having been removed.
pub fn match_route<G: GenericNode>(
    path_slice: &[&str],
    render_cfg: &HashMap<String, String>,
    templates: &TemplateMap<G>,
    locales: &Locales,
) -> RouteVerdict<G> {
    let path_vec: Vec<&str> = path_slice.to_vec();
    let path_joined = path_vec.join("/"); // This should not have a leading forward slash, it's used for asset fetching by the app shell

    let verdict;
    // There are different logic chains if we're using i18n, so we fork out early
    if locales.using_i18n && !path_slice.is_empty() {
        let locale = path_slice[0];
        // Check if the 'locale' is supported (otherwise it may be the first section of an uni18ned route)
        if locales.is_supported(locale) {
            // We'll assume this has already been i18ned (if one of your routes has the same name as a supported locale, ffs)
            let path_without_locale = path_slice[1..].to_vec().join("/");
            // Get the template to use
            let template = get_template_for_path(&path_without_locale, render_cfg, templates);
            verdict = match template {
                Some(template) => RouteVerdict::Found(RouteInfo {
                    locale: locale.to_string(),
                    // This will be used in asset fetching from the server
                    path: path_without_locale,
                    template: template.clone(),
                }),
                None => RouteVerdict::NotFound,
            };
        } else {
            // If the locale isn't supported, we assume that it's part of a route that still needs a locale (we'll detect the user's preferred)
            // This will result in a redirect, and the actual template to use will be determined after that
            // We'll just pass through the path to be redirected to (after it's had a locale placed in front)
            verdict = RouteVerdict::LocaleDetection(path_joined)
        }
    } else if locales.using_i18n {
        // If we're here, then we're using i18n, but we're at the root path, which is a locale detection point
        verdict = RouteVerdict::LocaleDetection(path_joined);
    } else {
        // Get the template to use
        let template = get_template_for_path(&path_joined, render_cfg, templates);
        verdict = match template {
            Some(template) => RouteVerdict::Found(RouteInfo {
                locale: locales.default.to_string(),
                // This will be used in asset fetching from the server
                path: path_joined,
                template: template.clone(),
            }),
            None => RouteVerdict::NotFound,
        };
    }

    verdict
}

/// Information about a route, which, combined with error pages and a client-side translations manager, allows the initialization of
/// the app shell and the rendering of a page.
pub struct RouteInfo<G: GenericNode> {
    /// The actual path of the route.
    pub path: String,
    /// The template that will be used. The app shell will derive pros and a translator to pass to the template function.
    pub template: Template<G>,
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
        struct $name<G: $crate::GenericNode>($crate::router::RouteVerdict<G>);
        impl<G: $crate::GenericNode> ::sycamore_router::Route for $name<G> {
            fn match_route(path: &[&str]) -> Self {
                let verdict = $crate::router::match_route(path, $render_cfg, $templates, $locales);
                // BUG Sycamore doesn't call the route verdict matching logic for some reason, but we get to this point
                Self(verdict)
            }
        }
    };
}
