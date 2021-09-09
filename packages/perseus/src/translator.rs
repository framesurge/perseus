use error_chain::{bail, error_chain};
use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};
use std::rc::Rc;
use unic_langid::{LanguageIdentifier, LanguageIdentifierError};

/// Defines app information about i18n, specifically about which locales are supported.
#[derive(Clone)]
pub struct Locales {
    /// The default locale, which will be used as a fallback if the user's locale can't be extracted. This will be built for at build-time.
    pub default: String,
    /// All other supported locales, which will all be built at build time.
    pub other: Vec<String>,
}
impl Locales {
    /// Gets all the supported locales by combining the default, and other.
    pub fn get_all(&self) -> Vec<&String> {
        let mut vec: Vec<&String> = vec![&self.default];
        vec.extend(&self.other);

        vec
    }
    /// Checks if the given locale is supported.
    pub fn is_supported(&self, locale: &str) -> bool {
        let locales = self.get_all();
        locales.iter().any(|l| *l == locale)
    }
}

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

/// Manages translations on the client-side for a single locale using Mozilla's [Fluent](https://projectfluent.org/) syntax. This
/// should generally be placed into an `Rc<T>` and referred to by every template in an app. You do NOT want to be cloning potentially
/// thousands of translations!
///
/// Fluent supports compound messages, with many variants, which can specified here using the form `[id].[variant]` in a translation ID,
/// as a `.` is not valid in an ID anyway, and so can be used as a delimiter. More than one dot will result in an error.
pub struct Translator {
    /// Stores the internal Fluent data for translating. This bundle directly owns its attached resources (translations).
    bundle: Rc<FluentBundle<FluentResource>>,
    /// The locale for which translations are being managed by this instance.
    locale: String,
}
impl Translator {
    /// Creates a new translator for a given locale, passing in translations in FTL syntax form.
    pub fn new(locale: String, ftl_string: String) -> Result<Self> {
        let resource = FluentResource::try_new(ftl_string)
            // If this errors, we get it still and a vector of errors (wtf.)
            .map_err(|(_, errs)| {
                ErrorKind::TranslationsStrSerFailed(
                    locale.clone(),
                    errs.iter().map(|e| e.to_string()).collect(),
                )
            })?;
        let lang_id: LanguageIdentifier =
            locale.parse().map_err(|err: LanguageIdentifierError| {
                ErrorKind::InvalidLocale(locale.clone(), err.to_string())
            })?;
        let mut bundle = FluentBundle::new(vec![lang_id]);
        bundle.add_resource(resource).map_err(|errs| {
            ErrorKind::TranslationsStrSerFailed(
                locale.clone(),
                errs.iter().map(|e| e.to_string()).collect(),
            )
        })?;

        Ok(Self {
            bundle: Rc::new(bundle),
            locale,
        })
    }
    /// Gets the path to the given URL in whatever locale the instance is configured for.
    pub fn url<S: Into<String> + std::fmt::Display>(&self, url: S) -> String {
        format!("/{}{}", self.locale, url)
    }
    /// Gets the locale for which this instancce is configured.
    pub fn get_locale(&self) -> String {
        self.locale.clone()
    }
    /// Translates the given ID. This additionally takes any arguments that should be interpolated. If your i18n system also has variants,
    /// they should be specified somehow in the ID.
    /// # Panics
    /// This will `panic!` if any errors occur while trying to prepare the given ID. Therefore, this method should only be used for
    /// hardcoded IDs that can be confirmed as valid. If you need to parse arbitrary IDs, use `.translate_checked()` instead.
    pub fn translate<I: Into<String> + std::fmt::Display>(
        &self,
        id: I,
        args: Option<FluentArgs>,
    ) -> String {
        let translation_res = self.translate_checked(&id.to_string(), args);
        match translation_res {
            Ok(translation) => translation,
            Err(_) => panic!("translation id '{}' not found for locale '{}' (if you're not hardcoding the id, use `.translate_checked()` instead)", id, self.locale)
        }
    }
    /// Translates the given ID, returning graceful errors. This additionally takes any arguments that should be interpolated. If your
    /// i18n system also has variants, they should be specified somehow in the ID.
    pub fn translate_checked<I: Into<String> + std::fmt::Display>(
        &self,
        id: I,
        args: Option<FluentArgs>,
    ) -> Result<String> {
        let id_str = id.to_string();
        // Deal with the possibility of a specified variant
        let id_vec: Vec<&str> = id_str.split('.').collect();
        let id_str = id_vec[0].to_string();
        let variant = id_vec.get(1);

        // This is the message in the Fluent system, an unformatted translation (still needs variables etc.)
        // This may also be compound, which means it has multiple variants
        let msg = self.bundle.get_message(&id_str);
        let msg = match msg {
            Some(msg) => msg,
            None => bail!(ErrorKind::TranslationIdNotFound(
                id_str,
                self.locale.clone()
            )),
        };
        // This module accumulates errors in a provided buffer, we'll handle them later
        let mut errors = Vec::new();
        let value = msg.value();
        let mut translation = None; // If it's compound, the requested variant may not exist
        if let Some(value) = value {
            // Non-compound, just one variant
            translation = Some(
                self.bundle
                    .format_pattern(value, args.as_ref(), &mut errors),
            );
        } else {
            // Compound, many variants, one should be specified
            if let Some(variant) = variant {
                for attr in msg.attributes() {
                    // Once we find the requested variant, we don't need to continue (they should all be unique)
                    if &attr.id() == variant {
                        translation = Some(self.bundle.format_pattern(
                            attr.value(),
                            args.as_ref(),
                            &mut errors,
                        ));
                        break;
                    }
                }
            } else {
                bail!(ErrorKind::TranslationFailed(
                    id_str,
                    self.locale.clone(),
                    "no variant provided for compound message".to_string()
                ))
            }
        }
        // Check for any errors
        // TODO apparently these aren't all fatal, but how do we know?
        if !errors.is_empty() {
            bail!(ErrorKind::TranslationFailed(
                id_str,
                self.locale.clone(),
                errors.iter().map(|e| e.to_string()).collect()
            ))
        }
        // Make sure we've actually got a translation
        match translation {
            Some(translation) => Ok(translation.to_string()),
            None => bail!(ErrorKind::NoTranslationDerived(id_str, self.locale.clone())),
        }
    }
    /// Gets the Fluent bundle for more advanced translation requirements.
    pub fn get_bundle(&self) -> Rc<FluentBundle<FluentResource>> {
        Rc::clone(&self.bundle)
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
