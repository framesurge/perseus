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

/// Translates the given ID conveniently, taking arguments for interpolation as required.
#[macro_export]
macro_rules! t {
    // When there are no arguments to interpolate
    ($id:expr) => {
        {
            let render_ctx = ::sycamore::context::use_context::<::perseus::templates::RenderCtx>();
            let translator = render_ctx.translator;
            translator.translate($id, ::std::option::Option::None)
        }
    };
    // When there are arguments to interpolate
    ($id:expr, {
        $($key:literal: $value:expr),+
    }) => {
        {
            let render_ctx = ::sycamore::context::use_context::<::perseus::templates::RenderCtx>();
            let translator = render_ctx.translator;
            let mut args = ::fluent_bundle::FluentArgs::new();
            $(
                args.set($key, $value);
            )+

            translator.translate($id, ::std::option::Option::Some(args))
        }
    };
}

/// Gets the link to the given resource in internationalized form conveniently.
#[macro_export]
macro_rules! link {
    ($url:expr) => {{
        let render_ctx = ::sycamore::context::use_context::<::perseus::templates::RenderCtx>();
        let translator = render_ctx.translator;
        translator.url($url)
    }};
}
