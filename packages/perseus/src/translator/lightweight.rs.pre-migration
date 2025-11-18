use crate::reactor::Reactor;
use crate::translator::errors::*;
use crate::PerseusNodeType;
use std::collections::HashMap;
use sycamore::prelude::{use_context, Scope, Signal};

/// The file extension used by the lightweight translator, which expects JSON
/// files.
pub const LIGHTWEIGHT_TRANSLATOR_FILE_EXT: &str = "json";

/// Manages translations for a single locale using a custom lightweight
/// translations management system optimized for systems that don't need
/// [Fluent]()'s complexity. If you need control over things like
/// pluralization, gender, etc., you should use the `translator-fluent`
/// feature instead.
///
/// The reason this exists is to enable systems that don't need those features
/// to access i18n with smaller Wasm bundle sizes, since Fluent tends to create
/// substantial bloat.
///
/// Translations for this system should be specified in JSON form, with simple
/// key-value pairs from translation ID to actual translation, with `{ $variable
/// }` syntax used for variables (spacing matters!). If you need to do something
/// like pluralization with this system, you should use multiple separate
/// translation IDs.
///
/// This system supports variants only in the most basic way: you could create
/// multiple 'sub-ids' on ID `x` by having one ID called `x.y` and another
/// called `x.z`, etc., but the system doesn't particularly care, unlike Fluent,
/// which explicitly handles these cases.
#[derive(Clone)]
pub struct LightweightTranslator {
    /// The locale for which translations are being managed by this instance.
    locale: String,
    /// An internal store of the key-value pairs of translation IDs to
    /// translations.
    translations: HashMap<String, String>,
}
impl std::fmt::Debug for LightweightTranslator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LightweightTranslator")
            .field("locale", &self.locale)
            .finish()
    }
}
impl LightweightTranslator {
    /// Creates a new translator for a given locale, passing in translations in
    /// JSON form.
    pub fn new(locale: String, json_string: String) -> Result<Self, TranslatorError> {
        // Deserialize the JSON
        let translations =
            serde_json::from_str::<HashMap<String, String>>(&json_string).map_err(|err| {
                TranslatorError::TranslationsStrSerFailed {
                    locale: locale.to_string(),
                    source: err.into(),
                }
            })?;

        Ok(Self {
            translations,
            locale,
        })
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
    ///
    /// # Panics
    /// This will `panic!` if any errors occur while trying to prepare the given
    /// ID. Therefore, this method should only be used for hardcoded IDs
    /// that can be confirmed as valid. If you need to parse arbitrary IDs, use
    /// `.translate_checked()` instead.
    pub fn translate(&self, id: &str, args: Option<TranslationArgs>) -> String {
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
        args: Option<TranslationArgs>,
    ) -> Result<String, TranslatorError> {
        match self.translations.get(id) {
            Some(translation) => {
                let mut translation = translation.to_string();
                // Loop through each of the arguments and interpolate them
                if let Some(args) = args {
                    for (k, v) in args.0.iter() {
                        // Replace `${<k>}`, with `v`
                        translation = translation.replace(&format!("{{ ${} }}", k), v);
                    }
                }
                Ok(translation)
            }
            None => Err(TranslatorError::TranslationIdNotFound {
                locale: self.locale.to_string(),
                id: id.to_string(),
            }),
        }
    }
    /// Gets the underlying translations for more advanced translation
    /// requirements.
    ///
    /// Most of the time, if you need to call this, you should seriously
    /// consider using `translator-fluent` instead.
    pub fn get_bundle(&self) -> &HashMap<String, String> {
        &self.translations
    }
}

/// A *very* simple argument interpolation system based on a `HashMap`. Any more
/// complex functionality should use `translator-fluent` instead.
#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub struct TranslationArgs(pub HashMap<String, String>);
impl TranslationArgs {
    /// Alias for `.insert()` (needed for Fluent compat).
    pub fn set(&mut self, k: &str, v: &str) -> Option<String> {
        self.0.insert(k.to_string(), v.to_string())
    }
    /// Alias for `.get()` (needed for Fluent compat).
    pub fn get(&self, k: &str) -> Option<&String> {
        self.0.get(k)
    }
    /// Alias for `.new()` (needed for Fluent compat).
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

/// The internal lightweight backend for the `t!` macro.
#[doc(hidden)]
pub fn t_macro_backend(id: &str, cx: Scope) -> String {
    // This `G` doesn't actually need to match up at all, but we do need to find the
    // right type
    let translator = use_context::<Reactor<PerseusNodeType>>(cx).get_translator();
    translator.translate(id, None)
}
/// The internal lightweight backend for the `t!` macro, when it's used with
/// arguments.
#[doc(hidden)]
pub fn t_macro_backend_with_args(id: &str, args: TranslationArgs, cx: Scope) -> String {
    // This `G` doesn't actually need to match up at all, but we do need to find the
    // right type
    let translator = use_context::<Reactor<PerseusNodeType>>(cx).get_translator();
    translator.translate(id, Some(args))
}
/// The internal lightweight backend for the `link!` macro.
#[doc(hidden)]
pub fn link_macro_backend(url: &str, cx: Scope) -> String {
    // This `G` doesn't actually need to match up at all, but we do need to find the
    // right type
    let translator = use_context::<Reactor<PerseusNodeType>>(cx).get_translator();
    translator.url(url)
}
