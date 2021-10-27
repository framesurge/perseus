use crate::plugins::*;
use crate::GenericNode;
use std::any::Any;
use std::marker::PhantomData;

type FunctionalActionsRegistrar<G> =
    Box<dyn Fn(FunctionalPluginActions<G>) -> FunctionalPluginActions<G>>;
type ControlActionsRegistrar = Box<dyn Fn(ControlPluginActions) -> ControlPluginActions>;

/// A Perseus plugin. This must be exported by all plugin crates so the user can register the plugin easily.
pub struct Plugin<G: GenericNode, D: Any> {
    /// The machine name of the plugin, which will be used as a key in a HashMap with many other plugins. This should be the public
    /// crate name in all cases.
    pub name: String,
    /// A function that will be provided functional actions. It should then register runners from the plugin for every action that it
    /// takes.
    pub functional_actions_registrar: FunctionalActionsRegistrar<G>,
    /// A function that will be provided control actions. It should then register runners from the plugin for every action
    /// that it takes.
    pub control_actions_registrar: ControlActionsRegistrar,
    /// Whether or not the plugin is tinker-only, in which case it will only be loaded at tinker-time, and will not be sent to the client.
    pub is_tinker_only: bool,

    plugin_data_type: PhantomData<D>,
}
impl<G: GenericNode, D: Any> Plugin<G, D> {
    /// Creates a new plugin with a name, functional actions, control actions, and whether or not the plugin is tinker-only.
    pub fn new(
        name: &str,
        functional_actions_registrar: impl Fn(FunctionalPluginActions<G>) -> FunctionalPluginActions<G>
            + 'static,
        control_actions_registrar: impl Fn(ControlPluginActions) -> ControlPluginActions + 'static,
        is_tinker_only: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            functional_actions_registrar: Box::new(functional_actions_registrar),
            control_actions_registrar: Box::new(control_actions_registrar),
            is_tinker_only,
            plugin_data_type: PhantomData::default(),
        }
    }
}
