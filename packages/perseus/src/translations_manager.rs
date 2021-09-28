// This file contains the logic for a universal interface to fecth `Translator` instances for given locales
// At simplest, this is just a filesystem interface, but it might be something like a database in production
// This has its own error management logic because the user may implement it separately

use thiserror::Error;

/// Errors that can occur in a translations manager.
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum TranslationsManagerError {
    #[error("translations not found for locale '{locale}'")]
    NotFound { locale: String },
    #[error("translations for locale '{locale}' couldn't be read")]
    ReadFailed {
        locale: String,
        #[source]
        source: Box<dyn std::error::Error>,
    },
    #[error("translations for locale '{locale}' couldn't be serialized into translator")]
    SerializationFailed {
        locale: String,
        #[source]
        source: Box<dyn std::error::Error>,
    },
}

use crate::Translator;
use futures::future::join_all;
use std::collections::HashMap;
use std::fs;

/// A trait for systems that manage where to put translations. At simplest, we'll just write them to static files, but they might also
/// be stored in a CMS. It is **strongly** advised that any implementations use some form of caching, guided by `FsTranslationsManager`.
#[async_trait::async_trait]
pub trait TranslationsManager: Clone {
    /// Gets a translator for the given locale.
    async fn get_translator_for_locale(
        &self,
        locale: String,
    ) -> Result<Translator, TranslationsManagerError>;
    /// Gets the translations in string format for the given locale (avoids deserialize-then-serialize).
    async fn get_translations_str_for_locale(
        &self,
        locale: String,
    ) -> Result<String, TranslationsManagerError>;
}

/// A utility function for allowing parallel futures execution. This returns a tuple of the locale and the translations as a JSON string.
async fn get_translations_str_and_cache(
    locale: String,
    manager: &FsTranslationsManager,
) -> (String, String) {
    let translations_str = manager
        .get_translations_str_for_locale(locale.to_string())
        .await
        .unwrap_or_else(|_| {
            panic!(
                "translations for locale to be cached '{}' couldn't be loaded",
                locale
            )
        });

    (locale, translations_str)
}

/// The default translations manager. This will store static files in the specified location on disk. This should be suitable for
/// nearly all development and serverful use-cases. Serverless is another matter though (more development needs to be done). This
/// mandates that translations be stored as files named as the locale they describe (e.g. 'en-US.ftl', 'en-US.json', etc.).
#[derive(Clone)]
pub struct FsTranslationsManager {
    root_path: String,
    /// A map of locales to cached translations. This decreases the number of file reads significantly for the locales specified. This
    /// does NOT cache dynamically, and will only cache the requested locales. Translators can be created when necessary from these.
    cached_translations: HashMap<String, String>,
    /// The locales being cached for easier access.
    cached_locales: Vec<String>,
    /// The file extension expected (e.g. JSON, FTL, etc). This allows for greater flexibility of translation engines (future).
    file_ext: String,
}
impl FsTranslationsManager {
    /// Creates a new filesystem translations manager. You should provide a path like `/translations` here. You should also provide
    /// the locales you want to cache, which will have their translations stored in memory. Any supported locales not specified here
    /// will not be cached, and must have their translations read from disk on every request. If fetching translations for any of the
    /// given locales fails, this will panic (locales to be cached should always be hardcoded).
    // TODO performance analysis of manual caching strategy
    pub async fn new(root_path: String, locales_to_cache: Vec<String>, file_ext: String) -> Self {
        // Initialize a new instance without any caching first
        let mut manager = Self {
            root_path,
            cached_translations: HashMap::new(),
            cached_locales: Vec::new(),
            file_ext,
        };
        // Now use that to get the translations for the locales we want to cache (all done in parallel)
        let mut futs = Vec::new();
        for locale in &locales_to_cache {
            futs.push(get_translations_str_and_cache(locale.to_string(), &manager));
        }
        let cached_translations_kv_vec = join_all(futs).await;
        manager.cached_translations = cached_translations_kv_vec.iter().cloned().collect();
        // We only declare the locales that are being cached after getting translations becuase otherwise those getters would be using undefined caches
        manager.cached_locales = locales_to_cache;

        manager
    }
}
#[async_trait::async_trait]
impl TranslationsManager for FsTranslationsManager {
    async fn get_translations_str_for_locale(
        &self,
        locale: String,
    ) -> Result<String, TranslationsManagerError> {
        // Check if the locale is cached for
        // No dynamic caching, so if it isn't cached it stays that way
        if self.cached_locales.contains(&locale) {
            Ok(self.cached_translations.get(&locale).unwrap().to_string())
        } else {
            // The file must be named as the locale it describes
            let asset_path = format!("{}/{}.{}", self.root_path, locale, self.file_ext);
            let translations_str = match fs::metadata(&asset_path) {
                Ok(_) => fs::read_to_string(&asset_path).map_err(|err| {
                    TranslationsManagerError::ReadFailed {
                        locale: locale.clone(),
                        source: err.into(),
                    }
                })?,
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                    return Err(TranslationsManagerError::NotFound {
                        locale: locale.clone(),
                    })
                }
                Err(err) => {
                    return Err(TranslationsManagerError::ReadFailed {
                        locale: locale.clone(),
                        source: err.into(),
                    })
                }
            };
            Ok(translations_str)
        }
    }
    async fn get_translator_for_locale(
        &self,
        locale: String,
    ) -> Result<Translator, TranslationsManagerError> {
        // Check if the locale is cached for
        // No dynamic caching, so if it isn't cached it stays that way
        let translations_str;
        if self.cached_locales.contains(&locale) {
            translations_str = self.cached_translations.get(&locale).unwrap().to_string();
        } else {
            translations_str = self.get_translations_str_for_locale(locale.clone()).await?;
        }
        // We expect the translations defined there, but not the locale itself
        let translator = Translator::new(locale.clone(), translations_str).map_err(|err| {
            TranslationsManagerError::SerializationFailed {
                locale: locale.clone(),
                source: err.into(),
            }
        })?;

        Ok(translator)
    }
}

/// A dummy translations manager for use if you don't want i18n. This avoids errors of not being able to find translations. If you set
/// `no_i18n: true` in the `locales` section of `define_app!`, this will be used by default. If you intend to use i18n, do not use this!
#[derive(Clone, Default)]
pub struct DummyTranslationsManager;
impl DummyTranslationsManager {
    /// Creates a new dummy translations manager.
    pub fn new() -> Self {
        Self::default()
    }
}
#[async_trait::async_trait]
impl TranslationsManager for DummyTranslationsManager {
    async fn get_translations_str_for_locale(
        &self,
        _locale: String,
    ) -> Result<String, TranslationsManagerError> {
        Ok(String::new())
    }
    async fn get_translator_for_locale(
        &self,
        locale: String,
    ) -> Result<Translator, TranslationsManagerError> {
        let translator = Translator::new(locale.clone(), String::new()).map_err(|err| {
            TranslationsManagerError::SerializationFailed {
                locale,
                source: err.into(),
            }
        })?;

        Ok(translator)
    }
}
