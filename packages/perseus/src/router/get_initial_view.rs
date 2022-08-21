use crate::error_pages::ErrorPageData;
use crate::errors::*;
use crate::i18n::{detect_locale, ClientTranslationsManager, Locales};
use crate::router::match_route;
use crate::router::{RouteInfo, RouteVerdict, RouterLoadState, RouterState};
use crate::template::{PageProps, TemplateMap, TemplateNodeType};
use crate::utils::checkpoint;
use crate::ErrorPages;
use fmterr::fmt_err;
use std::collections::HashMap;
use sycamore::prelude::*;
use sycamore::rt::Reflect; // We can piggyback off Sycamore to avoid bringing in `js_sys`
use wasm_bindgen::JsValue;

pub(crate) struct GetInitialViewProps<'a, 'cx> {
    /// The app's reactive scope.
    pub cx: Scope<'cx>,
    /// The path we're rendering for (not the template path, the full path,
    /// though parsed a little).
    pub path: String,
    /// The router state.
    pub router_state: RouterState,
    /// A *client-side* translations manager to use (this manages caching
    /// translations).
    pub translations_manager: &'a ClientTranslationsManager,
    /// The error pages, for use if something fails.
    pub error_pages: &'a ErrorPages<TemplateNodeType>,
    /// The locales settings the app is using.
    pub locales: &'a Locales,
    /// The templates the app is using.
    pub templates: &'a TemplateMap<TemplateNodeType>,
    /// The render configuration of the app (which lays out routing information,
    /// among other things).
    pub render_cfg: &'a HashMap<String, String>,
}

