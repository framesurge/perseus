use crate::plugins::*;
use crate::GenericNode;
use std::any::Any;
use std::marker::PhantomData;

type FunctionalActionsRegistrar<G> =
    Box<dyn Fn(FunctionalPluginActions<G>) -> FunctionalPluginActions<G>>;

/// A plugin, either functional or control.
pub struct Plugin<G: GenericNode, D: Any> {
    /// The machine name of the plugin, which will be used as a key in a HashMap with many other plugins. This should be the public
    /// crate name in all cases.
    pub name: String,
    /// The type of the plugin. If this is a control plugin, the control actions will be attached.
    pub plugin_type: PluginType,
    /// A function that will be provided functional plugin actions. It should then register runners from the plugin for every action
    /// that it takes.
    pub functional_actions_registrar: FunctionalActionsRegistrar<G>,

    plugin_data_type: PhantomData<D>,
}
impl<G: GenericNode, D: Any> Plugin<G, D> {
    /// Creates a new plugin with a name, functional actions, and optional control actions.
    pub fn new(
        name: &str,
        plugin_type: PluginType,
        functional_actions_registrar: impl Fn(FunctionalPluginActions<G>) -> FunctionalPluginActions<G>
            + 'static,
    ) -> Self {
        Self {
            name: name.to_string(),
            plugin_type,
            functional_actions_registrar: Box::new(functional_actions_registrar),
            plugin_data_type: PhantomData::default(),
        }
    }
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
