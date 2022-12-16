use crate::plugins::*;
use std::any::Any;
use std::collections::HashMap;

type PluginDataMap = HashMap<String, Box<dyn Any + Send + Sync>>;

/// A representation of all the plugins used by an app.
///
/// Due to the sheer number and complexity of nested fields, this is best
/// transferred in an `Rc`, which unfortunately results in double indirection
/// for runner functions.
pub struct Plugins {
    /// The functional actions that this plugin takes. This is defined by
    /// default such that all actions are assigned to a default, and so they
    /// can all be run without long chains of matching `Option<T>`s.
    pub functional_actions: FunctionalPluginActions,
    /// The control actions that this plugin takes. This is defined by default
    /// such that all actions are assigned to a default, and so they can all
    /// be run without long chains of matching `Option<T>`s.
    pub control_actions: ControlPluginActions,
    plugin_data: PluginDataMap,
}
impl Default for Plugins {
    fn default() -> Self {
        Self {
            functional_actions: FunctionalPluginActions::default(),
            control_actions: ControlPluginActions::default(),
            plugin_data: HashMap::default(),
        }
    }
}
impl std::fmt::Debug for Plugins {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Plugins")
            .field("functional_actions", &self.functional_actions)
            .field("control_actions", &self.control_actions)
            .finish()
    }
}
impl Plugins {
    /// Creates a new instance of `Plugins`, with no actions taken by any
    /// plugins, and the data map empty.
    pub fn new() -> Self {
        Self::default()
    }
    /// Registers a new plugin, consuming `self`. For control actions, this will
    /// check if a plugin has already registered on an action, and throw an
    /// error if one has, noting the conflict explicitly in the error message.
    /// This can only register plugins that run exclusively on the
    /// server-side (including tinker-time and the build process).
    // We allow unused variables and the like for linting because otherwise any
    // errors in Wasm compilation will show these up, which is annoying
    pub fn plugin<D: Any + Send + Sync>(
        #[cfg_attr(target_arch = "wasm32", allow(unused_mut))] mut self,
        // This is a function so that it never gets called if we're compiling for Wasm, which means
        // Rust eliminates it as dead code!
        #[cfg_attr(target_arch = "wasm32", allow(unused_variables))] plugin: impl Fn() -> Plugin<D>
            + Send
            + Sync,
        #[cfg_attr(target_arch = "wasm32", allow(unused_variables))] plugin_data: D,
    ) -> Self {
        // If we're compiling for Wasm, plugins that don't run on the client side
        // shouldn't be added (they'll then be eliminated as dead code)
        #[cfg(not(target_arch = "wasm32"))]
        {
            let plugin = plugin();
            // If the plugin can run on the client-side, it should use `.client_plugin()`
            // for verbosity We don;t have access to the actual plugin data
            // until now, so we can;t do this at the root of the function
            if plugin.env != PluginEnv::Server {
                panic!("attempted to register plugin that can run on the client with `.plugin()`, this plugin should be registered with `.plugin_with_client_privilege()` (this will increase your final bundle size)")
            }
            // Insert the plugin data
            let plugin_data: Box<dyn Any + Send + Sync> = Box::new(plugin_data);
            let res = self.plugin_data.insert(plugin.name.clone(), plugin_data);
            // If there was an old value, there are two plugins with the same name, which is
            // very bad (arbitrarily inconsistent behavior overriding)
            if res.is_some() {
                panic!("two plugins have the same name '{}', which could lead to arbitrary and inconsistent behavior modification (please file an issue with the plugin that doesn't have the same name as its crate)", &plugin.name);
            }
            // Register functional and control actions using the plugin's provided registrar
            self.functional_actions =
                (plugin.functional_actions_registrar)(self.functional_actions);
            self.control_actions = (plugin.control_actions_registrar)(self.control_actions);
        }

        self
    }
    /// The same as `.plugin()`, but registers a plugin that can run on the
    /// client-side. This is deliberately separated out to make conditional
    /// compilation feasible and to emphasize to users what's increasing their
    /// bundle sizes. Note that this should also be used for plugins that
    /// run on both the client and server.
    pub fn plugin_with_client_privilege<D: Any + Send + Sync>(
        mut self,
        // This is a function to preserve a similar API interface with `.plugin()`
        plugin: impl Fn() -> Plugin<D> + Send + Sync,
        plugin_data: D,
    ) -> Self {
        let plugin = plugin();
        // If the plugin doesn't need client privileges, it shouldn't have them (even
        // though this is just a semantic thing)
        if plugin.env == PluginEnv::Server {
            panic!("attempted to register plugin that doesn't ever run on the client with `.plugin_with_client_privilege()`, you should use `.plugin()` instead")
        }
        // Insert the plugin data
        let plugin_data: Box<dyn Any + Send + Sync> = Box::new(plugin_data);
        let res = self.plugin_data.insert(plugin.name.clone(), plugin_data);
        // If there was an old value, there are two plugins with the same name, which is
        // very bad (arbitrarily inconsistent behavior overriding)
        if res.is_some() {
            panic!("two plugins have the same name '{}', which could lead to arbitrary and inconsistent behavior modification (please file an issue with the plugin that doesn't have the same name as its crate)", &plugin.name);
        }
        // Register functional and control actions using the plugin's provided registrar
        self.functional_actions = (plugin.functional_actions_registrar)(self.functional_actions);
        self.control_actions = (plugin.control_actions_registrar)(self.control_actions);

        self
    }
    /// Gets a reference to the map of plugin data. Note that each element of
    /// plugin data is additionally `Box`ed.
    pub fn get_plugin_data(&self) -> &PluginDataMap {
        &self.plugin_data
    }
}