/// Gets the initial view that we should display when the app first loads. This
/// doesn't need to be asynchronous, since initial loads provide everything
/// necessary for hydration in one single HTML file (including state and
/// translator sources).
///
/// Note that this function can only be run once, since it will delete the
/// initial state infrastructure from the page entirely. If this function is run
/// without that infrastructure being present, an error page will be rendered.
///
/// Note that this will automatically update the router state just before it
/// returns, meaning that any errors that may occur after this function has been
/// called need to reset the router state to be an error.
pub(crate) fn get_initial_view(
    GetInitialViewProps {
        cx,
        path,
        mut router_state,
        translations_manager,
        error_pages,
        locales,
        templates,
        render_cfg,
    }: GetInitialViewProps<'_, '_>,
) -> InitialView {
    // Start by figuring out what template we should be rendering
    let path_segments = path
        .split('/')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>(); // This parsing is identical to the Sycamore router's
    let verdict = match_route(&path_segments, &render_cfg, &templates, &locales);
    match &verdict {
        RouteVerdict::Found(RouteInfo {
            path,
            template,
            locale,
            // Since we're not requesting anything from the server, we don't need to worry about
            // whether it's an incremental match or not
            was_incremental_match: _,
        }) => InitialView::View({
            let path_with_locale = match locale.as_str() {
                "xx-XX" => path.clone(),
                locale => format!("{}/{}", locale, &path),
            };
            // Update the router state
            router_state.set_load_state(RouterLoadState::Loading {
                template_name: template.get_path(),
                path: path_with_locale.clone(),
            });
            router_state.set_last_verdict(verdict.clone());

            // Get the initial state and decide what to do from that
            let initial_state = get_initial_state();
            match initial_state {
                InitialState::Present(state) => {
                    checkpoint("initial_state_present");
                    let global_state = get_global_state();
                    // Unset the initial state variable so we perform subsequent renders correctly
                    // This monstrosity is needed until `web-sys` adds a `.set()` method on `Window`
                    // We don't do this for the global state because it should hang around
                    // uninitialized until a template wants it (if we remove it before then, we're
                    // stuffed)
                    Reflect::set(
                        &JsValue::from(web_sys::window().unwrap()),
                        &JsValue::from("__PERSEUS_INITIAL_STATE"),
                        &JsValue::undefined(),
                    )
                    .unwrap();

                    // Get the translator from the page (this has to exist, or the server stuffed
                    // up); doing this without a network request minimizes
                    // the time to interactivity (improving UX drastically), while meaning that we
                    // never have to fetch translations separately unless the user switches locales
                    let translations_str = match get_translations() {
                        Some(translations_str) => translations_str,
                        None => {
                            router_state.set_load_state(RouterLoadState::ErrorLoaded {
                                path: path_with_locale.clone(),
                            });
                            return InitialView::View(error_pages.get_view_and_render_head(
                                cx,
                                "*",
                                500,
                                "expected translations in global variable, but none found",
                                None,
                            ));
                        }
                    };
                    let translator = translations_manager
                        .get_translator_for_translations_str(&locale, &translations_str);
                    let translator = match translator {
                        Ok(translator) => translator,
                        Err(err) => {
                            router_state.set_load_state(RouterLoadState::ErrorLoaded {
                                path: path_with_locale.clone(),
                            });
                            return InitialView::View(match &err {
                                // These errors happen because we couldn't get a translator, so they certainly don't get one
                                ClientError::FetchError(FetchError::NotOk { url, status, .. }) => error_pages.get_view_and_render_head(cx, url, *status, &fmt_err(&err), None),
                                ClientError::FetchError(FetchError::SerFailed { url, .. }) => error_pages.get_view_and_render_head(cx, url, 500, &fmt_err(&err), None),
                                ClientError::LocaleNotSupported { .. } => error_pages.get_view_and_render_head(cx, &format!("/{}/...", locale), 404, &fmt_err(&err), None),
                                // No other errors should be returned
                                _ => panic!("expected 'AssetNotOk'/'AssetSerFailed'/'LocaleNotSupported' error, found other unacceptable error")
                            });
                        }
                    };

                    let path = template.get_path();
                    let page_props = PageProps {
                        path: path_with_locale.clone(),
                        state,
                        global_state,
                    };
                    // Pre-emptively declare the page interactive 9since all we do from this point
                    // is hydrate
                    checkpoint("page_interactive");
                    // Update the router state
                    router_state.set_load_state(RouterLoadState::Loaded {
                        template_name: path,
                        path: path_with_locale,
                    });
                    // Return the actual template, for rendering/hydration
                    template.render_for_template_client(page_props, cx, translator)
                }
                // We have an error that the server sent down, so we should just return that error
                // view
                InitialState::Error(ErrorPageData { url, status, err }) => {
                    checkpoint("initial_state_error");
                    router_state.set_load_state(RouterLoadState::ErrorLoaded {
                        path: path_with_locale.clone(),
                    });
                    // We don't need to replace the head, because the server's handled that for us
                    error_pages.get_view(cx, &url, status, &err, None)
                }
                // The entire purpose of this function is to work with the initial state, so if this
                // is true, then we have a problem
                // Theoretically, this should never
                // happen... (but I've seen magical infinite loops that crash browsers, so I'm
                // hedging my bets)
                InitialState::NotPresent => {
                    checkpoint("initial_state_error");
                    router_state.set_load_state(RouterLoadState::ErrorLoaded {
                        path: path_with_locale.clone(),
                    });
                    error_pages.get_view_and_render_head(cx, "*", 400, "expected initial state render, found subsequent load (highly likely to be a core perseus bug)", None)
                }
            }
        }),
        // If the user is using i18n, then they'll want to detect the locale on any paths
        // missing a locale Those all go to the same system that redirects to the
        // appropriate locale Note that `container` doesn't exist for this scenario
        RouteVerdict::LocaleDetection(path) => {
            InitialView::Redirect(detect_locale(path.clone(), &locales))
        }
        RouteVerdict::NotFound => InitialView::View({
            checkpoint("not_found");
            if let InitialState::Error(ErrorPageData { url, status, err }) = get_initial_state() {
                router_state.set_load_state(RouterLoadState::ErrorLoaded { path: url.clone() });
                // If this is an error from an initial state page, then we'll hydrate whatever's
                // already there
                //
                // Since this page has come from the server, anything could have happened, so we
                // provide no translator (and one certainly won't exist in context)
                // But we don't need to replace the head, since the server will have already
                // done that
                error_pages.get_view(cx, &url, status, &err, None)
            } else {
                // TODO Update the router state
                // router_state.set_load_state(RouterLoadState::ErrorLoaded {
                //     path: path_with_locale.clone()
                // });
                // Given that were only handling the initial load, this should really never
                // happen...
                error_pages.get_view_and_render_head(cx, "", 404, "not found", None)
            }
        }),
    }
}

