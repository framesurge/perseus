mod action;
mod control;
mod functional;
mod plugin;
mod plugins_list;

pub use action::*;
pub use control::*;
pub use functional::*;
pub use plugin::*;
pub use plugins_list::*;

/// A helper function for plugins that don't take any control actions. This just inserts an empty registrar.
pub fn empty_control_actions_registrar(_: ControlPluginActions) -> ControlPluginActions {
    ControlPluginActions::default()
}
