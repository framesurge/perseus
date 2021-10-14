use perseus::plugins::*;
use perseus::Template;

pub fn get_test_plugin<G: perseus::GenericNode>() -> Plugin<G> {
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
                        .template(|_| sycamore::template! { p { "Hey from a plugin!" } })
                        .head(|_| {
                            sycamore::template! {
                                title { "About Page (Plugin Modified) | Perseus Example – Plugins" }
                            }
                        })]
                });
            actions.tinker.register_plugin("test-plugin", |_, _| {
                println!("{:?}", std::env::current_dir().unwrap())
            });
            actions
        }),
    }
}
