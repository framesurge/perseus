use crate::locales::Locales;
use crate::template::TemplateMap;
use crate::templates::ArcTemplateMap;
use crate::Html;
use crate::Template;
use std::collections::HashMap;
use std::rc::Rc;
use sycamore::prelude::ReadSignal;
use sycamore::prelude::Signal;

/// The backend for `get_template_for_path` to avoid code duplication for the `Arc` and `Rc` versions.
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
        // Next, an ISR match (more complex), which we only want to run if we didn't get an exact match above
        if template_name.is_empty() {
            // We progressively look for more and more specificity of the path, adding each segment
            // That way, we're searching forwards rather than backwards, which is more efficient
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

        // Return the necessary info for the caller to get the template in a form it wants (might be an `Rc` of a reference)
        (template_name, was_incremental_match)
	}};
}

/// Determines the template to use for the given path by checking against the render configuration., also returning whether we matched
/// a simple page or an incrementally-generated one (`true` for incrementally generated). Note that simple pages include those on
/// incrementally-generated templates that we pre-rendered with *build paths* at build-time (and are hence in an immutable store rather
/// than a mutable store).
///
/// This houses the central routing algorithm of Perseus, which is based fully on the fact that we know about every single page except
/// those rendered with ISR, and we can infer about them based on template root path domains. If that domain system is violated, this
/// routing algorithm will not behave as expected whatsoever (as far as routing goes, it's undefined behaviour)!
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

/// A version of `get_template_for_path` that accepts an `ArcTemplateMap<G>`. This is used by `match_route_atomic`, which should be used in scenarios in which the
/// template map needs to be passed betgween threads.
///
/// Warning: this returns a `&Template<G>` rather than a `Rc<Template<G>>`, and thus should only be used independently of the rest of Perseus (through `match_route_atomic`).
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

/// Matches the given path to a `RouteVerdict`. This takes a `TemplateMap` to match against, the render configuration to index, and it
/// needs to know if i18n is being used. The path this takes should be raw, it may or may not have a locale, but should be split into
/// segments by `/`, with empty ones having been removed.
pub fn match_route<G: Html>(
    path_slice: &[&str],
    render_cfg: &HashMap<String, String>,
    templates: &TemplateMap<G>,
    locales: &Locales,
) -> RouteVerdict<G> {
    let path_vec = path_slice.to_vec();
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

/// A version of `match_route` that accepts an `ArcTemplateMap<G>`. This should be used in multithreaded situations, like on the server.
pub fn match_route_atomic<'a, G: Html>(
    path_slice: &[&str],
    render_cfg: &HashMap<String, String>,
    templates: &'a ArcTemplateMap<G>,
    locales: &Locales,
) -> RouteVerdictAtomic<'a, G> {
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
            // If the locale isn't supported, we assume that it's part of a route that still needs a locale (we'll detect the user's preferred)
            // This will result in a redirect, and the actual template to use will be determined after that
            // We'll just pass through the path to be redirected to (after it's had a locale placed in front)
            verdict = RouteVerdictAtomic::LocaleDetection(path_joined)
        }
    } else if locales.using_i18n {
        // If we're here, then we're using i18n, but we're at the root path, which is a locale detection point
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

/// Information about a route, which, combined with error pages and a client-side translations manager, allows the initialization of
/// the app shell and the rendering of a page.
#[derive(Debug)]
pub struct RouteInfo<G: Html> {
    /// The actual path of the route.
    pub path: String,
    /// The template that will be used. The app shell will derive props and a translator to pass to the template function.
    pub template: Rc<Template<G>>,
    /// Whether or not the matched page was incrementally-generated at runtime (if it has been yet). If this is `true`, the server will
    /// use a mutable store rather than an immutable one. See the book for more details.
    pub was_incremental_match: bool,
    /// The locale for the template to be rendered in.
    pub locale: String,
}

/// The possible outcomes of matching a route. This is an alternative implementation of Sycamore's `Route` trait to enable greater
/// control and tighter integration of routing with templates. This can only be used if `Routes` has been defined in context (done
/// automatically by the CLI).
#[derive(Debug)]
pub enum RouteVerdict<G: Html> {
    /// The given route was found, and route information is attached.
    Found(RouteInfo<G>),
    /// The given route was not found, and a `404 Not Found` page should be shown.
    NotFound,
    /// The given route maps to the locale detector, which will redirect the user to the attached path (in the appropriate locale).
    LocaleDetection(String),
}

/// Information about a route, which, combined with error pages and a client-side translations manager, allows the initialization of
/// the app shell and the rendering of a page.
///
/// This version is designed for multithreaded scenarios, and stores a reference to a template rather than an `Rc<Template<G>>`. That means this is not compatible
/// with Perseus on the client-side, only on the server-side.
#[derive(Debug)]
pub struct RouteInfoAtomic<'a, G: Html> {
    /// The actual path of the route.
    pub path: String,
    /// The template that will be used. The app shell will derive props and a translator to pass to the template function.
    pub template: &'a Template<G>,
    /// Whether or not the matched page was incrementally-generated at runtime (if it has been yet). If this is `true`, the server will
    /// use a mutable store rather than an immutable one. See the book for more details.
    pub was_incremental_match: bool,
    /// The locale for the template to be rendered in.
    pub locale: String,
}

/// The possible outcomes of matching a route. This is an alternative implementation of Sycamore's `Route` trait to enable greater
/// control and tighter integration of routing with templates. This can only be used if `Routes` has been defined in context (done
/// automatically by the CLI).
///
/// This version uses `RouteInfoAtomic`, and is designed for multithreaded scenarios (i.e. on the server).
#[derive(Debug)]
pub enum RouteVerdictAtomic<'a, G: Html> {
    /// The given route was found, and route information is attached.
    Found(RouteInfoAtomic<'a, G>),
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
        struct $name<G: $crate::Html>($crate::internal::router::RouteVerdict<G>);
        impl<G: $crate::Html> ::sycamore_router::Route for $name<G> {
            fn match_route(path: &[&str]) -> Self {
                let verdict = $crate::internal::router::match_route(path, $render_cfg, $templates, $locales);
                Self(verdict)
            }
        }
    };
}

/// The state for the router.
#[derive(Clone, Debug)]
pub struct RouterState {
    /// The router's current load state.
    load_state: Signal<RouterLoadState>,
}
impl Default for RouterState {
    /// Creates a default instance of the router state intended for server-side usage.
    fn default() -> Self {
        Self {
            load_state: Signal::new(RouterLoadState::Server),
        }
    }
}
impl RouterState {
    /// Gets the load state of the router.
    pub fn get_load_state(&self) -> ReadSignal<RouterLoadState> {
        self.load_state.handle()
    }
    /// Sets the load state of the router.
    pub fn set_load_state(&self, new: RouterLoadState) {
        self.load_state.set(new);
    }
}

/// The current load state of the router. You can use this to be warned of when a new page is about to be loaded (and display a loading bar or the like, perhaps).
#[derive(Clone, Debug)]
pub enum RouterLoadState {
    /// The page has been loaded.
    Loaded {
        /// The name of the template being loaded (mostly for convenience).
        template_name: String,
        /// The full path to the new page being loaded (including the locale, if we're using i18n).
        path: String,
    },
    /// A new page is being loaded, and will soon replace whatever is currently loaded. The name of the new template is attached.
    Loading {
        /// The name of the template being loaded (mostly for convenience).
        template_name: String,
        /// The full path to the new page being loaded (including the locale, if we're using i18n).
        path: String,
    },
    /// We're on the server, and there is no router. Whatever you render based on this state will appear when the user first loads the page, before it's made interactive.
    Server,
}
