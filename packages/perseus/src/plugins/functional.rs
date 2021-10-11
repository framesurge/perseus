use crate::plugins::{PluginAction, PluginData, Runner};
use std::collections::HashMap;

/// An action for a functional plugin, which can be taken by many plugins.
#[derive(Default)]
pub struct FunctionalPluginAction<A> {
    /// The runners that will be called when this action is run.
    runners: HashMap<String, Runner<A>>,
}
impl<A> PluginAction<A> for FunctionalPluginAction<A> {
    fn run(&self, action_data: A, plugin_data: &HashMap<String, Box<dyn PluginData>>) {
        for (plugin_name, runner) in &self.runners {
            runner(
                &action_data,
                // We must have data registered for every active plugin (even if it's empty)
                plugin_data.get(plugin_name).unwrap_or_else(|| {
                    panic!("no plugin data for registered plugin {}", plugin_name)
                }),
            );
        }
    }
    fn register_plugin(&mut self, name: String, runner: Runner<A>) {
        self.runners.insert(name, runner);
    }
}

/// All the actions that a functional plugin can perform. These are all designed to be compatible with other plugins such that two plugins
/// can execute the same action.
#[derive(Default)]
pub struct FunctionalPluginActions {
    /// Actions pertaining to the build process.
    pub build_actions: FunctionalPluginBuildActions,
    /// Actions pertaining to the export process.
    pub export_actions: FunctionalPluginExportActions,
    /// Actions pertaining to the server.
    pub server_actions: FunctionalPluginServerActions,
    /// Actions pertaining to the client-side code.
    pub client_actions: FunctionalPluginClientActions,
}

// TODO add actions

/// The actions a functional plugin can take that pertain to the build process.
#[derive(Default)]
pub struct FunctionalPluginBuildActions {
    /// Runs after the build process if it completes successfully.
    pub on_after_successful_build: FunctionalPluginAction<()>,
}
/// The actions a functional plugin can take that pertain to the export process.
#[derive(Default)]
pub struct FunctionalPluginExportActions {}
/// The actions a functional plugin can take that pertain to the server.
#[derive(Default)]
pub struct FunctionalPluginServerActions {}
/// The actions a functional plugin can take that pertain to the client-side code.
#[derive(Default)]
pub struct FunctionalPluginClientActions {}
