use crate::plugins::*;
use crate::GenericNode;

/// A plugin, either functional or control.
pub struct Plugin<G: GenericNode> {
    /// The machine name of the plugin, which will be used as a key in a HashMap with many other plugins. This should be the public
    /// crate name in all cases.
    pub name: String,
    /// The type of the plugin. If this is a control plugin, the control actions will be attached.
    pub plugin_type: PluginType,
    /// A function that will be provided functional plugin actions. It should then register runners from the plugin for every action
    /// that it takes.
    pub functional_actions_registrar:
        Box<dyn Fn(FunctionalPluginActions<G>) -> FunctionalPluginActions<G>>,
}

/// A plugin type, with the relevant associated actions.
pub enum PluginType {
    /// A functional plugin. These can only take actions that supplement core functionality, and are designed to be compatible with
    /// other plugins such that two plugins can take the same actions.
    Functional,
    /// A control plugin. These can do everything a functional plugin can, but they can also replace core functionality entirely, in
    /// which case only a single plugin can take a single control action. This has an attached function that registers runners for
    /// control actions.
    Control(Box<dyn Fn(ControlPluginActions) -> ControlPluginActions>),
}
