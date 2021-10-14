mod error_pages;
mod templates;

use perseus::define_app;
use perseus::plugins::*;
use perseus::Template;

fn get_test_plugin<G: perseus::GenericNode>() -> Plugin<G> {
    Plugin {
        name: "test-plugin".to_string(),
        plugin_type: PluginType::Functional,
        functional_actions_registrar: Box::new(|mut actions| {
            actions
                .settings_actions
                .add_static_aliases
                .register_plugin("test-plugin", |_, _| {
                    let mut map = std::collections::HashMap::new();
                    map.insert("/Cargo.toml".to_string(), "Cargo.toml".to_string());
                    map
                });
            actions
                .settings_actions
                .add_templates
                .register_plugin("test-plugin", |_, _| {
                    vec![Template::new("about")
                        .template(|_| sycamore::template! { p { "Hey from a plugin!" } })]
                });
            actions
        }),
    }
}

define_app! {
    templates: [
        crate::templates::index::get_template::<G>(),
        crate::templates::about::get_template::<G>()
    ],
    error_pages: crate::error_pages::get_error_pages(),
    static_aliases: {
        "/test.txt" => "static/test.txt"
    },
    plugins: Plugins::new()
        .plugin(get_test_plugin(), ())
}
