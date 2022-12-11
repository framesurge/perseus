use super::Turbine;
use crate::{
    errors::PluginError, i18n::TranslationsManager, plugins::PluginAction, stores::MutableStore,
};

impl<M: MutableStore, T: TranslationsManager> Turbine<M, T> {
    /// Runs tinker plugin actions.
    pub fn tinker(&self) -> Result<(), PluginError> {
        // Run all the tinker actions
        // Note: this is deliberately synchronous, tinker actions that need a
        // multithreaded async runtime should probably be making their own engines!
        self.plugins
            .functional_actions
            .tinker
            .run((), self.plugins.get_plugin_data())?;

        Ok(())
    }
}
