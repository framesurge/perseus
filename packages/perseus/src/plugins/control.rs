use crate::plugins::*;
use std::any::Any;
use std::collections::HashMap;

/// A control action, which can only be taken by one plugin. When run, control actions will return an `Option<R>` on what
/// their runners return, which will be `None` if no runner is set.
pub struct ControlPluginAction<A, R> {
    /// The name of the plugin that controls this action. As this is a control action, only one plugin can manage a single action.
    controller_name: String,
    /// The single runner function for this action. This may not be defined if no plugin takes this action.
    runner: Option<Runner<A, R>>,
}
impl<A, R> PluginAction<A, R, Option<R>> for ControlPluginAction<A, R> {
    /// Runs the single registered runner for the action.
    fn run(&self, action_data: A, plugin_data: &HashMap<String, Box<dyn Any + Send>>) -> Option<R> {
        // If no runner is defined, this won't have any effect (same as functional actions with no registered runners)
        self.runner.as_ref().map(|runner| {
            runner(
                &action_data,
                // We must have data registered for every active plugin (even if it's empty)
                &**plugin_data.get(&self.controller_name).unwrap_or_else(|| {
                    panic!(
                        "no plugin data for registered plugin {}",
                        &self.controller_name
                    )
                }),
            )
        })
    }
    fn register_plugin(
        &mut self,
        name: &str,
        runner: impl Fn(&A, &(dyn Any + Send)) -> R + Send + 'static,
    ) {
        self.register_plugin_box(name, Box::new(runner))
    }
    fn register_plugin_box(&mut self, name: &str, runner: Runner<A, R>) {
        // Check if the action has already been taken by another plugin
        if self.runner.is_some() {
            // We panic here because an explicitly requested plugin couldn't be loaded, so we have to assume that any further behavior in the engine is unwanted
            // Therefore, a graceful error would be inappropriate, this is critical in every sense
            panic!("attempted to register runner from plugin '{}' for control action that already had a registered runner from plugin '{}' (these plugins conflict, see the book for further details)", name, self.controller_name);
        }

        self.controller_name = name.to_string();
        self.runner = Some(runner);
    }
}
// Using a default implementation allows us to avoid the action data having to implement `Default` as well, which is frequently infeasible
impl<A, R> Default for ControlPluginAction<A, R> {
    fn default() -> Self {
        Self {
            controller_name: String::default(),
            runner: None,
        }
    }
}

/// All the control actions that a plugin can take.
#[derive(Default)]
pub struct ControlPluginActions {
    /// Actions pertaining to the modification of settings created with the `define_app!` macro.
    pub settings_actions: ControlPluginSettingsActions,
    /// Actions pertaining to the build process.
    pub build_actions: ControlPluginBuildActions,
    /// Actions pertaining to the export process.
    pub export_actions: ControlPluginExportActions,
    /// Actions pertaining to the server.
    pub server_actions: ControlPluginServerActions,
    /// Actions pertaining to the client-side code.
    pub client_actions: ControlPluginClientActions,
}

/// Control actions that pertain to altering settings from `define_app!`.
#[derive(Default)]
pub struct ControlPluginSettingsActions {
    /// Sets an immutable store to be used everywhere. This will provided the current immutable store for reference.
    pub set_immutable_store:
        ControlPluginAction<crate::stores::ImmutableStore, crate::stores::ImmutableStore>,
    /// Sets the locales to be used everywhere, providing the current ones for reference.
    pub set_locales: ControlPluginAction<crate::locales::Locales, crate::locales::Locales>,
    /// Sets the app root to be used everywhere. This must correspond to the ID of an empty HTML `div`.
    pub set_app_root: ControlPluginAction<(), String>,
}
/// Control actions that pertain to the build process.
#[derive(Default)]
pub struct ControlPluginBuildActions {}
/// Control actions that pertain to the export process.
#[derive(Default)]
pub struct ControlPluginExportActions {}
/// Control actions that pertain to the server.
#[derive(Default)]
pub struct ControlPluginServerActions {}
/// Control actions that pertain to the client-side code. As yet, there are none of these.
#[derive(Default)]
pub struct ControlPluginClientActions {}
