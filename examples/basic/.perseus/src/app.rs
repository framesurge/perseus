// This file is used for processing data from the `define_app!` macro
// It also applies plugin opportunities for changing aspects thereof

pub use app::get_plugins;
use perseus::{
    internal::i18n::Locales,
    stores::ImmutableStore,
    templates::{ArcTemplateMap, TemplateMap},
    ErrorPages, Html, PluginAction, Plugins,
};
use std::{collections::HashMap, rc::Rc, sync::Arc};

pub use app::{get_mutable_store, get_translations_manager};

// These functions all take plugins so we don't have to perform possibly very costly allocation more than once in an environment (e.g. browser, build process, export process, server)

// pub fn get_mutable_store() -> impl MutableStore {
//     todo!()
// }
pub fn get_immutable_store<G: Html>(plugins: &Plugins<G>) -> ImmutableStore {
    let immutable_store = app::get_immutable_store();
    plugins
        .control_actions
        .settings_actions
        .set_immutable_store
        .run(immutable_store.clone(), plugins.get_plugin_data())
        .unwrap_or(immutable_store)
}
pub fn get_app_root<G: Html>(plugins: &Plugins<G>) -> String {
    plugins
        .control_actions
        .settings_actions
        .set_app_root
        .run((), plugins.get_plugin_data())
        .unwrap_or_else(|| app::APP_ROOT.to_string())
}
// pub async fn get_translations_manager() -> impl TranslationsManager {
//     todo!()
// }
pub fn get_locales<G: Html>(plugins: &Plugins<G>) -> Locales {
    let locales = app::get_locales();
    plugins
        .control_actions
        .settings_actions
        .set_locales
        .run(locales.clone(), plugins.get_plugin_data())
        .unwrap_or(locales)
}
// This also performs rescoping and security checks so that we don't include anything outside the project root
pub fn get_static_aliases<G: Html>(plugins: &Plugins<G>) -> HashMap<String, String> {
    let mut static_aliases = app::get_static_aliases();
    // This will return a map of plugin name to another map of static aliases that that plugin produced
    let extra_static_aliases = plugins
        .functional_actions
        .settings_actions
        .add_static_aliases
        .run((), plugins.get_plugin_data());
    for (_plugin_name, aliases) in extra_static_aliases {
        let new_aliases: HashMap<String, String> = aliases
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        static_aliases.extend(new_aliases);
    }

    let mut scoped_static_aliases = HashMap::new();
    for (url, path) in static_aliases {
        // We need to move this from being scoped to the app to being scoped for `.perseus/`
        // TODO make sure this works properly on Windows
        let new_path = if path.starts_with('/') {
            // Absolute paths are a security risk and are disallowed
            panic!(
                "it's a security risk to include absolute paths in `static_aliases` ('{}')",
                path
            );
        } else if path.starts_with("../") {
            // Anything outside this directory is a security risk as well
            panic!("it's a security risk to include paths outside the current directory in `static_aliases` ('{}')", path);
        } else if path.starts_with("./") {
            // `./` -> `../` (moving to execution from `.perseus/`)
            // But if we're operating standalone, it stays the same
            if cfg!(feature = "standalone") {
                path.to_string()
            } else {
                format!(".{}", path)
            }
        } else {
            // Anything else gets a `../` prepended
            // But if we're operating standalone, it stays the same
            if cfg!(feature = "standalone") {
                path.to_string()
            } else {
                format!("../{}", path)
            }
        };

        scoped_static_aliases.insert(url, new_path);
    }

    scoped_static_aliases
}
// This doesn't take plugins because that would actually increase allocation and indirection on the server
pub fn get_templates_map<G: Html>(plugins: &Plugins<G>) -> TemplateMap<G> {
    let mut templates = app::get_templates_map::<G>();
    // This will return a map of plugin name to a vector of templates to add
    let extra_templates = plugins
        .functional_actions
        .settings_actions
        .add_templates
        .run((), plugins.get_plugin_data());
    for (_plugin_name, plugin_templates) in extra_templates {
        // Turn that vector into a template map by extracting the template root paths as keys
        for template in plugin_templates {
            templates.insert(template.get_path(), Rc::new(template));
        }
    }

    templates
}
pub fn get_templates_map_atomic<G: Html>(plugins: &Plugins<G>) -> ArcTemplateMap<G> {
    let mut templates = app::get_templates_map_atomic::<G>();
    // This will return a map of plugin name to a vector of templates to add
    let extra_templates = plugins
        .functional_actions
        .settings_actions
        .add_templates
        .run((), plugins.get_plugin_data());
    for (_plugin_name, plugin_templates) in extra_templates {
        // Turn that vector into a template map by extracting the template root paths as keys
        for template in plugin_templates {
            templates.insert(template.get_path(), Arc::new(template));
        }
    }

    templates
}
pub fn get_error_pages<G: Html>(plugins: &Plugins<G>) -> ErrorPages<G> {
    let mut error_pages = app::get_error_pages::<G>();
    // This will return a map of plugin name to a map of status codes to error pages
    let extra_error_pages = plugins
        .functional_actions
        .settings_actions
        .add_error_pages
        .run((), plugins.get_plugin_data());
    for (_plugin_name, plugin_error_pages) in extra_error_pages {
        for (status, error_page) in plugin_error_pages {
            error_pages.add_page_rc(status, error_page);
        }
    }

    error_pages
}

// We provide alternatives for `get_templates_map` and `get_error_pages` that get their own plugins
// This avoids major allocation/sync problems on the server
pub fn get_templates_map_contained<G: Html>() -> TemplateMap<G> {
    let plugins = get_plugins::<G>();
    get_templates_map(&plugins)
}
pub fn get_templates_map_atomic_contained<G: Html>() -> ArcTemplateMap<G> {
    let plugins = get_plugins::<G>();
    get_templates_map_atomic(&plugins)
}
pub fn get_error_pages_contained<G: Html>() -> ErrorPages<G> {
    let plugins = get_plugins::<G>();
    get_error_pages(&plugins)
}
