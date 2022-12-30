use std::any::Any;
use std::collections::HashMap;

use crate::errors::PluginError;

/// A runner function, which takes action data and plugin data, returning a
/// `Result<R, Box<dyn Error>>`.
// A: some stuff the specific action gets
// dyn Any + Send: the plugin options
// R: the return type
pub type Runner<A, R> = Box<
    dyn Fn(&A, &(dyn Any + Send + Sync)) -> Result<R, Box<dyn std::error::Error + Send + Sync>>
        + Send
        + Sync,
>;

/// A trait for the interface for a plugin action, which abstracts whether it's
/// a functional or a control action.
///
/// `R2` here denotes the return type of the entire plugin series. For instance,
/// functional plugins return a `HashMap` of the results of each plugin.
pub trait PluginAction<A, R, R2>: Send + Sync {
    /// Runs the action. This takes data that the action should expect, along
    /// with a map of plugins to their data.
    ///
    /// If any of the underlying plugins whose runners are executed by this
    /// function return an error, the first error will be returned
    /// immediately, and further execution will be aborted. Since
    /// execution may happen in an arbitrary order, there is no guarantee that
    /// the same error will be thrown each time if multiple plugins are
    /// being used.
    fn run(
        &self,
        action_data: A,
        plugin_data: &HashMap<String, Box<dyn Any + Send + Sync>>,
    ) -> Result<R2, PluginError>;
    /// Registers a plugin that takes this action.
    ///
    /// # Panics
    /// If the action type can only be taken by one plugin, and one has already
    /// been set, this may panic (e.g. for control actions), as this is a
    /// critical, unrecoverable error that Perseus doesn't need to do anything
    /// after. If a plugin registration fails, we have to assume that all
    /// work in the engine may be not what the user intended.
    fn register_plugin(
        &mut self,
        name: &str,
        runner: impl Fn(&A, &(dyn Any + Send + Sync)) -> Result<R, Box<dyn std::error::Error + Send + Sync>>
            + Send
            + Sync
            + 'static,
    );
    /// Same as `.register_plugin()`, but takes a prepared runner in a `Box`.
    fn register_plugin_box(&mut self, name: &str, runner: Runner<A, R>);
}
