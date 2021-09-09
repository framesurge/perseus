/// Errors for translators. These are separate so new translators can easily be created in a modular fashion.
pub mod errors;

// We export each translator by name
#[cfg(feature = "translator-fluent")]
mod fluent;
#[cfg(feature = "translator-fluent")]
pub use fluent::{FluentTranslator, FLUENT_TRANSLATOR_FILE_EXT};

// And then we export defaults using feature gates
#[cfg(feature = "translator-dflt-fluent")]
pub use FluentTranslator as Translator;
#[cfg(feature = "translator-dflt-fluent")]
pub use FLUENT_TRANSLATOR_FILE_EXT as TRANSLATOR_FILE_EXT;
