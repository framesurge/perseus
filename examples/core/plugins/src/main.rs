mod plugin;
mod templates;

use perseus::{plugins::Plugins, prelude::*};

#[perseus::main(perseus_warp::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .error_views(ErrorViews::unlocalized_development_default())
        .plugins(Plugins::new().plugin_with_client_privilege(
            plugin::get_test_plugin,
            plugin::TestPluginData {
                console_greeting: "Hey from a plugin!".to_string(),
            },
        ))
}
