// TODO parse `no_i18n` properly so the user can specify `false`

/// An internal macro used for defining a function to get the user's preferred config manager (which requires multiple branches).
#[macro_export]
macro_rules! define_get_config_manager {
    () => {
        pub fn get_config_manager() -> impl $crate::ConfigManager {
            // This will be executed in the context of the user's directory, but moved into `.perseus`
            $crate::FsConfigManager::new("./dist".to_string())
        }
    };
    ($config_manager:expr) => {
        pub fn get_config_manager() -> impl $crate::ConfigManager {
            $config_manager
        }
    };
}
/// An internal macro used for defining the HTML `id` at which to render the Perseus app (which requires multiple branches). The default
/// is `root`.
#[macro_export]
macro_rules! define_app_root {
    () => {
        pub static APP_ROOT: &str = "root";
    };
    ($root_id:literal) => {
        pub static APP_ROOT: &str = $root_id;
    };
}
/// An internal macro used for defining a function to get the user's preferred translations manager (which requires multiple branches).
#[macro_export]
macro_rules! define_get_translations_manager {
    ($locales:expr) => {
        pub async fn get_translations_manager() -> impl $crate::TranslationsManager {
            // This will be executed in the context of the user's directory, but moved into `.perseus`
            // Note that `translations/` must be next to `src/`, not within it
            // By default, all translations are cached
            let all_locales: Vec<String> = $locales
                .get_all()
                .iter()
                // We have a `&&String` at this point, hence the double clone
                .cloned()
                .cloned()
                .collect();
            $crate::FsTranslationsManager::new(
                "../translations".to_string(),
                all_locales,
                $crate::TRANSLATOR_FILE_EXT.to_string(),
            )
            .await
        }
    };
    ($locales:expr, $no_i18n:literal) => {
        pub async fn get_translations_manager() -> impl $crate::TranslationsManager {
            $crate::translations_manager::DummyTranslationsManager::new()
        }
    };
    ($locales:expr, $translations_manager:expr) => {
        pub async fn get_translations_manager() -> impl $crate::TranslationsManager {
            $translations_manager
        }
    };
    // If the user doesn't want i18n but also sets their own transations manager, the latter takes priority
    ($locales:expr, $no_i18n:literal, $translations_manager:expr) => {
        pub async fn get_translations_manager() -> impl $crate::TranslationsManager {
            $translations_manager
        }
    };
}
/// An internal macro used for defining locales data. This is abstracted because it needs multiple branches.
#[macro_export]
macro_rules! define_get_locales {
    {
        default: $default_locale:literal,
        other: [$($other_locale:literal),*]
    } => {
        pub fn get_locales() -> $crate::Locales {
            $crate::Locales {
                default: $default_locale.to_string(),
                other: vec![
                    $($other_locale.to_string()),*
                ],
                using_i18n: true
            }
        }
    };
    // With i18n disabled, the default locale will be `xx-XX`
    {
        default: $default_locale:literal,
        other: [$($other_locale:literal),*],
        no_i18n: $no_i18n:literal
    } => {
        pub fn get_locales() -> $crate::Locales {
            $crate::Locales {
                default: "xx-XX".to_string(),
                other: Vec::new(),
                using_i18n: false
            }
        }
    };
}

/// Defines the components to create an entrypoint for the app. The actual entrypoint is created in the `.perseus/` crate (where we can
/// get all the dependencies without driving the user's `Cargo.toml` nuts). This also defines the template map. This is intended to make
/// compatibility with the Perseus CLI significantly easier. Perseus makes i18n opt-out, so if you don't intend to use it, set `no_i18n`
/// to `true` in `locales`. Note that you must still specify a default locale for verbosity and correctness. If you specify `no_i18n` and
/// a custom translations manager, the latter will override.
///
/// Warning: all properties must currently be in the correct order (`root`, `templates`, `error_pages`, `locales`, `config_manager`,
/// `translations_manager`).
// TODO make this syntax even more compact and beautiful? (error pages inside templates?)
#[macro_export]
macro_rules! define_app {
    // With locales
    {
        $(root: $root_selector:literal,)?
        templates: [
            $($template:expr),+
        ],
        error_pages: $error_pages:expr,
        // This deliberately enforces verbose i18n definition, and forces developers to consider i18n as integral
        locales: {
            default: $default_locale:literal,
            // The user doesn't have to define any other locales
            other: [$($other_locale:literal),*]
            $(,no_i18n: $no_i18n:literal)?
        }
        $(,config_manager: $config_manager:expr)?
        $(,translations_manager: $translations_manager:expr)?
    } => {
        define_app!(
            @define_app,
            {
                $(root: $root_selector,)?
                templates: [
                    $($template),+
                ],
                error_pages: $error_pages,
                locales: {
                    default: $default_locale,
                    // The user doesn't have to define any other locales (but they'll still get locale detection and the like)
                    other: [$($other_locale),*]
                }
                $(,config_manager: $config_manager)?
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
        $(,config_manager: $config_manager:expr)?
        $(,translations_manager: $translations_manager:expr)?
    } => {
        define_app!(
            @define_app,
            {
                $(root: $root_selector,)?
                templates: [
                    $($template),+
                ],
                error_pages: $error_pages,
                // This deliberately enforces verbose i18n definition, and forces developers to consider i18n as integral
                locales: {
                    default: "xx-XX",
                    other: [],
                    no_i18n: true
                }
                $(,config_manager: $config_manager)?
                $(,translations_manager: $translations_manager)?
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
            // This deliberately enforces verbose i18n definition, and forces developers to consider i18n as integral
            locales: {
                default: $default_locale:literal,
                // The user doesn't have to define any other locales
                other: [$($other_locale:literal),*]
                // If this is defined at all, i18n will be disabled and the default locale will be set to `xx-XX`
                $(,no_i18n: $no_i18n:literal)?
            }
            $(,config_manager: $config_manager:expr)?
            $(,translations_manager: $translations_manager:expr)?
        }
    ) => {
        /// The html `id` that will find the app root to render Perseus in. For server-side interpolation, this MUST be an element of
        /// the form <div id="root_id">` in your markup (double or single quotes, `root_id` replaced by what this property is set to).
        $crate::define_app_root!($($root_selector)?);

        /// Gets the config manager to use. This allows the user to conveniently test production managers in development. If nothing is
        /// given, the filesystem will be used.
        $crate::define_get_config_manager!($($config_manager)?);

        /// Gets the translations manager to use. This allows the user to conveniently test production managers in development. If
        /// nothing is given, the filesystem will be used.
        $crate::define_get_translations_manager!(get_locales() $(, $no_i18n)? $(, $translations_manager)?);

        /// Defines the locales the app should build for, specifying defaults and common locales (which will be built at build-time
        /// rather than on-demand).
        $crate::define_get_locales! {
            default: $default_locale,
            other: [
                $($other_locale),*
            ]
            $(, no_i18n: $no_i18n)?
        }

        /// Gets a map of all the templates in the app by their root paths.
        pub fn get_templates_map<G: $crate::GenericNode>() -> $crate::TemplateMap<G> {
            $crate::get_templates_map![
                $($template),+
            ]
        }

        /// Gets a list of all the templates in the app in the order the user provided them.
        pub fn get_templates_vec<G: $crate::GenericNode>() -> Vec<$crate::Template<G>> {
            vec![
                $($template),+
            ]
        }

        /// Gets the error pages (done here so the user doesn't have to worry about naming).
        pub fn get_error_pages<G: $crate::GenericNode>() -> $crate::ErrorPages<G> {
            $error_pages
        }
    };
}
