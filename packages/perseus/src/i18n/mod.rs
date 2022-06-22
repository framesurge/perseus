#[cfg(target_arch = "wasm32")]
mod client_translations_manager;
#[cfg(target_arch = "wasm32")]
mod locale_detector;
mod locales;
mod translations_manager;

#[cfg(target_arch = "wasm32")]
pub use client_translations_manager::ClientTranslationsManager;
#[cfg(target_arch = "wasm32")]
pub use locale_detector::detect_locale;
pub use locales::Locales;
pub use translations_manager::{
    FsTranslationsManager, TranslationsManager, TranslationsManagerError,
};

// No explicitly internal things here
