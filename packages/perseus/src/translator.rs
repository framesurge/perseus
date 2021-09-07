use crate::errors::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Defines app information about i18n, specifically about which locales are supported.
#[derive(Clone)]
pub struct Locales {
    /// The default locale, which will be used as a fallback if the user's locale can't be extracted. This will be built for at build-time.
    pub default: String,
    /// Common locales that should be built for at build-time.
    pub common: Vec<String>,
    /// All other supported locales, which won't be built unless requested at request-time.
    pub other: Vec<String>,
}
impl Locales {
    /// Gets all the supported locales by combining the default, common, and other.
    pub fn get_all(&self) -> Vec<&String> {
        let mut vec: Vec<&String> = vec![&self.default];
        vec.extend(&self.common);
        vec.extend(&self.other);

        vec
    }
    /// Gets the locales that should be built at build time, the default and common.
    pub fn get_default_and_common(&self) -> Vec<&String> {
        let mut vec: Vec<&String> = vec![&self.default];
        vec.extend(&self.common);

        vec
    }
    /// Checks if the given locale is supported.
    pub fn is_supported(&self, locale: &str) -> bool {
        let locales = self.get_all();
        locales.iter().any(|l| *l == locale)
    }
}

/// Manages translations on the client-side for a single locale. This should generally be placed into an `Rc<T>` and referred to by
/// every template in an app. You do NOT want to be cloning potentially thousands of translations!
#[derive(Serialize, Deserialize)]
pub struct Translator {
    /// Stores a map of translation IDs to actual translations for the current locale.
    translations: HashMap<String, String>,
    /// The locale for which translations are being managed by this instance.
    pub locale: String,
}
// TODO support variables, plurals, etc.
impl Translator {
    /// Creates a new instance of the translator for the given locale.
    pub fn new(locale: String, translations: HashMap<String, String>) -> Self {
        Self {
            translations,
            locale,
        }
    }
    /// Creates an empty translator that doesn't do anything. This is instantiated on the server in particular. This should NEVER be
    /// used on the client-side! This won't allocate translations or a locale.
    pub fn empty() -> Self {
        Self {
            translations: HashMap::new(),
            locale: String::new(),
        }
    }
    /// Gets the translation of the given ID for the current locale.
    /// # Panics
    /// This will panic if the translation ID is not found. If you need to translate an arbitrary ID, you should use `.translate_checked()`
    /// instead.
    pub fn translate<I: Into<String> + std::fmt::Display>(&self, id: I) -> String {
        let translation_res = self.translate_checked(&id.to_string());
        match translation_res {
            Ok(translation) => translation,
            Err(_) => panic!("translation id '{}' not found for locale '{}' (if you're not hardcoding the id, use `.translate_checked()` instead)", id, self.locale)
        }
    }
    /// Gets the translation of the given ID for the current locale. This will return an error gracefully if the ID doesn't exist. If
    /// you're hardcoding a translation ID in though, you should use `.translate()` instead, which will panic if the ID doesn't exist.
    pub fn translate_checked<I: Into<String> + std::fmt::Display>(&self, id: I) -> Result<String> {
        let id_str = id.to_string();
        let translation = self.translations.get(&id_str);
        match translation {
            Some(translation) => Ok(translation.to_string()),
            None => bail!(ErrorKind::TranslationIdNotFound(
                id_str,
                self.locale.clone()
            )),
        }
    }
}

/// A super-shortcut for translating stuff. Your translator must be named `translator` for this to work.
// FIXME
#[macro_export]
macro_rules! t {
    ($id:literal, $translator:expr) => {
        $translator.translate($id)
    };
}
