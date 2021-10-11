use crate::plugins::*;
use std::collections::HashMap;

type PluginDataMap = HashMap<String, Box<dyn PluginData>>;

/// A representation of all the plugins used by an app. Due to the sheer number and compexity of nested fields, this is best transferred
/// in an `Rc`, which unfortunately results in double indirection for runner functions.
#[derive(Default)]
pub struct Plugins {
    /// The actions that functional plugins can take. This is defined by default such that all actions are assigned to a default, and so
    /// they can all be run without long chains of matching `Option<T>`s.
    pub functional_actions: FunctionalPluginActions,
    /// The actions that control plugins can take. This is defined by default such that all actions are assigned to a default, and so
    /// they can all be run without long chains of matching `Option<T>`s.
    pub control_actions: ControlPluginActions,
    plugin_data: PluginDataMap,
}
impl Plugins {
    /// Creates a new instance of `Plugins`, with no actions taken by any plugins, and the data map empty.
    pub fn new() -> Self {
        Self::default()
    }
    /// Registers a new plugin, consuming `self`. For functional plugins, this will simply register the plugin on each of the actions
    /// that it takes. For control plugins, this will check if a plugin has already registered on an action, and throw an error if one
    /// has, noting the conflict explicitly in the error message.
    pub fn plugin(mut self, plugin: Plugin, plugin_data: impl PluginData) -> Self {
        // Insert the plugin data
        let plugin_data: Box<dyn PluginData> = Box::new(plugin_data);
        let res = self.plugin_data.insert(plugin.name.clone(), plugin_data);
        // If there was an old value, there are two plugins with the same name, which is very bad (arbitrarily inconsistent behavior overriding)
        if res.is_some() {
            panic!("two plugins have the same name '{}', which could lead to arbitrary and inconsistent behavior modification (please file an issue with the plugin that doesn't have the same name as its crate)", &plugin.name);
        }
        // Register functional actions using the plugin's provided registrar
        // This has no access to control actions
        self.functional_actions = (plugin.functional_actions_registrar)(self.functional_actions);

        // If this is a control plugin, register control actions as well with the plugin's provided registrar
        if let PluginType::Control(control_actions_registrar) = plugin.plugin_type {
            self.control_actions = (control_actions_registrar)(self.control_actions);
        }

        self
    }
    /// Gets a reference to the map of plugin data. Note that each element of plugin data is additionally `Box`ed.
    pub fn get_plugin_data(&self) -> &PluginDataMap {
        &self.plugin_data
    }
}
