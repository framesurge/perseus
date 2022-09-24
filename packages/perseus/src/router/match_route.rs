use super::{RouteInfo, RouteInfoAtomic, RouteVerdict, RouteVerdictAtomic};
use crate::i18n::Locales;
use crate::template::{ArcTemplateMap, Template, TemplateMap};
use crate::Html;
use std::collections::HashMap;
use std::rc::Rc;

/// The backend for `get_template_for_path` to avoid code duplication for the
/// `Arc` and `Rc` versions.
macro_rules! get_template_for_path {
    ($raw_path:expr, $render_cfg:expr, $templates:expr) => {{
        let mut path = $raw_path;
        // If the path is empty, we're looking for the special `index` page
        if path.is_empty() {
            path = "index";
        }

        let mut was_incremental_match = false;
        // Match the path to one of the templates
        let mut template_name = String::new();
        // We'll try a direct match first
        if let Some(template_root_path) = $render_cfg.get(path) {
            template_name = template_root_path.to_string();
        }
        // Next, an ISR match (more complex), which we only want to run if we didn't get
        // an exact match above
        if template_name.is_empty() {
            // We progressively look for more and more specificity of the path, adding each
            // segment That way, we're searching forwards rather than backwards,
            // which is more efficient
            let path_segments: Vec<&str> = path.split('/').collect();
            for (idx, _) in path_segments.iter().enumerate() {
                // Make a path out of this and all the previous segments
                let path_to_try = path_segments[0..(idx + 1)].join("/") + "/*";

                // If we find something, keep going until we don't (maximise specificity)
                if let Some(template_root_path) = $render_cfg.get(&path_to_try) {
                    was_incremental_match = true;
                    template_name = template_root_path.to_string();
                } else {
                    break;
                }
            }
        }
        // If we still have nothing, then the page doesn't exist
        if template_name.is_empty() {
            return (None, was_incremental_match);
        }

        // Return the necessary info for the caller to get the template in a form it
        // wants (might be an `Rc` of a reference)
        (template_name, was_incremental_match)
    }};
}

/// Determines the template to use for the given path by checking against the
/// render configuration, also returning whether we matched a simple page or an
/// incrementally-generated one (`true` for incrementally generated). Note that
/// simple pages include those on incrementally-generated templates that we
/// pre-rendered with *build paths* at build-time (and are hence in an immutable
/// store rather than a mutable store).
///
/// This houses the central routing algorithm of Perseus, which is based fully
/// on the fact that we know about every single page except those rendered with
/// ISR, and we can infer about them based on template root path domains. If
/// that domain system is violated, this routing algorithm will not behave as
/// expected whatsoever (as far as routing goes, it's undefined behavior)!
///
/// *Note:* in the vast majority of cases, you should never need to use
/// this function.
pub fn get_template_for_path<G: Html>(
    raw_path: &str,
    render_cfg: &HashMap<String, String>,
    templates: &TemplateMap<G>,
) -> (Option<Rc<Template<G>>>, bool) {
    let (template_name, was_incremental_match) =
        get_template_for_path!(raw_path, render_cfg, templates);

    (
        templates.get(&template_name).cloned(),
        was_incremental_match,
    )
}

/// A version of `get_template_for_path` that accepts an `ArcTemplateMap<G>`.
/// This is used by `match_route_atomic`, which should be used in scenarios in
/// which the template map needs to be passed between threads.
///
/// Warning: this returns a `&Template<G>` rather than a `Rc<Template<G>>`, and
/// thus should only be used independently of the rest of Perseus (through
/// `match_route_atomic`).
///
/// *Note:* in the vast majority of cases, you should never need to use
/// this function.
pub fn get_template_for_path_atomic<'a, G: Html>(
    raw_path: &str,
    render_cfg: &HashMap<String, String>,
    templates: &'a ArcTemplateMap<G>,
) -> (Option<&'a Template<G>>, bool) {
    let (template_name, was_incremental_match) =
        get_template_for_path!(raw_path, render_cfg, templates);

    (
        templates
            .get(&template_name)
            .map(|pointer| pointer.as_ref()),
        was_incremental_match,
    )
}

