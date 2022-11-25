use perseus::plugins::{empty_control_actions_registrar, Plugin, PluginAction, PluginEnv};
use perseus::Template;

#[derive(Debug)]
pub struct TestPluginData {
    pub about_page_greeting: String,
}

pub fn get_test_plugin<G: perseus::Html>() -> Plugin<G, TestPluginData> {
    Plugin::new(
        "test-plugin",
        |mut actions| {
            actions
                .settings_actions
                .add_static_aliases
                .register_plugin("test-plugin", |_, _| {
                    let mut map = std::collections::HashMap::new();
                    map.insert("/Cargo.toml".to_string(), "Cargo.toml".to_string());
                    map
                });
            actions.settings_actions.add_templates.register_plugin(
                "test-plugin",
                |_, plugin_data| {
                    if let Some(plugin_data) = plugin_data.downcast_ref::<TestPluginData>() {
                        let about_page_greeting = plugin_data.about_page_greeting.to_string();
                        vec![Template::new("about").template(move |cx| {
                            sycamore::view! { cx,  p { (about_page_greeting) } }
                        })]
                    } else {
                        unreachable!()
                    }
                },
            );
            actions.tinker.register_plugin("test-plugin", |_, _| {
                println!("{:?}", std::env::current_dir().unwrap());
                // This is completely pointless, but demonstrates how plugin dependencies can
                // blow up binary sizes if they aren't made tinker-only plugins
                let test = "[package]\name = \"test\"";
                let parsed: toml::Value = toml::from_str(test).unwrap();
                println!("{}", toml::to_string(&parsed).unwrap());
            });
            actions
        },
        empty_control_actions_registrar,
        PluginEnv::Both,
    )
}
