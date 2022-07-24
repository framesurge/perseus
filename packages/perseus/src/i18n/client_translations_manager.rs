use super::Locales;
use crate::errors::*;
use crate::i18n::Translator;
use crate::shell::fetch;
use crate::utils::get_path_prefix_client;
use std::cell::RefCell;
use std::rc::Rc;

/// Manages translations in the app shell. This handles fetching translations
/// from the server as well as caching for performance. This is distinct from
/// `TranslationsManager` in that it operates on the client-side rather than on
/// the server. This optimizes for users viewing many pages in the same locale,
/// which is by far the most common use of most websites in terms of i18n.
///
/// This holds mutability internally to avoid issues with async/await.
#[derive(Debug, Clone)]
pub(crate) struct ClientTranslationsManager {
    /// The cached translator. If the same locale is requested again, this will
    /// simply be returned.
    cached_translator: Rc<RefCell<Option<Translator>>>,
    locales: Locales,
}
impl ClientTranslationsManager {
    /// Creates a new client-side translations manager that hasn't cached
    /// anything yet. This needs to know about an app's supported locales so
    /// it can avoid network requests to unsupported locales.
    pub fn new(locales: &Locales) -> Self {
        Self {
            cached_translator: Rc::new(RefCell::new(None)),
            locales: locales.clone(),
        }
    }
    /// Gets an `&'static Translator` for the given locale. This will use the
    /// internally cached `Translator` if possible, and will otherwise fetch
    /// the translations from the server. This manages mutability for caching
    /// internally.
    pub async fn get_translator_for_locale<'a>(
        &'a self,
        locale: &'a str,
    ) -> Result<Translator, ClientError> {
        let path_prefix = get_path_prefix_client();
        // Check if we've already cached
        let mut cached_translator = self.cached_translator.borrow_mut();
        if cached_translator.is_some() && cached_translator.as_ref().unwrap().get_locale() == locale
        {
            Ok(cached_translator.as_ref().unwrap().clone())
        } else {
            // Check if the locale is supported and we're actually using i18n
            if self.locales.is_supported(locale) && self.locales.using_i18n {
                // Get the translations data
                let asset_url = format!("{}/.perseus/translations/{}", path_prefix, locale);
                // If this doesn't exist, then it's a 404 (we went here by explicit navigation
                // after checking the locale, so that's a bug)
                let translations_str = fetch(&asset_url).await;
                let translator = match translations_str {
                    Ok(translations_str) => match translations_str {
                        Some(translations_str) => {
                            // All good, turn the translations into a translator
                            match Translator::new(locale.to_string(), translations_str) {
                                Ok(translator) => translator,
                                Err(err) => {
                                    return Err(FetchError::SerFailed {
                                        url: asset_url,
                                        source: err.into(),
                                    }
                                    .into())
                                }
                            }
                        }
                        // If we get a 404 for a supported locale, that's an exception
                        None => panic!(
                            "server returned 404 for translations for known supported locale '{}'",
                            locale
                        ),
                    },
                    Err(err) => match err {
                        not_ok_err @ ClientError::FetchError(FetchError::NotOk { .. }) => {
                            return Err(not_ok_err)
                        }
                        // No other errors should be returned
                        _ => panic!("expected 'AssetNotOk' error, found other unacceptable error"),
                    },
                };
                // Cache that translator
                *cached_translator = Some(translator);
                // Now return that
                Ok(cached_translator.as_ref().unwrap().clone())
            } else if !self.locales.using_i18n {
                // If we aren't even using i18n, then it would be pointless to fetch
                // translations
                let translator = Translator::new("xx-XX".to_string(), "".to_string()).unwrap();
                // Cache that translator
                *cached_translator = Some(translator);
                // Now return that
                Ok(cached_translator.as_ref().unwrap().clone())
            } else {
                Err(ClientError::LocaleNotSupported {
                    locale: locale.to_string(),
                })
            }
        }
    }
}