/// A representation of the possible outcomes of getting the view for the
/// initial load.
pub(crate) enum InitialView {
    /// A view is available to be rendered/hydrated.
    View(View<TemplateNodeType>),
    /// We need to redirect somewhere else, and the path to redirect to is
    /// attached.
    ///
    /// Currently, this is only used by locale redirection, though this could
    /// theoretically also be used for server-level reloads, if those
    /// directives are ever supported.
    Redirect(String),
}

/// A representation of whether or not the initial state was present. If it was,
/// it could be `None` (some templates take no state), and if not, then this
/// isn't an initial load, and we need to request the page from the server. It
/// could also be an error that the server has rendered.
#[derive(Debug)]
pub(crate) enum InitialState {
    /// A non-error initial state has been injected. This could be `None`, since
    /// not all pages have state.
    Present(Option<String>),
    /// An initial state has been injected that indicates an error.
    Error(ErrorPageData),
    /// No initial state has been injected (or if it has, it's been deliberately
    /// unset).
    NotPresent,
}

/// Gets the initial state injected by the server, if there was any. This is
/// used to differentiate initial loads from subsequent ones, which have
/// different log chains to prevent double-trips (a common SPA problem).
pub(crate) fn get_initial_state() -> InitialState {
    let val_opt = web_sys::window().unwrap().get("__PERSEUS_INITIAL_STATE");
    let js_obj = match val_opt {
        Some(js_obj) => js_obj,
        None => return InitialState::NotPresent,
    };
    // The object should only actually contain the string value that was injected
    let state_str = match js_obj.as_string() {
        Some(state_str) => state_str,
        None => return InitialState::NotPresent,
    };
    // On the server-side, we encode a `None` value directly (otherwise it will be
    // some convoluted stringified JSON)
    if state_str == "None" {
        InitialState::Present(None)
    } else if state_str.starts_with("error-") {
        // We strip the prefix and escape any tab/newline control characters (inserted
        // by `fmterr`) Any others are user-inserted, and this is documented
        let err_page_data_str = state_str
            .strip_prefix("error-")
            .unwrap()
            .replace('\n', "\\n")
            .replace('\t', "\\t");
        // There will be error page data encoded after `error-`
        let err_page_data = match serde_json::from_str::<ErrorPageData>(&err_page_data_str) {
            Ok(render_cfg) => render_cfg,
            // If there's a serialization error, we'll create a whole new error (500)
            Err(err) => ErrorPageData {
                url: "[current]".to_string(),
                status: 500,
                err: format!("couldn't serialize error from server: '{}'", err),
            },
        };
        InitialState::Error(err_page_data)
    } else {
        InitialState::Present(Some(state_str))
    }
}

/// Gets the global state injected by the server, if there was any. If there are
/// errors in this, we can return `None` and not worry about it, they'll be
/// handled by the initial state.
pub(crate) fn get_global_state() -> Option<String> {
    let val_opt = web_sys::window().unwrap().get("__PERSEUS_GLOBAL_STATE");
    let js_obj = match val_opt {
        Some(js_obj) => js_obj,
        None => return None,
    };
    // The object should only actually contain the string value that was injected
    let state_str = match js_obj.as_string() {
        Some(state_str) => state_str,
        None => return None,
    };
    // On the server-side, we encode a `None` value directly (otherwise it will be
    // some convoluted stringified JSON)
    match state_str.as_str() {
        "None" => None,
        state_str => Some(state_str.to_string()),
    }
}

/// Gets the translations injected by the server, if there was any. If there are
/// errors in this, we can return `None` and not worry about it, they'll be
/// handled by the initial state.
pub(crate) fn get_translations() -> Option<String> {
    let val_opt = web_sys::window().unwrap().get("__PERSEUS_TRANSLATIONS");
    let js_obj = match val_opt {
        Some(js_obj) => js_obj,
        None => return None,
    };
    // The object should only actually contain the string value that was injected
    let state_str = match js_obj.as_string() {
        Some(state_str) => state_str,
        None => return None,
    };

    // With translations, there's no such thing as `None` (even apps without i18n
    // have a dummy translator)
    Some(state_str)
}
