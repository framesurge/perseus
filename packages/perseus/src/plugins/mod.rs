mod action;
mod control;
mod functional;
mod plugin;
mod plugins_list;

pub use action::{PluginAction, Runner};
pub use control::*;
pub use functional::*;
pub use plugin::{Plugin, PluginEnv};
pub use plugins_list::Plugins;

/// A helper function for plugins that don't take any functional actions. This
/// just inserts and empty registrar.
pub fn empty_functional_actions_registrar<G: crate::Html>(
    _: FunctionalPluginActions,
) -> FunctionalPluginActions {
    FunctionalPluginActions::default()
}

/// A helper function for plugins that don't take any control actions. This just
/// inserts an empty registrar.
pub fn empty_control_actions_registrar(_: ControlPluginActions) -> ControlPluginActions {
    ControlPluginActions::default()
}
