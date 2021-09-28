#![allow(missing_docs)]

use thiserror::Error;

/// Errors that can occur in a translator.
#[derive(Error, Debug)]
pub enum TranslatorError {
    #[error("translation id '{id}' not found for locale '{locale}'")]
    TranslationIdNotFound { id: String, locale: String },
    #[error("translations string for locale '{locale}' couldn't be parsed")]
    TranslationsStrSerFailed {
        locale: String,
        // TODO
        #[source]
        source: Box<dyn std::error::Error>,
    },
    #[error("locale '{locale}' is of invalid form")]
    InvalidLocale {
        locale: String,
        // We have a source here to support different i18n systems' definitions of a locale
        #[source]
        source: Box<dyn std::error::Error>,
    },
    #[error("translating message '{id}' into '{locale}' failed")]
    TranslationFailed {
        id: String,
        locale: String,
        source: Box<dyn std::error::Error>,
    },
    /// This could be caused by an invalid variant for a compound message.
    #[error("no translation could be derived for message '{id}' in locale '{locale}'")]
    NoTranslationDerived { id: String, locale: String },
}
