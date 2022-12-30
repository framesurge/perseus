use perseus::plugins::{empty_control_actions_registrar, Plugin, PluginAction, PluginEnv};

#[derive(Debug)]
pub struct TestPluginData {
    pub console_greeting: String,
}

pub fn get_test_plugin() -> Plugin<TestPluginData> {
    Plugin::new(
        "test-plugin",
        |mut actions| {
            // Add a static alias for `Cargo.toml`
            actions
                .settings_actions
                .add_static_aliases
                .register_plugin("test-plugin", |_, _| {
                    let mut map = std::collections::HashMap::new();
                    map.insert("/Cargo.toml".to_string(), "Cargo.toml".to_string());
                    Ok(map)
                });
            // Log the greeting the user provided when the app starts up
            actions
                .client_actions
                .start
                .register_plugin("test-plugin", |_, data| {
                    // Perseus can't do this for you just yet, but you can always `.unwrap()`
                    let data = data.downcast_ref::<TestPluginData>().unwrap();
                    perseus::web_log!("{}", data.console_greeting);
                    Ok(())
                });
            actions.tinker.register_plugin("test-plugin", |_, _| {
                println!("{:?}", std::env::current_dir().unwrap());
                // This is completely pointless, but demonstrates how plugin dependencies can
                // blow up binary sizes if they aren't made tinker-only plugins
                let test = "[package]\nname = \"test\"";
                let parsed: toml::Value = toml::from_str(test).unwrap();
                println!("{}", toml::to_string(&parsed).unwrap());
                Ok(())
            });
            actions
        },
        empty_control_actions_registrar,
        PluginEnv::Both,
    )
}
