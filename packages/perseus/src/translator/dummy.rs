use crate::translator::errors::*;

/// An empty file extension, because all translators must provide one.
pub const DUMMY_TRANSLATOR_FILE_EXT: &str = "";

/// A dummy translator that will be used if an app doesn't use internationalization. This has literally no capabilities whatsoever,
/// and serves as a blank API interface. If this is called as if it's a fully-fledged translator, it will panic.
///
/// If you're using i18n, enable the `translator-fluent` feature flag to replace this with `FluentTranslator`, which will actually translate
/// things.
pub struct DummyTranslator;
impl DummyTranslator {
    /// Creates a new dummy translator, accepting the usual parameters for translators.
    pub fn new(_locale: String, _translations_string: String) -> Result<Self, TranslatorError> {
        Ok(Self {})
    }
    /// A dummy function for localizing a URL. This will panic if called.
    pub fn url(&self, _url: &str) -> String {
        panic!("attempted to call function on dummy translator, please add the `translator-fluent` feature flag if you want to use i18n")
    }
    /// Returns the `xx-XX` locale always, which is used by Perseus if i18n is disabled.
    pub fn get_locale(&self) -> String {
        "xx-XX".to_string()
    }
    /// A dummy function that will NOT translate the given ID! This will panic if called.
    pub fn translate(&self, _id: &str) -> String {
        panic!("attempted to call function on dummy translator, please add the `translator-fluent` feature flag if you want to use i18n")
    }
    /// A dummy function that will NOT translate the given ID! This will panic if called.
    pub fn translate_checked<I: Into<String> + std::fmt::Display>(
        &self,
        _id: &str,
    ) -> Result<String, TranslatorError> {
        panic!("attempted to call function on dummy translator, please add the `translator-fluent` feature flag if you want to use i18n")
    }
}

/// A useless pseudo-map. This is a workaround until conditional compilation of expressions is supported, which will simplify this
/// system significantly.
#[doc(hidden)]
pub struct TranslationArgs;
impl TranslationArgs {
    /// Creates a new instance of this `struct`.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }
    /// A filler function to conform to the typical argument-setting interface. Again, this will be entirele unnecessary once conditional
    /// expression compilation is supported.
    pub fn set(&self, _key: &str, _val: &str) {}
}

/// The internal dummy backend for the `t!` macro. This
#[doc(hidden)]
pub fn t_macro_backend(_id: &str) -> String {
    panic!("attempted to call translator macro, you should enable the `translator-fluent` flag to use i18n")
}
/// The internal Fluent backend for the `t!` macro, when it's used with arguments.
#[doc(hidden)]
pub fn t_macro_backend_with_args(_id: &str, _args: TranslationArgs) -> String {
    panic!("attempted to call translator macro, you should enable the `translator-fluent` flag to use i18n")
}
/// The internal Fluent backend for the `link!` macro.
#[doc(hidden)]
pub fn link_macro_backend(_url: &str) -> String {
    panic!("attempted to call translator macro, you should enable the `translator-fluent` flag to use i18n")
}
