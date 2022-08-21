use crate::error_pages::ErrorPages;
use crate::errors::*;
use crate::i18n::ClientTranslationsManager;
use crate::page_data::PageDataPartial;
use crate::router::{get_global_state, RouteVerdict, RouterLoadState, RouterState};
use crate::template::{PageProps, Template, TemplateNodeType};
use crate::utils::checkpoint;
use crate::utils::fetch;
use crate::utils::get_path_prefix_client;
use crate::utils::replace_head;
use fmterr::fmt_err;
use std::rc::Rc;
use sycamore::prelude::*;

pub(crate) struct GetSubsequentViewProps<'a> {
    /// The app's reactive scope.
    pub cx: Scope<'a>,
    /// The path we're rendering for (not the template path, the full path,
    /// though parsed a little).
    pub path: String,
    /// The template to render for.
    pub template: Rc<Template<TemplateNodeType>>,
    /// Whether or not the router returned an incremental match (if this page
    /// exists on a template using incremental generation and it wasn't defined
    /// at build time).
    pub was_incremental_match: bool,
    /// The locale we're rendering in.
    pub locale: String,
    /// The router state.
    pub router_state: RouterState,
    /// A *client-side* translations manager to use (this manages caching
    /// translations).
    pub translations_manager: ClientTranslationsManager,
    /// The error pages, for use if something fails.
    pub error_pages: Rc<ErrorPages<TemplateNodeType>>,
    /// The current route verdict. This will be stored in context so that it can
    /// be used for possible reloads. Eventually, this will be made obsolete
    /// when Sycamore supports this natively.
    pub route_verdict: RouteVerdict<TemplateNodeType>,
}

/// Gets the view to render on a change of route after the app has already
/// loaded. This involves network requests to determine the state of the page,
/// which is then used to render directly. We don't need to request the HTML,
/// since that would just take longer, and we have everything we need to render
/// it. We also won't be hydrating anything, so there's no point in getting the
/// HTML, it actually slows page transitions down.
///
/// Note that this will automatically update the router state just before it
/// returns, meaning that any errors that may occur after this function has been
/// called need to reset the router state to be an error.
// TODO Eliminate all panics in this function
pub(crate) async fn get_subsequent_view(
    GetSubsequentViewProps {
        cx,
        path,
        template,
        was_incremental_match,
        locale,
        mut router_state,
        translations_manager,
        error_pages,
        route_verdict,
    }: GetSubsequentViewProps<'_>,
) -> View<TemplateNodeType> {
    let path_with_locale = match locale.as_str() {
        "xx-XX" => path.clone(),
        locale => format!("{}/{}", locale, &path),
    };
    // Update the router state
    router_state.set_load_state(RouterLoadState::Loading {
        template_name: template.get_path(),
        path: path_with_locale.clone(),
    });
    router_state.set_last_verdict(route_verdict.clone());

    checkpoint("initial_state_not_present");
    // If we're getting data about the index page, explicitly set it to that
    // This can be handled by the Perseus server (and is), but not by static
    // exporting
    let path = match path.is_empty() {
        true => "index".to_string(),
        false => path,
    };
    // Get the static page data
    // TODO Only get the page state here
    let asset_url = format!(
        "{}/.perseus/page/{}/{}.json?template_name={}&was_incremental_match={}",
        get_path_prefix_client(),
        locale,
        path,
        template.get_path(),
        was_incremental_match
    );
    // If this doesn't exist, then it's a 404 (we went here by explicit navigation,
    // but it may be an unservable ISR page or the like)
    let page_data_str = fetch(&asset_url).await;
    match &page_data_str {
        Ok(page_data_str_opt) => match page_data_str_opt {
            Some(page_data_str) => {
                // All good, deserialize the page data
                let page_data = serde_json::from_str::<PageDataPartial>(&page_data_str);
                match page_data {
                    Ok(page_data) => {
                        // Interpolate the metadata directly into the document's `<head>`
                        replace_head(&page_data.head);

                        // Now get the translator (this will be cached if the user hasn't switched
                        // locales)
                        let translator = translations_manager
                            .get_translator_for_locale(&locale)
                            .await;
                        let translator = match translator {
                            Ok(translator) => translator,
                            Err(err) => {
                                router_state.set_load_state(RouterLoadState::ErrorLoaded {
                                    path: path_with_locale.clone(),
                                });
                                match &err {
                                    // These errors happen because we couldn't get a translator, so they certainly don't get one
                                    ClientError::FetchError(FetchError::NotOk { url, status, .. }) => return error_pages.get_view_and_render_head(cx, url, *status, &fmt_err(&err), None),
                                    ClientError::FetchError(FetchError::SerFailed { url, .. }) => return error_pages.get_view_and_render_head(cx, url, 500, &fmt_err(&err), None),
                                    ClientError::LocaleNotSupported { locale } => return error_pages.get_view_and_render_head(cx, &format!("/{}/...", locale), 404, &fmt_err(&err), None),
                                    // No other errors should be returned
                                    _ => panic!("expected 'AssetNotOk'/'AssetSerFailed'/'LocaleNotSupported' error, found other unacceptable error")
                                }
                            }
                        };

                        let page_props = PageProps {
                            path: path_with_locale.clone(),
                            state: page_data.state,
                            // This will probably be overriden by the already-set version (unless no
                            // page has used global state yet)
                            global_state: get_global_state(),
                        };
                        let template_name = template.get_path();
                        // Pre-emptively update the router state
                        checkpoint("page_interactive");
                        // Update the router state
                        router_state.set_load_state(RouterLoadState::Loaded {
                            template_name,
                            path: path_with_locale,
                        });
                        // Now return the view that should be rendered
                        template.render_for_template_client(page_props, cx, translator)
                    }
                    // If the page failed to serialize, an exception has occurred
                    Err(err) => {
                        router_state.set_load_state(RouterLoadState::ErrorLoaded {
                            path: path_with_locale.clone(),
                        });
                        panic!("page data couldn't be serialized: '{}'", err)
                    }
                }
            }
            // No translators ready yet
            None => {
                router_state.set_load_state(RouterLoadState::ErrorLoaded {
                    path: path_with_locale.clone(),
                });
                error_pages.get_view_and_render_head(cx, &asset_url, 404, "page not found", None)
            }
        },
        Err(err) => {
            router_state.set_load_state(RouterLoadState::ErrorLoaded {
                path: path_with_locale.clone(),
            });
            match &err {
                // No translators ready yet
                ClientError::FetchError(FetchError::NotOk { url, status, .. }) => {
                    error_pages.get_view_and_render_head(cx, url, *status, &fmt_err(&err), None)
                }
                // No other errors should be returned
                _ => panic!("expected 'AssetNotOk' error, found other unacceptable error"),
            }
        }
    }
}
