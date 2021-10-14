use std::any::Any;
use std::collections::HashMap;

/// A representation of the data that a plugin takes. This will be provided to all its actions, though the type must be downcast first.
/// This is implemented for anything that implemented `Any`.
pub trait PluginData: Any {}
impl<T> PluginData for T where T: Any {}
/// A runner function, which takes action data and plugin data.
pub type Runner<A, R> = Box<dyn Fn(&A, &Box<dyn PluginData>) -> R>;

/// A trait for the interface for a plugin action, which abstracts whether it's a functional plugin or a control plugin.
pub trait PluginAction<A, R, R2> {
    /// Runs the action. This takes data that the action should expect, along with a map of plugins to their data.
    fn run(&self, action_data: A, plugin_data: &HashMap<String, Box<dyn PluginData>>) -> R2;
    /// Registers a plugin that takes this action.
    ///
    /// # Panics
    /// If the action type can only be taken by one plugin, and one has already been set, this may panic (e.g. for control plugins),
    /// as this is a critical, unrecoverable error that Perseus doesn't need to do anything after. If a plugin registration fails,
    /// we have to assume that all work in the engine may be not what the user intended.
    fn register_plugin(
        &mut self,
        name: &str,
        runner: impl Fn(&A, &Box<dyn PluginData>) -> R + 'static,
    );
    /// Same as `.register_plugin()`, but takes a prepared runner in a `Box`.
    fn register_plugin_box(&mut self, name: &str, runner: Runner<A, R>);
}
