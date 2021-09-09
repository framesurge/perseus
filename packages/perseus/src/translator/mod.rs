/// Errors for translators. These are separate so new translators can easily be created in a modular fashion.
pub mod errors;
mod fluent;

// We export each translator by name
pub use fluent::{FluentTranslator, FLUENT_TRANSLATOR_FILE_EXT};

// And then we export defaults using feature gates
// TODO feature-gate these lines
pub use FluentTranslator as Translator;
pub use FLUENT_TRANSLATOR_FILE_EXT as TRANSLATOR_FILE_EXT;
