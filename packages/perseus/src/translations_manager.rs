// This file contains the logic for a universal interface to fecth `Translator` instances for given locales
// At simplest, this is just a filesystem interface, but it might be something like a database in production
// This has its own error management logic because the user may implement it separately

use crate::Translator;
use error_chain::{bail, error_chain};
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

// This has no foreign links because everything to do with config management should be isolated and generic
error_chain! {
    errors {
        /// For when the locale wasn't found. Locales will be checked for existence before fetching is attempted, so this indicates
        /// a bug in the storage system.
        NotFound(locale: String) {
            description("translations not found")
            display("translations for locale '{}' not found", locale)
        }
        /// For when translations couldn't be read for some generic reason.
        ReadFailed(locale: String, err: String) {
            description("translations couldn't be read")
            display("translations for locale '{}' couldn't be read, error was '{}'", locale, err)
        }
        /// For when serializing into the `Translator` data structure failed.
        SerializationFailed(locale: String, err: String) {
            description("translations couldn't be serialized into translator")
            display("translations for locale '{}' couldn't be serialized into translator, error was '{}'", locale, err)
        }
    }
}

/// A trait for systems that manage where to put translations. At simplest, we'll just write them to static files, but they might also
/// be stored in a CMS.
#[async_trait::async_trait]
pub trait TranslationsManager: Clone {
    /// Gets a translator for the given locale.
    async fn get_translator_for_locale(&self, locale: String) -> Result<Translator>;
    /// Gets the translations in stirng format for the given locale (avoids deserialize-then-serialize). This should NOT include the
    /// translator's `locale` property, it should simply be a `HashMap<String, String>` in JSON format.
    async fn get_translations_str_for_locale(&self, locale: String) -> Result<String>;
}

/// The default translations manager. This will store static files in the specified location on disk. This should be suitable for
/// nearly all development and serverful use-cases. Serverless is another matter though (more development needs to be done). This
/// mandates that translations be stored as JSON files named as the locale they describe (e.g. 'en-US.json').
#[derive(Clone)]
pub struct FsTranslationsManager {
    root_path: String,
    /// A map of locales to cached translators. This decreases the number of file reads significantly for the locales specified. This
    /// does NOT cache dynamically, and will only cache the requested locales.
    cached_translators: HashMap<String, Arc<Translator>>,
    /// The locales being cached for easier access.
    cached_locales: Vec<String>
}
// TODO implement caching
impl FsTranslationsManager {
    /// Creates a new filesystem translations manager. You should provide a path like `/translations` here.
    pub fn new(root_path: String) -> Self {
        Self {
            root_path,
            cached_translators: HashMap::new(),
            cached_locales: Vec::new()
        }
    }
}
#[async_trait::async_trait]
impl TranslationsManager for FsTranslationsManager {
    async fn get_translations_str_for_locale(&self, locale: String) -> Result<String> {
        // The file must be named as the locale it describes
        let asset_path = format!("{}/{}.json", self.root_path, locale);
        let translations_str = match fs::metadata(&asset_path) {
            Ok(_) => fs::read_to_string(&asset_path)
                .map_err(|err| ErrorKind::ReadFailed(asset_path, err.to_string()))?,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                bail!(ErrorKind::NotFound(asset_path))
            }
            Err(err) => bail!(ErrorKind::ReadFailed(locale.to_string(), err.to_string())),
        };

        Ok(translations_str)
    }
    async fn get_translator_for_locale(&self, locale: String) -> Result<Translator> {
        let translations_str = self.get_translations_str_for_locale(locale.clone()).await?;
        // We expect the translations defined there, but not the locale itself
        let translations = serde_json::from_str::<HashMap<String, String>>(&translations_str)
            .map_err(|err| ErrorKind::SerializationFailed(locale.clone(), err.to_string()))?;
        let translator = Translator::new(locale, translations);

        Ok(translator)
    }
}
