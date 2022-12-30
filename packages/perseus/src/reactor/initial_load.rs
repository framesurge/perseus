use std::collections::HashMap;

use crate::{
    checkpoint,
    error_views::ServerErrorData,
    errors::*,
    i18n::detect_locale,
    path::PathMaybeWithLocale,
    router::{match_route, FullRouteInfo, FullRouteVerdict, RouterLoadState},
    state::TemplateState,
    utils::get_path_prefix_client,
};
use serde_json::Value;
use sycamore::{
    prelude::{Scope, ScopeDisposer},
    view::View,
    web::Html,
};
use web_sys::Element;

use super::{Reactor, WindowVariable};

impl<G: Html> Reactor<G> {
    /// Gets the initial view to hydrate, which will be the same as what the
    /// engine-side rendered and provided. This will automatically extract
    /// the current path from the browser.
    pub(crate) fn get_initial_view<'a>(
        &self,
        cx: Scope<'a>,
    ) -> Result<InitialView<'a, G>, ClientError> {
        // Get the current path, removing any base paths to avoid relative path locale
        // redirection loops (in previous versions of Perseus, we used Sycamore to
        // get the path, and it strips this out automatically)
        // Note that this does work with full URL paths, because
        // `get_path_prefix_client` does automatically get just the pathname
        // component.
        let path_prefix = get_path_prefix_client();
        let path = web_sys::window().unwrap().location().pathname().unwrap();
        let path = if path.starts_with(&path_prefix) {
            path.strip_prefix(&path_prefix).unwrap()
        } else {
            &path
        };
        let path = js_sys::decode_uri_component(&path)
            .map_err(|_| ClientPlatformError::InitialPath)?
            .as_string()
            .ok_or(ClientPlatformError::InitialPath)?;

        // Start by figuring out what template we should be rendering
        let path_segments = path
            .split('/')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>(); // This parsing is identical to the Sycamore router's
        let verdict = match_route(
            &path_segments,
            &self.render_cfg,
            &self.entities,
            &self.locales,
        );
        // We'll need this later for setting the router state
        let slim_verdict = verdict.clone();
        match &verdict.into_full(&self.entities) {
            // WARNING: This will be triggered on *all* incremental paths, even if
            // the serber returns a 404!
            FullRouteVerdict::Found(FullRouteInfo {
                path,
                entity,
                locale,
                // Since we're not requesting anything from the server, we don't need to worry about
                // whether it's an incremental match or not
                was_incremental_match: _,
            }) => {
                let full_path = PathMaybeWithLocale::new(&path, &locale);
                // Update the router state as we try to load (since this is the initial
                // view, this will be the first change since the server)
                self.router_state.set_load_state(RouterLoadState::Loading {
                    template_name: entity.get_path(),
                    path: full_path.clone(),
                });
                self.router_state.set_last_verdict(slim_verdict);

                // Get the initial state and decide what to do from that. We can guarantee that
                // this locale is supported because it came from `match_route`.
                let state = self.get_initial_state(locale)?;

                // Get the translator from the page (this has to exist, or the server stuffed
                // up); doing this without a network request minimizes
                // the time to interactivity (improving UX drastically), while meaning that we
                // never have to fetch translations separately unless the user switches locales
                let translations_str = match WindowVariable::new_str("__PERSEUS_TRANSLATIONS") {
                    WindowVariable::Some(state_str) => state_str,
                    WindowVariable::Malformed | WindowVariable::None => {
                        return Err(ClientInvariantError::Translations.into())
                    }
                };
                // This will cache the translator internally in the reactor (which can be
                // accessed later through the`t!` macro etc.). This locale is guaranteed to
                // be supported, because it came from a `match_route`.
                self.translations_manager
                    .set_translator_for_translations_str(&locale, &translations_str)?;

                #[cfg(feature = "cache-initial-load")]
                {
                    // Cache the page's head in the PSS (getting it as reliably as we can, which
                    // isn't perfect, hence the feature-gate). Without this, we
                    // would have to get the head from the server on
                    // a subsequent load back to this page, which isn't ideal.
                    let head_str = Self::get_head()?;
                    self.state_store.add_head(&full_path, head_str, false); // We know this is a page
                }

                // Get the widget states and register them all as preloads in the state store so
                // they can be accessed by the `Widget` component. Like other
                // window variables, this will always be present, even if there
                // were no widgets used.
                let widget_states =
                    match WindowVariable::<HashMap<PathMaybeWithLocale, Value>>::new_obj(
                        "__PERSEUS_INITIAL_WIDGET_STATES",
                    ) {
                        WindowVariable::Some(states) => states,
                        WindowVariable::None | WindowVariable::Malformed => {
                            return Err(ClientInvariantError::WidgetStates.into())
                        }
                    };
                for (widget_path, state_res) in widget_states.into_iter() {
                    // NOTE: `state_res` could be `ServerErrorData`!
                    self.state_store.add_initial_widget(widget_path, state_res);
                }

                // Render the actual template to the root (done imperatively due to child
                // scopes)
                let (view, disposer) = entity.render_for_template_client(full_path, state, cx)?;

                Ok(InitialView::View(view, disposer))
            }
            // If the user is using i18n, then they'll want to detect the locale on any paths
            // missing a locale. Those all go to the same system that redirects to the
            // appropriate locale. This returns a full URL to imperatively redirect to.
            FullRouteVerdict::LocaleDetection(path) => Ok(InitialView::Redirect(detect_locale(
                path.clone(),
                &self.locales,
            ))),
            // Since all unlocalized 404s go to a redirect, we always have a locale here. Provided
            // the server is being remotely reasonable, we should have translations too,
            // *unless* the error page was exported, in which case we're up the creek.
            // TODO Fetch translations with exported error pages? Solution??
            FullRouteVerdict::NotFound { locale } => {
                // Check what we have in the error page data. We would expect this to be a
                // `ClientError::ServerError { status: 404, source: "page not found" }`, but
                // other invariants could have been broken. So, we propagate any errors up
                // happily. If this is `Ok(_)`, we have a *serious* problem, as
                // that means the engine thought this page was valid, but we
                // disagree. This should not happen without tampering,
                // so we'll return an invariant error.
                // We can guarantee that the locale is supported because it came from a
                // `match_route`, even though the route wasn't found. If the app
                // isn't using i18n, it will be `xx-XX`.
                match self.get_initial_state(locale) {
                    Err(err) => Err(err),
                    Ok(_) => Err(ClientInvariantError::RouterMismatch.into()),
                }
            }
        }
    }

    /// Gets the initial state injected by the server, if there was any. This is
    /// used to differentiate initial loads from subsequent ones, which have
    /// different log chains to prevent double-trips (a common SPA problem).
    ///
    /// # Panics
    /// This will panic if the given locale is not supported.
    fn get_initial_state(&self, locale: &str) -> Result<TemplateState, ClientError> {
        let state_str = match WindowVariable::new_str("__PERSEUS_INITIAL_STATE") {
            WindowVariable::Some(state_str) => state_str,
            WindowVariable::Malformed | WindowVariable::None => {
                return Err(ClientInvariantError::InitialState.into())
            }
        };

        // If there was an error, it's specially injected with this prefix before error
        // page data
        if state_str.starts_with("error-") {
            // We strip the prefix and escape any tab/newline control characters (inserted
            // by `fmterr`). Any others are user-inserted, and this is documented.
            let err_page_data_str = state_str
                .strip_prefix("error-")
                .unwrap()
                .replace('\n', "\\n")
                .replace('\t', "\\t");
            // There will be error page data encoded after `error-`
            let err_page_data = match serde_json::from_str::<ServerErrorData>(&err_page_data_str) {
                Ok(data) => data,
                Err(err) => {
                    return Err(ClientInvariantError::InitialStateError { source: err }.into())
                }
            };
            // This will be sent back to the handler for proper rendering, the only
            // difference is that the user won't get a flash to an error page,
            // they will have started with an error
            let err = ClientError::ServerError {
                status: err_page_data.status,
                message: err_page_data.msg,
            };
            // We do this in here so that even incremental pages that appear fine to the
            // router, but that actually error out, trigger this checkpoint
            if err_page_data.status == 404 {
                checkpoint("not_found");
            }
            // In some nice cases, the server will have been able to figure out the locale,
            // which we should have (this is one of those things that most sites
            // don't bother with because it's not easy to build, and *this* is
            // where a framework really shines). If we do have it, it'll be
            // in the `__PERSEUS_TRANSLATIONS` variable. If that's there, then the error
            // provided will be localized, so, if we can't get the translator,
            // we'll prefer to return an internal error that comes up as a popup
            // (since we don't want to replace a localized error with an unlocalized one).
            // If we know we have something unlocalized, just replace it with whatever we
            // have now.
            //
            // Note: in the case of a server-given error, we'll only not have translations
            // if there was an internal error (since `/this-page-does-not-exist`
            // would be a locale redirection).
            match WindowVariable::new_str("__PERSEUS_TRANSLATIONS") {
                // We have translations! Any errors in resolving them fully will be propagated.
                // We guarantee that this locale is supported based on the invariants of this
                // function.
                WindowVariable::Some(translations_str) => self
                    .translations_manager
                    .set_translator_for_translations_str(locale, &translations_str)?,
                // This would be extremely odd...but it's still a problem that could happen (and
                // there *should* be a localized error that the user can see)
                WindowVariable::Malformed => return Err(ClientInvariantError::Translations.into()),
                // There was probably an internal server error
                WindowVariable::None => (),
            };

            Err(err)
        } else {
            match TemplateState::from_str(&state_str) {
                Ok(state) => Ok(state),
                // An actual error means the state was provided, but it was malformed, so we'll
                // render an error page
                Err(_) => Err(ClientInvariantError::InitialState.into()),
            }
        }
    }

    /// Gets the entire contents of the current `<head>`, up to the Perseus
    /// head-end comment (which prevents any JS that was loaded later from
    /// being included). This is not guaranteed to always get exactly the
    /// original head, but it's pretty good, and prevents unnecessary
    /// network requests, while enabling the caching of initially loaded
    /// pages.
    #[cfg(feature = "cache-initial-load")]
    fn get_head() -> Result<String, ClientError> {
        use wasm_bindgen::JsCast;

        let document = web_sys::window().unwrap().document().unwrap();
        // Get the current head
        // The server sends through a head, so we can guarantee that one is present (and
        // it's mandated for custom initial views)
        let head_node = document.query_selector("head").unwrap().unwrap();
        // Get all the elements after the head boundary (otherwise we'd be duplicating
        // the initial stuff)
        let els = head_node
            .query_selector_all(r#"meta[itemprop='__perseus_head_boundary'] ~ *"#)
            .unwrap();
        // No, `NodeList` does not have an iterator implementation...
        let mut head_vec = Vec::new();
        for i in 0..els.length() {
            let elem: Element = els.get(i).unwrap().unchecked_into();
            // Check if this is the delimiter that denotes the end of the head (it's
            // impossible for the user to add anything below here), since we don't
            // want to get anything that other scripts might have added (but if that shows
            // up, it shouldn't be catastrophic)
            if elem.tag_name().to_lowercase() == "meta"
                && elem.get_attribute("itemprop") == Some("__perseus_head_end".to_string())
            {
                break;
            }
            let html = elem.outer_html();
            head_vec.push(html);
        }

        Ok(head_vec.join("\n"))
    }
}

/// A representation of the possible outcomes of getting the view for the
/// initial load.
pub(crate) enum InitialView<'app, G: Html> {
    /// The provided view and scope disposer are ready to render the page.
    View(View<G>, ScopeDisposer<'app>),
    /// We need to redirect somewhere else, and the *full URL* to redirect to is
    /// attached.
    ///
    /// Currently, this is only used by locale redirection, though this could
    /// theoretically also be used for server-level reloads, if those
    /// directives are ever supported.
    Redirect(String),
}
