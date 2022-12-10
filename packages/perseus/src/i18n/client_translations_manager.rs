use super::Locales;
use crate::errors::*;
use crate::i18n::Translator;
use crate::utils::fetch;
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
    pub(crate) fn new(locales: &Locales) -> Self {
        Self {
            cached_translator: Rc::new(RefCell::new(None)),
            locales: locales.clone(),
        }
    }
    /// Gets the currently cached translator. This is designed to be used as a
    /// backend for a method that works with both this and the engine-side-provided
    /// translator.
    pub(crate) fn get_translator(&self) -> Option<Translator> {
        self.cached_translator.borrow().as_ref().cloned()
    }
    /// An internal preflight check performed before getting a translator. This
    /// consists of making sure the locale is supported, and that the app is
    /// actually using i18n. If i18n is not being used, then this will
    /// return a dummy translator on its own, and no further computation should
    /// be performed. If you need to fetch a translator after calling this, then
    /// you should be sure to cache it.
    ///
    /// This will return `false` if the caller needs to take no further action to
    /// set this translator, or `true` if it shoudl go ahead.
    fn preflight_check(&self, locale: &str) -> Result<bool, ClientError> {
        // Check if we've already cached
        let mut cached_translator = self.cached_translator.borrow_mut();
        if cached_translator.is_some() && cached_translator.as_ref().unwrap().get_locale() == locale
        {
            Ok(false)
        } else {
            // Check if the locale is supported and we're actually using i18n
            if self.locales.is_supported(locale) && self.locales.using_i18n {
                // We're clear to fetch a translator for this locale
                Ok(true)
            } else if !self.locales.using_i18n {
                // If we aren't even using i18n, then it would be pointless to fetch
                // translations
                let translator = Translator::new("xx-XX".to_string(), "".to_string()).unwrap();
                // Cache that translator
                *cached_translator = Some(translator);
                // Now return that
                Ok(false)
            } else {
                Err(ClientError::LocaleNotSupported {
                    locale: locale.to_string(),
                })
            }
        }
    }
    /// Caches the given translator internally for future use without needing to
    /// make network requests.
    fn cache_translator(&self, translator: Translator) {
        *self.cached_translator.borrow_mut() = Some(translator);
    }

    /// Caches a `Translator` for the given locale, using the given translations
    /// string. This is intended to be used when fetching the translations
    /// string from the window variable provided by the server for initial
    /// loads.
    pub(crate) fn set_translator_for_translations_str(
        &self,
        locale: &str,
        translations_str: &str,
    ) -> Result<(), ClientError> {
        if self.preflight_check(locale)? {
            let translator =
                match Translator::new(locale.to_string(), translations_str.to_string()) {
                    Ok(translator) => translator,
                    Err(err) => {
                        return Err(FetchError::SerFailed {
                            url: "*".to_string(),
                            source: err.into(),
                            ty: AssetType::Translations,
                        }
                                   .into())
                    }
                };
            self.cache_translator(translator);
        }

        Ok(())
    }
    /// Caches a `Translator` for the given locale. This will use the
    /// internally cached `Translator` if possible, and will otherwise fetch
    /// the translations from the server. This manages mutability for caching
    /// internally.
    pub(crate) async fn set_translator_for_locale<'a>(
        &'a self,
        locale: &'a str,
    ) -> Result<Translator, ClientError> {
        if self.preflight_check(locale)? {
            let path_prefix = get_path_prefix_client();
            // Get the translations data
            let asset_url = format!("{}/.perseus/translations/{}", path_prefix, locale);
            // If this doesn't exist, then it's a 404 (we went here by explicit navigation
            // after checking the locale, so that's a bug)
            let translations_str = fetch(&asset_url, AssetType::Translations).await?;
            let translator = match translations_str {
                Some(translations_str) => {
                    // All good, turn the translations into a translator
                    self.get_translator_for_translations_str(locale, &translations_str)?
                }
                // If we get a 404 for a supported locale, that's an exception
                None => {
                    return Err(ClientInvariantError::ValidLocaleNotProvided {
                        locale: locale.to_string(),
                    })
                }
            };
            self.cache_translator(translator)
        }

        Ok(())
    }
}
