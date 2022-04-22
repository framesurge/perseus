/// Errors for translators. These are separate so new translators can easily be created in a modular fashion.
pub mod errors;

// We export each translator by name
#[cfg(feature = "translator-fluent")]
mod fluent;
#[cfg(feature = "translator-fluent")]
pub use fluent::{FluentTranslator, FLUENT_TRANSLATOR_FILE_EXT};

#[cfg(all(not(feature = "translator-fluent")))]
mod dummy;
#[cfg(all(not(feature = "translator-fluent")))]
pub use dummy::{DummyTranslator, DUMMY_TRANSLATOR_FILE_EXT};

// And then we export defaults using feature gates
#[cfg(feature = "translator-fluent")]
pub use FluentTranslator as Translator;
#[cfg(feature = "translator-fluent")]
pub use FLUENT_TRANSLATOR_FILE_EXT as TRANSLATOR_FILE_EXT;

// And then we export the appropriate macro backends, hidden from the docs
#[cfg(feature = "translator-fluent")]
#[doc(hidden)]
pub use fluent::link_macro_backend;
#[cfg(feature = "translator-fluent")]
#[doc(hidden)]
pub use fluent::t_macro_backend;
#[cfg(feature = "translator-fluent")]
#[doc(hidden)]
pub use fluent::t_macro_backend_with_args;
#[cfg(feature = "translator-fluent")]
#[doc(hidden)]
pub use fluent::TranslationArgs;

#[cfg(all(not(feature = "translator-fluent")))]
#[doc(hidden)]
pub use dummy::link_macro_backend;
#[cfg(all(not(feature = "translator-fluent")))]
#[doc(hidden)]
pub use dummy::t_macro_backend;
#[cfg(all(not(feature = "translator-fluent")))]
#[doc(hidden)]
pub use dummy::t_macro_backend_with_args;
#[cfg(all(not(feature = "translator-fluent")))]
#[doc(hidden)]
pub use dummy::TranslationArgs;

// If no translators have been specified, we'll use a dummy one
#[cfg(all(not(feature = "translator-fluent")))]
pub use DummyTranslator as Translator;
#[cfg(all(not(feature = "translator-fluent")))]
pub use DUMMY_TRANSLATOR_FILE_EXT as TRANSLATOR_FILE_EXT;

/// Translates the given ID conveniently, taking arguments for interpolation as required.
#[macro_export]
macro_rules! t {
    // When there are no arguments to interpolate
    ($id:expr, $cx:expr) => {
        $crate::internal::i18n::t_macro_backend($id, $cx)
    };
    // When there are arguments to interpolate
    ($id:expr, {
        $($key:literal: $value:expr),+
    }, $cx:expr) => {{
        let mut args = $crate::internal::i18n::TranslationArgs::new();
        $(
            args.set($key, $value);
        )+
        $crate::internal::i18n::t_macro_backend_with_args($id, args, $cx)
    }};
}
/// Gets the link to the given resource in internationalized form conveniently.
#[macro_export]
macro_rules! link {
    ($url:expr, $cx:expr) => {
        $crate::internal::i18n::link_macro_backend($url, $cx)
    };
}
