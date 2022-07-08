/// An internal macro for adding the translations manager to an instance of an
/// app based on whether or not one has been provided. This can only be used
/// internally. Regardless of the outcome of this, the locales must already have
/// been set with `.locales_lit()`.
#[doc(hidden)]
#[macro_export]
macro_rules! add_translations_manager {
    ($app:expr, $tm:expr, $locales:expr) => {
        // We're not using `FsTranslationsManager`, set up the locales separately from
        // the translations manager
        $app = $app.locales_list($locales);
        $app = $app.translations_manager($tm);
    };
    ($app:expr, $locales:expr) => {
        // We're using `FsTranslationsManager`, and we have locales information, so
        // there's a nice convenience function to do all the caching stuff for us!
        $app = $app.locales_lit_and_translations_manager($locales);
    };
}

/// Defines the components to create an entrypoint for the app. The actual
/// entrypoint is created in the `.perseus/` crate (where we can get all the
/// dependencies without driving the user's `Cargo.toml` nuts). This also
/// defines the template map. This is intended to make compatibility with the
/// Perseus CLI significantly easier.
///
/// Warning: all properties must currently be in the correct order (`root`,
/// `templates`, `error_pages`, `global_state_creator`, `locales`,
/// `static_aliases`, `plugins`, `dist_path`, `mutable_store`,
/// `translations_manager`).
///
/// Note: as of v0.3.4, this is just a wrapper over `PerseusAppBase`, which is
/// the recommended way to create a new Perseus app (no macros involved).
#[macro_export]
macro_rules! define_app {
    // With locales
    {
        $(root: $root_selector:literal,)?
        templates: [
            $($template:expr),+
        ],
        error_pages: $error_pages:expr,
        $(global_state_creator: $global_state_creator:expr,)?
        // This deliberately enforces verbose i18n definition, and forces developers to consider i18n as integral
        locales: {
            default: $default_locale:literal,
            // The user doesn't have to define any other locales
            other: [$($other_locale:literal),*]
        }
        $(,static_aliases: {
            $($url:literal => $resource:literal),*
        })?
        $(,plugins: $plugins:expr)?
        $(,dist_path: $dist_path:literal)?
        $(,mutable_store: $mutable_store:expr)?
        $(,translations_manager: $translations_manager:expr)?
    } => {
        $crate::define_app!(
            @define_app,
            {
                $(root: $root_selector,)?
                templates: [
                    $($template),+
                ],
                error_pages: $error_pages,
                $(global_state_creator: $global_state_creator,)?
                locales: {
                    default: $default_locale,
                    // The user doesn't have to define any other locales (but they'll still get locale detection and the like)
                    other: [$($other_locale),*],
                    no_i18n: false
                }
                $(,static_aliases: {
                    $($url => $resource),*
                })?
                $(,plugins: $plugins)?
                $(,dist_path: $dist_path)?
                $(,mutable_store: $mutable_store)?
                $(,translations_manager: $translations_manager)?
            }
        );
    };
    // Without locales (default locale is set to xx-XX)
    {
        $(root: $root_selector:literal,)?
        templates: [
            $($template:expr),+
        ],
        error_pages: $error_pages:expr
        $(,global_state_creator: $global_state_creator:expr)?
        $(,static_aliases: {
            $($url:literal => $resource:literal),*
        })?
        $(,plugins: $plugins:expr)?
        $(,dist_path: $dist_path:literal)?
        $(,mutable_store: $mutable_store:expr)?
    } => {
        $crate::define_app!(
            @define_app,
            {
                $(root: $root_selector,)?
                templates: [
                    $($template),+
                ],
                error_pages: $error_pages,
                $(global_state_creator: $global_state_creator,)?
                // This deliberately enforces verbose i18n definition, and forces developers to consider i18n as integral
                locales: {
                    default: "xx-XX",
                    other: [],
                    no_i18n: true
                }
                $(,static_aliases: {
                    $($url => $resource),*
                })?
                $(,plugins: $plugins)?
                $(,dist_path: $dist_path)?
                $(,mutable_store: $mutable_store)?
            }
        );
    };
    // This is internal, and allows syntax abstractions and defaults
    (
        @define_app,
        {
            $(root: $root_selector:literal,)?
            templates: [
                $($template:expr),+
            ],
            error_pages: $error_pages:expr,
            $(global_state_creator: $global_state_creator:expr,)?
            // This deliberately enforces verbose i18n definition, and forces developers to consider i18n as integral
            locales: {
                default: $default_locale:literal,
                // The user doesn't have to define any other locales
                other: [$($other_locale:literal),*],
                // If this is `true`
                no_i18n: $no_i18n:literal
            }
            $(,static_aliases: {
                $($url:literal => $resource:literal),*
            })?
            $(,plugins: $plugins:expr)?
            $(,dist_path: $dist_path:literal)?
            $(,mutable_store: $mutable_store:expr)?
            $(,translations_manager: $translations_manager:expr)?
        }
    ) => {
        #[$crate::main]
        pub fn main<G: $crate::Html>() -> $crate::PerseusAppBase<G, impl $crate::stores::MutableStore, impl $crate::internal::i18n::TranslationsManager> {
            let mut app = $crate::PerseusAppBase::new();
            // If we have a mutable store, we'll actually initialize in a completely different way
            $(
                let mut app = $crate::PerseusAppBase::new_with_mutable_store($mutable_store).await;
            )?;
            // Conditionally add each property the user provided
            $(
                app = app.root($root_selector);
            )?;
            $(
                app = app.template(|| $template);
            )+;
            app = app.error_pages(|| $error_pages);
            $(
                app = app.global_state_creator($global_state_creator);
            )?;
            $($(
                app = app.static_alias($url, $resource);
            )*)?;
            $(
                app = app.plugins($plugins);
            )?;
            // Use `index.html` for the index view (for backward compatibility)
            // We need the filesystem here, and we don't need on it in the browser
            // We can't modify `app` if this is all in a block, so we compromise a bit
            let index_html = if cfg!(target_arch = "wasm32") {
                // In the browser, this would turn into using the hardocded default, but we don't need the index view there anyway
                ::std::result::Result::Err(::std::io::Error::from(::std::io::ErrorKind::NotFound))
            } else {
                ::std::fs::read_to_string("../index.html")
            };
            if let ::std::result::Result::Ok(index_html) = index_html {
                app = app.index_view_str(&index_html)
            }
            // Build a `Locales` instance
            let mut other_locales = ::std::vec::Vec::new();
            $(
                other_locales.push($other_locale.to_string());
            )*;
            // We can't guarantee that the user is using `FsTranslationsManager`, so we set only the locales, and the translations manager separately later
            let locales = $crate::internal::i18n::Locales {
                default: $default_locale.to_string(),
                other: other_locales,
                using_i18n: !$no_i18n
            };

            // Set the translations manager and locales information with a helper macro that can handle two different paths of provision
            $crate::add_translations_manager!(app, $($translations_manager,)? locales);

            app
        }

    };
}
