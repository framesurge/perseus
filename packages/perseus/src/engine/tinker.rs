use crate::{plugins::PluginAction, SsrNode, PerseusAppBase};
use crate::{i18n::TranslationsManager, stores::MutableStore};

/// Runs tinker plugin actions.
///
/// Note that this expects to be run in the root of the project.
pub fn tinker(app: PerseusAppBase<SsrNode, impl MutableStore, impl TranslationsManager>) {
    let plugins = app.get_plugins();
    // Run all the tinker actions
    // Note: this is deliberately synchronous, tinker actions that need a multithreaded async runtime should probably
    // be making their own engines!
    plugins
        .functional_actions
        .tinker
        .run((), plugins.get_plugin_data());
}
