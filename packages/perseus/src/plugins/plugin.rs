use crate::plugins::*;
use std::any::Any;
use std::marker::PhantomData;

type FunctionalActionsRegistrar = Box<dyn Fn(FunctionalPluginActions) -> FunctionalPluginActions>;
type ControlActionsRegistrar = Box<dyn Fn(ControlPluginActions) -> ControlPluginActions>;

/// The environments a plugin can run in. These will affect Wasm bundle size.
#[derive(PartialEq, Eq, Debug)]
pub enum PluginEnv {
    /// The plugin should only run on the client-side, and will be included in
    /// the final Wasm binary. More specifically, the plugin will only be
    /// included if the target architecture is `wasm32`.
    Client,
    /// The plugin should only run on the server-side (this includes tinker-time
    /// and the build process), and will NOT be included in the final Wasm
    /// binary. This will decrease binary sizes, and should be preferred in most
    /// cases.
    Server,
    /// The plugin will ruin everywhere, and will be included in the final Wasm
    /// binary.
    Both,
}

/// A Perseus plugin. This must be exported by all plugin crates so the user can
/// register the plugin easily.
pub struct Plugin<D: Any + Send + Sync> {
    /// The machine name of the plugin, which will be used as a key in a HashMap
    /// with many other plugins. This should be the public crate name in all
    /// cases.
    pub name: String,
    /// A function that will be provided functional actions. It should then
    /// register runners from the plugin for every action that it takes.
    pub functional_actions_registrar: FunctionalActionsRegistrar,
    /// A function that will be provided control actions. It should then
    /// register runners from the plugin for every action that it takes.
    pub control_actions_registrar: ControlActionsRegistrar,
    /// The environment that the plugin should run in.
    pub env: PluginEnv,

    plugin_data_type: PhantomData<D>,
}
impl<D: Any + Send + Sync> std::fmt::Debug for Plugin<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Plugin")
            .field("name", &self.name)
            .field("env", &self.env)
            .finish()
    }
}
impl<D: Any + Send + Sync> Plugin<D> {
    /// Creates a new plugin with a name, functional actions, control actions,
    /// and whether or not the plugin is tinker-only.
    pub fn new(
        name: &str,
        functional_actions_registrar: impl Fn(FunctionalPluginActions) -> FunctionalPluginActions
            + 'static,
        control_actions_registrar: impl Fn(ControlPluginActions) -> ControlPluginActions + 'static,
        env: PluginEnv,
    ) -> Self {
        Self {
            name: name.to_string(),
            functional_actions_registrar: Box::new(functional_actions_registrar),
            control_actions_registrar: Box::new(control_actions_registrar),
            env,
            plugin_data_type: PhantomData::default(),
        }
    }
}
