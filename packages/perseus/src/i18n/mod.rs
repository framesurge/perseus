// This module file is controlled, because it's used as an external module as
// well

#[cfg(client)]
mod client_translations_manager;
#[cfg(client)]
mod locale_detector;
mod locales;
mod translations_manager;

#[cfg(client)]
pub(crate) use client_translations_manager::ClientTranslationsManager;
#[cfg(client)]
pub(crate) use locale_detector::detect_locale;
pub use locales::Locales;
pub use translations_manager::{
    FsTranslationsManager, TranslationsManager, TranslationsManagerError,
};

// Export the `translator` module from here as well
pub use crate::translator::*;

#[doc(hidden)]
/// The default translations directory when we're running with the `.perseus/`
/// support structure.
pub static DFLT_TRANSLATIONS_DIR: &str = "./translations";
