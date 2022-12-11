use crate::errors::PluginError;
use crate::plugins::*;
use std::any::Any;
use std::collections::HashMap;

/// A control action, which can only be taken by one plugin. When run, control
/// actions will return an `Option<R>` on what their runners return, which will
/// be `None` if no runner is set.
pub struct ControlPluginAction<A, R> {
    /// The name of the plugin that controls this action. As this is a control
    /// action, only one plugin can manage a single action.
    controller_name: String,
    /// The single runner function for this action. This may not be defined if
    /// no plugin takes this action.
    runner: Option<Runner<A, R>>,
}
impl<A, R> PluginAction<A, R, Option<R>> for ControlPluginAction<A, R> {
    /// Runs the single registered runner for the action.
    fn run(
        &self,
        action_data: A,
        plugin_data: &HashMap<String, Box<dyn Any + Send + Sync>>,
    ) -> Result<Option<R>, PluginError> {
        // If no runner is defined, this won't have any effect (same as functional
        // actions with no registered runners)
        self.runner
            .as_ref()
            .map(|runner| {
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
                .map_err(|err| PluginError {
                    name: self.controller_name.to_string(),
                    source: err,
                })
            })
            // Turn `Option<Result<T, E>>` -> `Result<Option<T>, E>`
            .transpose()
    }
    fn register_plugin(
        &mut self,
        name: &str,
        runner: impl Fn(&A, &(dyn Any + Send + Sync)) -> Result<R, Box<dyn std::error::Error + Send + Sync>>
            + Send
            + Sync
            + 'static,
    ) {
        self.register_plugin_box(name, Box::new(runner))
    }
    fn register_plugin_box(&mut self, name: &str, runner: Runner<A, R>) {
        // Check if the action has already been taken by another plugin
        if self.runner.is_some() {
            // We panic here because an explicitly requested plugin couldn't be loaded, so
            // we have to assume that any further behavior in the engine is unwanted
            // Therefore, a graceful error would be inappropriate, this is critical in every
            // sense
            panic!("attempted to register runner from plugin '{}' for control action that already had a registered runner from plugin '{}' (these plugins conflict, see the book for further details)", name, self.controller_name);
        }

        self.controller_name = name.to_string();
        self.runner = Some(runner);
    }
}
// Using a default implementation allows us to avoid the action data having to
// implement `Default` as well, which is frequently infeasible
impl<A, R> Default for ControlPluginAction<A, R> {
    fn default() -> Self {
        Self {
            controller_name: String::default(),
            runner: None,
        }
    }
}
impl<A, R> std::fmt::Debug for ControlPluginAction<A, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ControlPluginAction")
            .field("controller_name", &self.controller_name)
            .field("runner", &self.runner.as_ref().map(|_| "Runner"))
            .finish()
    }
}

/// All the control actions that a plugin can take.
#[derive(Default, Debug)]
pub struct ControlPluginActions {
    /// Actions pertaining to the modification of settings created with
    /// `PerseusApp`.
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

/// Control actions that pertain to altering settings from `PerseusApp`.
#[derive(Default, Debug)]
pub struct ControlPluginSettingsActions {
    /// Sets an immutable store to be used everywhere. This will provided the
    /// current immutable store for reference.
    pub set_immutable_store:
        ControlPluginAction<crate::stores::ImmutableStore, crate::stores::ImmutableStore>,
    /// Sets the locales to be used everywhere, providing the current ones for
    /// reference.
    pub set_locales: ControlPluginAction<crate::i18n::Locales, crate::i18n::Locales>,
    /// Sets the app root to be used everywhere. This must correspond to the ID
    /// of an empty HTML `div`.
    pub set_app_root: ControlPluginAction<(), String>,
    /// Actions pertaining to the HTML shell, partitioned away for deliberate
    /// inconvenience (you should almost never use these).
    pub html_shell_actions: ControlPluginHtmlShellActions,
}
/// Control actions that pertain to the HTML shell. Note that these actions
/// should be used extremely sparingly, as they are very rarely needed (see the
/// available functional actions for the HTML shell), and they can have
/// confusing side effects for CSS hierarchies, as well as potentially
/// interrupting Perseus' interpolation processes. Changing certain things
/// with these may break Perseus completely in certain cases!
#[derive(Default, Debug)]
pub struct ControlPluginHtmlShellActions {
    /// Overrides whatever the user provided as their HTML shell completely.
    /// Whatever you provide here MUST contain a `<head>` and a `<body>` at
    /// least, or Perseus will completely fail.
    pub set_shell: ControlPluginAction<(), String>,
}
/// Control actions that pertain to the build process.
#[derive(Default, Debug)]
pub struct ControlPluginBuildActions {}
/// Control actions that pertain to the export process.
#[derive(Default, Debug)]
pub struct ControlPluginExportActions {}
/// Control actions that pertain to the server.
#[derive(Default, Debug)]
pub struct ControlPluginServerActions {}
/// Control actions that pertain to the client-side code. As yet, there are none
/// of these.
#[derive(Default, Debug)]
pub struct ControlPluginClientActions {}
