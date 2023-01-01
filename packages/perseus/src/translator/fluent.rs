use crate::{reactor::Reactor, translator::errors::*, PerseusNodeType};
use fluent_bundle::{bundle::FluentBundle, FluentArgs, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;
use std::sync::Arc;
use sycamore::prelude::{use_context, Scope};
use unic_langid::{LanguageIdentifier, LanguageIdentifierError};

/// The file extension used by the Fluent translator, which expects FTL files.
pub const FLUENT_TRANSLATOR_FILE_EXT: &str = "ftl";

/// Manages translations on the client-side for a single locale using Mozilla's [Fluent](https://projectfluent.org/) syntax.
/// This is safely `Clone`able, and is provided through Sycamore's context
/// system to every template in your app automatically. Usually, you will
/// use this only through the [`t!`] and [`link!`] macros.
///
/// Fluent supports compound messages, with many variants, which can specified
/// here using the form `[id].[variant]` in a translation ID, as a `.` is not
/// valid in an ID anyway, and so can be used as a delimiter. More than one dot
/// will result in an error.
///
/// Note that this uses the concurrent version of `FluentBundle` to support
/// server-side usage.
#[derive(Clone)]
pub struct FluentTranslator {
    /// Stores the internal Fluent data for translating. This bundle directly
    /// owns its attached resources (translations). This uses an `Arc<T>` to
    /// allow cloning, and so the broader translator should be cloned as
    /// sparingly as possible to minimize overhead.
    bundle: Arc<FluentBundle<FluentResource, IntlLangMemoizer>>,
    /// The locale for which translations are being managed by this instance.
    locale: String,
}
impl std::fmt::Debug for FluentTranslator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FluentTranslator")
            .field("locale", &self.locale)
            .finish()
    }
}
impl FluentTranslator {
    /// Creates a new translator for a given locale, passing in translations in
    /// FTL syntax form.
    pub fn new(locale: String, ftl_string: String) -> Result<Self, TranslatorError> {
        let resource = FluentResource::try_new(ftl_string)
            // If this errors, we get it still and a vector of errors (wtf.)
            .map_err(|(_, errs)| TranslatorError::TranslationsStrSerFailed {
                locale: locale.clone(),
                source: errs
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<String>()
                    .into(),
            })?;
        let lang_id: LanguageIdentifier =
            locale.parse().map_err(|err: LanguageIdentifierError| {
                TranslatorError::InvalidLocale {
                    locale: locale.clone(),
                    source: Box::new(err),
                }
            })?;
        let mut bundle = FluentBundle::new_concurrent(vec![lang_id]);
        bundle.add_resource(resource).map_err(|errs| {
            TranslatorError::TranslationsStrSerFailed {
                locale: locale.clone(),
                source: errs
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<String>()
                    .into(),
            }
        })?;
        let bundle = Arc::new(bundle);

        Ok(Self { bundle, locale })
    }
    /// Gets the path to the given URL in whatever locale the instance is
    /// configured for. This also applies the path prefix.
    pub fn url(&self, url: &str) -> String {
        let url = url.strip_prefix('/').unwrap_or(url);
        format!("{}/{}", self.locale, url)
    }
    /// Gets the locale for which this instance is configured.
    pub fn get_locale(&self) -> String {
        self.locale.clone()
    }
    /// Translates the given ID. This additionally takes any arguments that
    /// should be interpolated. If your i18n system also has variants,
    /// they should be specified somehow in the ID.
    /// # Panics
    /// This will `panic!` if any errors occur while trying to prepare the given
    /// ID. Therefore, this method should only be used for hardcoded IDs
    /// that can be confirmed as valid. If you need to parse arbitrary IDs, use
    /// `.translate_checked()` instead.
    pub fn translate(&self, id: &str, args: Option<FluentArgs>) -> String {
        let translation_res = self.translate_checked(id, args);
        match translation_res {
            Ok(translation) => translation,
            Err(_) => panic!("translation id '{}' not found for locale '{}' (if you're not hardcoding the id, use `.translate_checked()` instead)", id, self.locale)
        }
    }
    /// Translates the given ID, returning graceful errors. This additionally
    /// takes any arguments that should be interpolated. If your i18n system
    /// also has variants, they should be specified somehow in the ID.
    pub fn translate_checked(
        &self,
        id: &str,
        args: Option<FluentArgs>,
    ) -> Result<String, TranslatorError> {
        let id_str = id.to_string();
        // Deal with the possibility of a specified variant
        let id_vec: Vec<&str> = id_str.split('.').collect();
        let id_str = id_vec[0].to_string();
        let variant = id_vec.get(1);

        // This is the message in the Fluent system, an unformatted translation (still
        // needs variables etc.) This may also be compound, which means it has
        // multiple variants
        let msg = self.bundle.get_message(&id_str);
        let msg = match msg {
            Some(msg) => msg,
            None => {
                return Err(TranslatorError::TranslationIdNotFound {
                    id: id_str,
                    locale: self.locale.clone(),
                })
            }
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
                    // Once we find the requested variant, we don't need to continue (they should
                    // all be unique)
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
                return Err(TranslatorError::TranslationFailed {
                    id: id_str,
                    locale: self.locale.clone(),
                    source: "no variant provided for compound message".into(),
                });
            }
        }
        // Check for any errors
        // TODO apparently these aren't all fatal, but how do we know?
        if !errors.is_empty() {
            return Err(TranslatorError::TranslationFailed {
                id: id_str,
                locale: self.locale.clone(),
                source: errors
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<String>()
                    .into(),
            });
        }
        // Make sure we've actually got a translation
        match translation {
            Some(translation) => Ok(translation.to_string()),
            None => Err(TranslatorError::NoTranslationDerived {
                id: id_str,
                locale: self.locale.clone(),
            }),
        }
    }
    /// Gets the Fluent bundle for more advanced translation requirements.
    pub fn get_bundle(&self) -> &FluentBundle<FluentResource, IntlLangMemoizer> {
        &self.bundle
    }
}

/// An alias for `FluentArgs`. This is a workaround until conditional
/// compilation of expressions is supported, which will simplify this
/// system significantly.
#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub type TranslationArgs<'args> = FluentArgs<'args>;

/// The internal Fluent backend for the `t!` macro.
#[doc(hidden)]
pub fn t_macro_backend(id: &str, cx: Scope) -> String {
    // This `G` doesn't actually need to match up at all, but we do need to find the
    // right type
    let translator = use_context::<Reactor<PerseusNodeType>>(cx).get_translator();
    translator.translate(id, None)
}
/// The internal Fluent backend for the `t!` macro, when it's used with
/// arguments.
#[doc(hidden)]
pub fn t_macro_backend_with_args(id: &str, args: FluentArgs, cx: Scope) -> String {
    // This `G` doesn't actually need to match up at all, but we do need to find the
    // right type
    let translator = use_context::<Reactor<PerseusNodeType>>(cx).get_translator();
    translator.translate(id, Some(args))
}
/// The internal Fluent backend for the `link!` macro.
#[doc(hidden)]
pub fn link_macro_backend(url: &str, cx: Scope) -> String {
    // This `G` doesn't actually need to match up at all, but we do need to find the
    // right type
    let translator = use_context::<Reactor<PerseusNodeType>>(cx).get_translator();
    translator.url(url)
}
