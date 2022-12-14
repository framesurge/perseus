use serde_json::Value;
use sycamore::{
    prelude::{Scope, ScopeDisposer},
    view::View,
    web::Html,
};

use crate::{
    checkpoint,
    errors::{AssetType, ClientError, ClientInvariantError},
    i18n::detect_locale,
    page_data::PageDataPartial,
    path::PathMaybeWithLocale,
    router::{RouteInfo, RouteVerdict, RouterLoadState},
    state::{PssContains, TemplateState},
    utils::{fetch, get_path_prefix_client, replace_head},
};

use super::Reactor;

impl<G: Html> Reactor<G> {
    /// Gets the subsequent view, based on the given verdict.
    ///
    /// Note that 'server errors' as constructed by this function are
    /// constructed here, not deserialized from provided data.
    ///
    /// # Panics
    /// This function will panic on a locale redirection if a router has not
    /// been created on the given scope.
    pub(crate) async fn get_subsequent_view<'a>(
        &self,
        cx: Scope<'a>,
        verdict: RouteVerdict<G>,
    ) -> Result<(View<G>, ScopeDisposer<'a>), ClientError> {
        checkpoint("router_entry");

        match &verdict {
            RouteVerdict::Found(RouteInfo {
                path,
                template,
                locale,
                was_incremental_match,
            }) => {
                let full_path = PathMaybeWithLocale::new(&path, &locale);
                // Update the router state
                self.router_state.set_load_state(RouterLoadState::Loading {
                    template_name: template.get_path(),
                    path: full_path.clone(),
                });
                self.router_state.set_last_verdict(verdict.clone());

                checkpoint("initial_state_not_present");

                // Before we fetch anything, first check if there's an entry in the PSS already
                // (if there is, we can avoid a network request)
                let page_data = match self.state_store.contains(&full_path) {
                    // We only have one part of the puzzle (or nothing at all), and no guarantee
                    // that the other doesn't exist, so we'll have to check with
                    // the server to be safe. Remember that this function
                    // can't be used with widgets!
                    PssContains::State | PssContains::Head | PssContains::None => {
                        // Get the static page data (head and state)
                        let asset_url = format!(
                            "{}/.perseus/page/{}/{}.json?entity_name={}&was_incremental_match={}",
                            get_path_prefix_client(),
                            locale,
                            **path,
                            template.get_path(),
                            was_incremental_match
                        );
                        // If this doesn't exist, then it's a 404 (we went here by explicit
                        // navigation, but it may be an unservable ISR page
                        // or the like)
                        let page_data_str = fetch(&asset_url, AssetType::Page).await?;
                        match &page_data_str {
                            Some(page_data_str) => {
                                // All good, deserialize the page data
                                let page_data =
                                    serde_json::from_str::<PageDataPartial>(&page_data_str);
                                match page_data {
                                    Ok(page_data) => {
                                        // Add the head to the PSS for future use (we make
                                        // absolutely no
                                        // assumptions about state and leave that to the macros)
                                        self.state_store.add_head(
                                            &full_path,
                                            page_data.head.to_string(),
                                            false,
                                        );
                                        page_data
                                    }
                                    // If the page failed to serialize, it's a server error
                                    Err(err) => {
                                        return Err(ClientInvariantError::InvalidState {
                                            source: err,
                                        }
                                        .into())
                                    }
                                }
                            }
                            // This indicates the fetch found a 404 (any other errors were
                            // propagated by `?`)
                            None => {
                                return Err(ClientError::ServerError {
                                    status: 404,
                                    message: "page not found".to_string(),
                                })
                            }
                        }
                    }
                    // We have everything locally, so we can move right ahead!
                    PssContains::All => PageDataPartial {
                        // This will never be parsed, because the template closures use the active
                        // state preferentially, whose existence we verified
                        // by getting here
                        state: Value::Null,
                        head: self.state_store.get_head(&full_path).unwrap(),
                    },
                    // We only have document metadata, but the page definitely takes no state, so
                    // we're fine
                    PssContains::HeadNoState => PageDataPartial {
                        state: Value::Null,
                        head: self.state_store.get_head(&full_path).unwrap(),
                    },
                    // The page's data has been preloaded at some other time
                    PssContains::Preloaded => {
                        let page_data = self.state_store.get_preloaded(&full_path).unwrap();
                        // Register the head, otherwise it will never be registered and the page
                        // will never properly show up in the PSS (meaning
                        // future preload calls will go through, creating
                        // unnecessary network requests)
                        self.state_store
                            .add_head(&full_path, page_data.head.to_string(), false);
                        page_data
                    }
                };
                // Interpolate the metadata directly into the document's `<head>`
                replace_head(&page_data.head);

                // Now update the translator (this will do nothing if the user hasn't switched
                // locales). Importantly, if this returns an error, the error
                // views will almost certainly get the old translator. Because
                // this will be registered as an internal error as well,
                // that means we'll probably get a popup, which is much better UX than an error
                // page on `/fr-FR/foo` in Spanish.
                self.translations_manager
                    .set_translator_for_locale(&locale)
                    .await?;

                let template_name = template.get_path();
                // Pre-emptively update the router state
                checkpoint("page_interactive");
                // Update the router state
                self.router_state.set_load_state(RouterLoadState::Loaded {
                    template_name,
                    path: full_path.clone(),
                });
                // Now return the view that should be rendered
                let (view, disposer) = template.render_for_template_client(
                    full_path,
                    TemplateState::from_value(page_data.state),
                    cx,
                )?;

                Ok((view, disposer))
            }
            // For subsequent loads, this should only be possible if the dev forgot `link!()`
            // TODO Debug assertion that this doesn't happen perhaps?
            RouteVerdict::LocaleDetection(path) => {
                let dest = detect_locale(path.clone(), &self.locales);
                // Since this is only for subsequent loads, we know the router is instantiated
                // This shouldn't be a replacement navigation, since the user has deliberately
                // navigated here
                sycamore_router::navigate(&dest);
                todo!()
            }
            RouteVerdict::NotFound { locale } => {
                checkpoint("not_found");

                // Neatly return a `ClientError::ServerError`, which will be displayed somehow
                // by the caller (hopefully as a full-page view, but that will depend on the
                // user's error view logic)
                Err(ClientError::ServerError {
                    status: 404,
                    message: "page not found".to_string(),
                })
            }
        }
    }
}
