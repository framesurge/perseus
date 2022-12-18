use super::{RouteInfo, RouteVerdict};
use crate::i18n::Locales;
use crate::template::Entity;
use crate::{path::*, Html};
use std::collections::HashMap;

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
fn get_template_for_path<G: Html>(
    path: &str,
    render_cfg: &HashMap<String, String>,
    entities: &HashMap<String, Entity<G>>,
) -> (Option<Entity<G>>, bool) {
    let mut was_incremental_match = false;
    // Match the path to one of the entities
    let mut entity_name = None;
    // We'll try a direct match first
    if let Some(entity_root_path) = render_cfg.get(path) {
        entity_name = Some(entity_root_path.to_string());
    }
    // Next, an ISR match (more complex), which we only want to run if we didn't get
    // an exact match above
    if entity_name.is_none() {
        // We progressively look for more and more specificity of the path, adding each
        // segment That way, we're searching forwards rather than backwards,
        // which is more efficient
        let path_segments: Vec<&str> = path.split('/').collect();
        for (idx, _) in path_segments.iter().enumerate() {
            // Make a path out of this and all the previous segments
            let path_to_try = path_segments[0..(idx + 1)].join("/") + "/*";

            // If we find something, keep going until we don't (maximize specificity)
            if let Some(entity_root_path) = render_cfg.get(&path_to_try) {
                was_incremental_match = true;
                entity_name = Some(entity_root_path.to_string());
            } else {
                break;
            }
        }
    }
    // If we still have nothing, then the page doesn't exist
    if let Some(entity_name) = entity_name {
        (entities.get(&entity_name).cloned(), was_incremental_match)
    } else {
        (None, was_incremental_match)
    }
}

/// Matches the given path to a `RouteVerdict`. This takes a `TemplateMap` to
/// match against, the render configuration to index, and it needs to know if
/// i18n is being used. The path this takes should be raw, it may or may not
/// have a locale, but should be split into segments by `/`, with empty ones
/// having been removed.
///
/// *Note:* in the vast majority of cases, you should never need to use
/// this function.
pub(crate) fn match_route<G: Html>(
    path_slice: &[&str],
    render_cfg: &HashMap<String, String>,
    entities: &HashMap<String, Entity<G>>,
    locales: &Locales,
) -> RouteVerdict<G> {
    let path_vec = path_slice.to_vec();
    let path_joined = PathMaybeWithLocale(path_vec.join("/")); // This should not have a leading forward slash, it's used for asset fetching by
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
            let path_without_locale = PathWithoutLocale(path_slice[1..].to_vec().join("/"));
            // Get the template to use
            let (entity, was_incremental_match) =
                get_template_for_path(&path_without_locale, render_cfg, entities);
            verdict = match entity {
                Some(entity) => RouteVerdict::Found(RouteInfo {
                    locale: locale.to_string(),
                    // This will be used in asset fetching from the server
                    path: path_without_locale,
                    entity,
                    was_incremental_match,
                }),
                None => RouteVerdict::NotFound {
                    locale: locale.to_string(),
                },
            };
        } else {
            // If the locale isn't supported, we assume that it's part of a route that still
            // needs a locale (we'll detect the user's preferred)
            // This will result in a redirect, and the actual template to use will be
            // determined after that We'll just pass through the path to be
            // redirected to (after it's had a locale placed in front)
            let path_joined = PathWithoutLocale(path_joined.0);
            verdict = RouteVerdict::LocaleDetection(path_joined)
        }
    } else if locales.using_i18n {
        // If we're here, then we're using i18n, but we're at the root path, which is a
        // locale detection point
        let path_joined = PathWithoutLocale(path_joined.0);
        verdict = RouteVerdict::LocaleDetection(path_joined);
    } else {
        // We're not using i18n
        let path_joined = PathWithoutLocale(path_joined.0);
        // Get the template to use
        let (entity, was_incremental_match) =
            get_template_for_path(&path_joined, render_cfg, entities);
        verdict = match entity {
            Some(entity) => RouteVerdict::Found(RouteInfo {
                locale: locales.default.to_string(),
                // This will be used in asset fetching from the server
                path: path_joined,
                entity,
                was_incremental_match,
            }),
            None => RouteVerdict::NotFound {
                locale: "xx-XX".to_string(),
            },
        };
    }

    verdict
}
