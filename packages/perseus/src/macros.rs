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
/// An internal macro used for defining a function to get the user's preferred translations manager (which requires multiple branches).
#[macro_export]
macro_rules! define_get_translations_manager {
    () => {
        pub fn get_translations_manager() -> impl $crate::TranslationsManager {
            // This will be executed in the context of the user's directory, but moved into `.perseus`
            // Note that `translations/` must be next to `src/`, not within it
            $crate::FsTranslationsManager::new("../translations".to_string())
        }
    };
    ($translations_manager:expr) => {
        pub fn get_translations_manager() -> impl $crate::TranslationsManager {
            $translations_manager
        }
    };
}

/// Defines the components to create an entrypoint for the app. The actual entrypoint is created in the `.perseus/` crate (where we can
/// get all the dependencies without driving the user's `Cargo.toml` nuts). This also defines the template map. This is intended to make
/// compatibility with the Perseus CLI significantly easier.
#[macro_export]
macro_rules! define_app {
    {
        root: $root_selector:literal,
        route: $route:ty,
        // The user will define something very similar to a macro pattern, which will return the template's name and its render function
        // We don't use a match statement because we abstract `NotFound` matching
        router: {
            $(
                $pat:pat => [
                    $name:expr,
                    $fn:expr,
                    $locale:expr
                ]
            ),+
        },
        error_pages: $error_pages:expr,
        templates: [
            $($template:expr),+
        ],
        // This deliberately enforces verbose i18n definition, and forces developers to consider i18n as integral
        locales: {
            default: $default_locale:literal,
            // The user doesn't have to define any other locales
            other: [$($other_locale:literal),*]
        }
        $(,config_manager: $config_manager:expr)?
        $(,translations_manager: $translations_manager:expr)?
    } => {
        /// The CSS selector that will find the app root to render Perseus in.
        pub const APP_ROUTE: &str = $root_selector;

        // We alias the user's route enum so that don't have to worry about naming
        pub type AppRoute = $route;

        /// Gets the config manager to use. This allows the user to conveniently test production managers in development. If nothing is
        /// given, the filesystem will be used.
        $crate::define_get_config_manager!($($config_manager)?);

        /// Gets the translations manager to use. This allows the user to conveniently test production managers in development. If
        /// nothing is given, the filesystem will be used.
        $crate::define_get_translations_manager!($($translations_manager)?);

        /// Defines the locales the app should build for, specifying defaults and common locales (which will be built at build-time
        /// rather than on-demand).
        pub fn get_locales() -> $crate::Locales {
            $crate::Locales {
                default: $default_locale.to_string(),
                other: vec![
                    $($other_locale.to_string()),*
                ]
            }
        }

        /// Gets a map of all the templates in the app by their root paths.
        pub fn get_templates_map<G: $crate::GenericNode>() -> $crate::TemplateMap<G> {
            $crate::get_templates_map![
                $($template),+
            ]
        }

        /// Gets a list of all the templates in the app.
        pub fn get_templates_vec<G: $crate::GenericNode>() -> Vec<$crate::Template<G>> {
            vec![
                $($template),+
            ]
        }

        /// Gets the error pages (done here so the user doesn't have to worry about naming).
        pub fn get_error_pages() -> $crate::ErrorPages {
            $error_pages
        }

        /// Matches the given route to a template name, render function, and locale.
        pub fn match_route(route: $route) -> (String, $crate::template::TemplateFn<$crate::DomNode>, String) {
            match route {
                // We regurgitate all the user's custom matches
                $(
                    $pat => (
                        $name,
                        $fn,
                        $locale
                    ),
                )+
                // We MUST handle the NotFound route before this function
                <$route>::NotFound => panic!("not found route should've been handled before reaching `match_route` (this is a bug, please report it!)")
            }
        }
    };
}
