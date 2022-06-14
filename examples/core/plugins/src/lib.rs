mod error_pages;
mod plugin;
mod templates;

use perseus::{Html, PerseusApp, Plugins};

#[perseus::main(perseus_warp::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template)
        .template(crate::templates::about::get_template)
        .error_pages(crate::error_pages::get_error_pages)
        .plugins(Plugins::new().plugin_with_client_privilege(
            plugin::get_test_plugin,
            plugin::TestPluginData {
                about_page_greeting: "Hey from a plugin!".to_string(),
            },
        ))
}
