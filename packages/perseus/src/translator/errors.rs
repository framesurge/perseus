pub use error_chain::bail;
use error_chain::error_chain;

// This has its own error management because the user might be implementing translators themselves
error_chain! {
    errors {
        /// For when a translation ID doesn't exist.
        TranslationIdNotFound(id: String, locale: String) {
            description("translation id not found for current locale")
            display("translation id '{}' not found for locale '{}'", id, locale)
        }
        /// For when the given string of translations couldn't be correctly parsed
        TranslationsStrSerFailed(locale: String, err: String) {
            description("given translations string couldn't be parsed")
            display("given translations string for locale '{}' couldn't be parsed: '{}'", locale, err)
        }
        /// For when the given locale was invalid. This takes an error because different i18n systems may have different requirements.
        InvalidLocale(locale: String, err: String) {
            description("given locale was invalid")
            display("given locale '{}' was invalid: '{}'", locale, err)
        }
        /// For when the translation of a message failed for some reason generally.
        TranslationFailed(id: String, locale: String, err: String) {
            description("message translation failed")
            display("translation of message with id '{}' into locale '{}' failed: '{}'", id, locale, err)
        }
        /// For when the we couldn't arrive at a translation for some reason. This might be caused by an invalid variant for a compound
        /// message.
        NoTranslationDerived(id: String, locale: String) {
            description("no translation derived for message")
            display("no translation could be derived for message with id '{}' in locale '{}'", id, locale)
        }
    }
}