/// Matches the given path to a `RouteVerdict`. This takes a `TemplateMap` to
/// match against, the render configuration to index, and it needs to know if
/// i18n is being used. The path this takes should be raw, it may or may not
/// have a locale, but should be split into segments by `/`, with empty ones
/// having been removed.
///
/// *Note:* in the vast majority of cases, you should never need to use
/// this function.
pub fn match_route<G: Html>(
    path_slice: &[&str],
    render_cfg: &HashMap<String, String>,
    templates: &TemplateMap<G>,
    locales: &Locales,
) -> RouteVerdict<G> {
    let path_vec = path_slice.to_vec();
    let path_joined = path_vec.join("/"); // This should not have a leading forward slash, it's used for asset fetching by
                                          // the app shell

    let verdict;
    // There are different logic chains if we're using i18n, so we fork out early
    if locales.using_i18n && !path_slice.is_empty() {
        let locale = path_slice[0];
        // Check if the 'locale' is supported (otherwise it may be the first section of
        // an uni18ned route)
        if locales.is_supported(locale) {
            // We'll assume this has already been i18ned (if one of your routes has the same
            // name as a supported locale, ffs)
            let path_without_locale = path_slice[1..].to_vec().join("/");
            // Get the template to use
            let (template, was_incremental_match) =
                get_template_for_path(&path_without_locale, render_cfg, templates);
            verdict = match template {
                Some(template) => RouteVerdict::Found(RouteInfo {
                    locale: locale.to_string(),
                    // This will be used in asset fetching from the server
                    path: path_without_locale,
                    template,
                    was_incremental_match,
                }),
                None => RouteVerdict::NotFound,
            };
        } else {
            // If the locale isn't supported, we assume that it's part of a route that still
            // needs a locale (we'll detect the user's preferred)
            // This will result in a redirect, and the actual template to use will be
            // determined after that We'll just pass through the path to be
            // redirected to (after it's had a locale placed in front)
            verdict = RouteVerdict::LocaleDetection(path_joined)
        }
    } else if locales.using_i18n {
        // If we're here, then we're using i18n, but we're at the root path, which is a
        // locale detection point
        verdict = RouteVerdict::LocaleDetection(path_joined);
    } else {
        // Get the template to use
        let (template, was_incremental_match) =
            get_template_for_path(&path_joined, render_cfg, templates);
        verdict = match template {
            Some(template) => RouteVerdict::Found(RouteInfo {
                locale: locales.default.to_string(),
                // This will be used in asset fetching from the server
                path: path_joined,
                template,
                was_incremental_match,
            }),
            None => RouteVerdict::NotFound,
        };
    }

    verdict
}

/// A version of `match_route` that accepts an `ArcTemplateMap<G>`. This should
/// be used in multithreaded situations, like on the server.
///
/// *Note:* in the vast majority of cases, you should never need to use
/// this function.
pub fn match_route_atomic<'a, G: Html>(
    path_slice: &[&str],
    render_cfg: &HashMap<String, String>,
    templates: &'a ArcTemplateMap<G>,
    locales: &Locales,
) -> RouteVerdictAtomic<'a, G> {
    let path_vec: Vec<&str> = path_slice.to_vec();
    let path_joined = path_vec.join("/"); // This should not have a leading forward slash, it's used for asset fetching by
                                          // the app shell

    let verdict;
    // There are different logic chains if we're using i18n, so we fork out early
    if locales.using_i18n && !path_slice.is_empty() {
        let locale = path_slice[0];
        // Check if the 'locale' is supported (otherwise it may be the first section of
        // an uni18ned route)
        if locales.is_supported(locale) {
            // We'll assume this has already been i18ned (if one of your routes has the same
            // name as a supported locale, ffs)
            let path_without_locale = path_slice[1..].to_vec().join("/");
            // Get the template to use
            let (template, was_incremental_match) =
                get_template_for_path_atomic(&path_without_locale, render_cfg, templates);
            verdict = match template {
                Some(template) => RouteVerdictAtomic::Found(RouteInfoAtomic {
                    locale: locale.to_string(),
                    // This will be used in asset fetching from the server
                    path: path_without_locale,
                    template,
                    was_incremental_match,
                }),
                None => RouteVerdictAtomic::NotFound,
            };
        } else {
            // If the locale isn't supported, we assume that it's part of a route that still
            // needs a locale (we'll detect the user's preferred)
            // This will result in a redirect, and the actual template to use will be
            // determined after that We'll just pass through the path to be
            // redirected to (after it's had a locale placed in front)
            verdict = RouteVerdictAtomic::LocaleDetection(path_joined)
        }
    } else if locales.using_i18n {
        // If we're here, then we're using i18n, but we're at the root path, which is a
        // locale detection point
        verdict = RouteVerdictAtomic::LocaleDetection(path_joined);
    } else {
        // Get the template to use
        let (template, was_incremental_match) =
            get_template_for_path_atomic(&path_joined, render_cfg, templates);
        verdict = match template {
            Some(template) => RouteVerdictAtomic::Found(RouteInfoAtomic {
                locale: locales.default.to_string(),
                // This will be used in asset fetching from the server
                path: path_joined,
                template,
                was_incremental_match,
            }),
            None => RouteVerdictAtomic::NotFound,
        };
    }

    verdict
}
