use crate::plugins::{PluginAction, PluginData, Runner};
use std::collections::HashMap;

/// An action for a functional plugin, which can be taken by many plugins. When run, a functional action will return a map of plugin
/// names to their return types.
pub struct FunctionalPluginAction<A, R> {
    /// The runners that will be called when this action is run.
    runners: HashMap<String, Runner<A, R>>,
}
impl<A, R> PluginAction<A, R, HashMap<String, R>> for FunctionalPluginAction<A, R> {
    fn run(
        &self,
        action_data: A,
        plugin_data: &HashMap<String, Box<dyn PluginData>>,
    ) -> HashMap<String, R> {
        let mut returns: HashMap<String, R> = HashMap::new();
        for (plugin_name, runner) in &self.runners {
            let ret = runner(
                &action_data,
                // We must have data registered for every active plugin (even if it's empty)
                plugin_data.get(plugin_name).unwrap_or_else(|| {
                    panic!("no plugin data for registered plugin {}", plugin_name)
                }),
            );
            returns.insert(plugin_name.to_string(), ret);
        }

        returns
    }
    fn register_plugin(&mut self, name: String, runner: Runner<A, R>) {
        self.runners.insert(name, runner);
    }
}
// Using a default implementation allows us to avoid the action data having to implement `Default` as well, which is frequently infeasible
impl<A, R> Default for FunctionalPluginAction<A, R> {
    fn default() -> Self {
        Self {
            runners: HashMap::default(),
        }
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

/// The actions a functional plugin can take that pertain to the build process. Note that these actions are not available for the build
/// stage of the export process, and those should be registered separately.
#[derive(Default)]
pub struct FunctionalPluginBuildActions {
    /// Runs before the build process.
    pub before_build: FunctionalPluginAction<(), ()>,
    /// Runs after the build process if it completes successfully.
    pub after_successful_build: FunctionalPluginAction<(), ()>,
    /// Runs after the build process if it fails.
    pub after_failed_build: FunctionalPluginAction<crate::errors::ServerError, ()>,
}
/// The actions a functional plugin can take that pertain to the export process.
#[derive(Default)]
pub struct FunctionalPluginExportActions {
    /// Runs before the export process.
    pub before_export: FunctionalPluginAction<(), ()>,
    /// Runs after the build stage in the export process if it completes successfully.
    pub after_successful_build: FunctionalPluginAction<(), ()>,
    /// Runs after the build stage in the export process if it fails.
    pub after_failed_build: FunctionalPluginAction<crate::errors::ServerError, ()>,
    /// Runs after the export process if it fails.
    pub after_failed_export: FunctionalPluginAction<crate::errors::ServerError, ()>,
    /// Runs if copying the static directory failed.
    pub after_failed_static_copy: FunctionalPluginAction<fs_extra::error::Error, ()>,
    /// Runs if copying a static alias that was a directory failed.
    pub after_failed_static_alias_dir_copy: FunctionalPluginAction<fs_extra::error::Error, ()>,
    /// Runs if copying a static alias that was a file failed.
    pub after_failed_static_alias_file_copy: FunctionalPluginAction<std::io::Error, ()>,
    /// Runs after the export process if it completes successfully.
    pub after_successful_export: FunctionalPluginAction<(), ()>,
}
/// The actions a functional plugin can take that pertain to the server.
#[derive(Default)]
pub struct FunctionalPluginServerActions {
    /// Runs before the server activates. This runs AFTER the current directory has been appropriately set for a standalone binary vs
    /// running in the development environment (inside `.perseus/`).
    pub before_serve: FunctionalPluginAction<(), ()>,
    // TODO
    // /// Runs after the server has been configured for Perseus, allowing further configuration to take place. In nearly all cases, this
    // /// will be useless, as Perseus adds a wildcard handler, so any routes here will be ignored by Actix Web.
    // pub configure_server_after_perseus: FunctionalPluginAction<(), ()>,
    // /// Runs before the server has been configured by Perseus, allowing the setting of custom API routes before a wildcard handler is
    // /// put in place by Perseus.
    // pub configure_server_before_perseus: FunctionalPluginAction<(), ()>,
}
/// The actions a functional plugin can take that pertain to the client-side code. These in particular should be as fast as possible.
#[derive(Default)]
pub struct FunctionalPluginClientActions {
    /// Runs before anything else in the browser. Note that this runs after panics have been set to go to the console.
    pub start: FunctionalPluginAction<(), ()>,
}
