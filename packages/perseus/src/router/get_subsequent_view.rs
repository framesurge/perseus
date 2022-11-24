use crate::errors::*;
use crate::page_data::PageDataPartial;
use crate::router::{RouteManager, RouteVerdict, RouterLoadState};
use crate::state::PssContains;
use crate::template::{RenderCtx, Template, TemplateNodeType, TemplateState};
use crate::utils::checkpoint;
use crate::utils::fetch;
use crate::utils::get_path_prefix_client;
use crate::utils::replace_head;
use fmterr::fmt_err;
use std::rc::Rc;
use sycamore::prelude::*;
use serde_json::Value;

pub(crate) struct GetSubsequentViewProps<'a> {
    /// The app's reactive scope.
    pub cx: Scope<'a>,
    /// The app's route manager.
    pub route_manager: &'a RouteManager<'a, TemplateNodeType>,
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
pub(crate) async fn get_subsequent_view(
    GetSubsequentViewProps {
        cx,
        route_manager,
        path,
        template,
        was_incremental_match,
        locale,
        route_verdict,
    }: GetSubsequentViewProps<'_>,
) -> SubsequentView {
    let render_ctx = RenderCtx::from_ctx(cx);
    let router_state = &render_ctx.router;
    let translations_manager = &render_ctx.translations_manager;
    let error_pages = &render_ctx.error_pages;
    let pss = &render_ctx.page_state_store;

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
    // Before we fetch anything, first check if there's an entry in the PSS already
    // (if there is, we can avoid a network request)
    let page_data: Result<PageDataPartial, View<TemplateNodeType>> = match pss.contains(&path) {
        // We only have one part of the puzzle (or nothing at all), and no guarantee that the other
        // doesn't exist, so we'll have to check with the server to be safe
        PssContains::State | PssContains::Head | PssContains::None => {
            // // If we're getting data about the index page, explicitly set it to that
            // // This can be handled by the Perseus server (and is), but not by static
            // // exporting
            // let path_norm = match path.is_empty() {
            //     true => "index".to_string(),
            //     false => path.to_string(),
            // };
            let path_norm = path.to_string();
            // Get the static page data (head and state)
            let asset_url = format!(
                "{}/.perseus/page/{}/{}.json?template_name={}&was_incremental_match={}",
                get_path_prefix_client(),
                locale,
                path_norm,
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
                                // Add the head to the PSS for future use (we make absolutely no
                                // assumptions about state and leave that to the macros)
                                pss.add_head(&path, page_data.head.to_string());
                                Ok(page_data)
                            }
                            // If the page failed to serialize, it's a server error
                            Err(err) => {
                                router_state.set_load_state(RouterLoadState::ErrorLoaded {
                                    path: path_with_locale.clone(),
                                });
                                Err(error_pages.get_view_and_render_head(
                                    cx,
                                    &asset_url,
                                    500,
                                    &fmt_err(&err),
                                    None,
                                ))
                            }
                        }
                    }
                    // No translators ready yet
                    None => {
                        router_state.set_load_state(RouterLoadState::ErrorLoaded {
                            path: path_with_locale.clone(),
                        });
                        Err(error_pages.get_view_and_render_head(
                            cx,
                            &asset_url,
                            404,
                            "page not found",
                            None,
                        ))
                    }
                },
                Err(err) => {
                    router_state.set_load_state(RouterLoadState::ErrorLoaded {
                        path: path_with_locale.clone(),
                    });
                    match &err {
                        // No translators ready yet
                        ClientError::FetchError(FetchError::NotOk { url, status, .. }) => {
                            Err(error_pages.get_view_and_render_head(
                                cx,
                                url,
                                *status,
                                &fmt_err(&err),
                                None,
                            ))
                        }
                        // No other errors should be returned, but we'll give any a 400
                        _ => Err(error_pages.get_view_and_render_head(
                            cx,
                            &asset_url,
                            400,
                            &fmt_err(&err),
                            None,
                        )),
                    }
                }
            }
        }
        // We have everything locally, so we can move right ahead!
        PssContains::All => Ok(PageDataPartial {
            state: Value::Null, /* The macros will preferentially use the PSS state,
                                             * so this will never be parsed */
            head: pss.get_head(&path).unwrap(),
        }),
        // We only have document metadata, but the page definitely takes no state, so we're fine
        PssContains::HeadNoState => Ok(PageDataPartial {
            state: Value::Null,
            head: pss.get_head(&path).unwrap(),
        }),
        // The page's data has been preloaded at some other time
        PssContains::Preloaded => {
            let page_data = pss.get_preloaded(&path).unwrap();
            // Register the head, otherwise it will never be registered and the page will
            // never properly show up in the PSS (meaning future preload
            // calls will go through, creating unnecessary network requests)
            pss.add_head(&path, page_data.head.to_string());
            Ok(page_data)
        }
    };
    // Any errors will be prepared error pages ready for return
    let page_data = match page_data {
        Ok(page_data) => page_data,
        Err(view) => return SubsequentView::Error(view),
    };

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
                // These errors happen because we couldn't get a translator, so they certainly don't
                // get one
                ClientError::FetchError(FetchError::NotOk { url, status, .. }) => {
                    return SubsequentView::Error(error_pages.get_view_and_render_head(
                        cx,
                        url,
                        *status,
                        &fmt_err(&err),
                        None,
                    ))
                }
                ClientError::FetchError(FetchError::SerFailed { url, .. }) => {
                    return SubsequentView::Error(error_pages.get_view_and_render_head(
                        cx,
                        url,
                        500,
                        &fmt_err(&err),
                        None,
                    ))
                }
                ClientError::LocaleNotSupported { locale } => {
                    return SubsequentView::Error(error_pages.get_view_and_render_head(
                        cx,
                        &format!("/{}/...", locale),
                        404,
                        &fmt_err(&err),
                        None,
                    ))
                }
                // No other errors should be returned, but we'll give any a 400
                _ => {
                    return SubsequentView::Error(error_pages.get_view_and_render_head(
                        cx,
                        &format!("/{}/...", locale),
                        400,
                        &fmt_err(&err),
                        None,
                    ))
                }
            }
        }
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
    template.render_for_template_client(TemplateState::from_value(page_data.state), cx, route_manager, translator);
    SubsequentView::Success
}

pub(crate) enum SubsequentView {
    /// The page view *has been* rendered *and* displayed.
    Success,
    /// An error view was rendered, and shoudl now be displayed.
    Error(View<TemplateNodeType>),
}
