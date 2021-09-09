use crate::errors::*;
use crate::shell::fetch;
use crate::Locales;
use crate::Translator;
use std::rc::Rc;

/// Manages translations in the app shell. This handles fetching translations from the server as well as caching for performance.
/// This is distinct from `TranslationsManager` in that it operates on the client-side rather than on the server. This optimizes for
/// users viewing many pages in the same locale, which is by far the most common use of most websites in terms of i18n.
pub struct ClientTranslationsManager {
    /// The cached translator. If the same locale is requested again, this will simply be returned.
    cached_translator: Option<Rc<Translator>>,
    locales: Locales,
}
impl ClientTranslationsManager {
    /// Creates a new client-side translations manager that hasn't cached anything yet. This needs to know about an app's supported locales
    /// so it can avoid network requests to unsupported locales.
    pub fn new(locales: &Locales) -> Self {
        Self {
            cached_translator: None,
            locales: locales.clone(),
        }
    }
    /// Gets an `Rc<Translator>` for the given locale. This will use the internally cached `Translator` if possible, and will otherwise
    /// fetch the translations from the server. This needs mutability because it will modify its internal cache if necessary.
    pub async fn get_translator_for_locale(&mut self, locale: &str) -> Result<Rc<Translator>> {
        // Check if we've already cached
        if self.cached_translator.is_some()
            && self.cached_translator.as_ref().unwrap().get_locale() == locale
        {
            Ok(Rc::clone(self.cached_translator.as_ref().unwrap()))
        } else {
            // Check if the locale is supported
            if self.locales.is_supported(locale) {
                // Get the translations data
                let asset_url = format!("/.perseus/translations/{}", locale);
                // If this doesn't exist, then it's a 404 (we went here by explicit navigation after checking the locale, so that's a bug)
                let translations_str = fetch(&asset_url).await;
                let translator = match translations_str {
                    Ok(translations_str) => match translations_str {
                        Some(translations_str) => {
                            // All good, turn the translations into a translator
                            let translator = Translator::new(locale.to_string(), translations_str);
                            match translator {
                                Ok(translator) => translator,
                                Err(err) => {
                                    bail!(ErrorKind::AssetSerFailed(asset_url, err.to_string()))
                                }
                            }
                        }
                        // If we get a 404 for a supported locale, that's an exception
                        None => panic!(
                            "server returned 404 for translations for known supported locale '{}'",
                            locale
                        ),
                    },
                    Err(err) => match err.kind() {
                        ErrorKind::AssetNotOk(url, status, err) => bail!(ErrorKind::AssetNotOk(
                            url.to_string(),
                            *status,
                            err.to_string()
                        )),
                        // No other errors should be returned
                        _ => panic!("expected 'AssetNotOk' error, found other unacceptable error"),
                    },
                };
                // Cache that translator
                self.cached_translator = Some(Rc::new(translator));
                // Now return that
                Ok(Rc::clone(self.cached_translator.as_ref().unwrap()))
            } else {
                bail!(ErrorKind::LocaleNotSupported(locale.to_string()))
            }
        }
    }
}
