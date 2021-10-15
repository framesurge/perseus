use crate::plugins::{PluginAction, Runner};
use crate::GenericNode;
use std::any::Any;
use std::collections::HashMap;

/// An action for a functional plugin, which can be taken by many plugins. When run, a functional action will return a map of plugin
/// names to their return types.
pub struct FunctionalPluginAction<A, R> {
    /// The runners that will be called when this action is run.
    runners: HashMap<String, Runner<A, R>>,
}
impl<A, R> PluginAction<A, R, HashMap<String, R>> for FunctionalPluginAction<A, R> {
    fn run(
        &self,
        action_data: A,
        plugin_data: &HashMap<String, Box<dyn Any>>,
    ) -> HashMap<String, R> {
        let mut returns: HashMap<String, R> = HashMap::new();
        for (plugin_name, runner) in &self.runners {
            let ret = runner(
                &action_data,
                // We must have data registered for every active plugin (even if it's empty)
                &**plugin_data.get(plugin_name).unwrap_or_else(|| {
                    panic!("no plugin data for registered plugin {}", plugin_name)
                }),
            );
            returns.insert(plugin_name.to_string(), ret);
        }

        returns
    }
    fn register_plugin(&mut self, name: &str, runner: impl Fn(&A, &dyn Any) -> R + 'static) {
        self.register_plugin_box(name, Box::new(runner))
    }
    fn register_plugin_box(&mut self, name: &str, runner: Runner<A, R>) {
        self.runners.insert(name.to_string(), runner);
    }
}
// Using a default implementation allows us to avoid the action data having to implement `Default` as well, which is frequently infeasible
impl<A, R> Default for FunctionalPluginAction<A, R> {
    fn default() -> Self {
        Self {
            runners: HashMap::default(),
        }
    }
}

/// All the actions that a functional plugin can perform. These are all designed to be compatible with other plugins such that two plugins
/// can execute the same action.
pub struct FunctionalPluginActions<G: GenericNode> {
    /// The all-powerful action that can modify the Perseus engine itself. Because modifying the code you're running doesn't work with
    /// compiled languages like Rust, this has its own command in the CLI, `perseus tinker`. This is best used for modifying
    /// `.perseus/Cargo.toml` or other files. Ensure that you add signal comments so you don't apply the same modifications twice!
    /// This will be executed in the context of `.perseus/`. As usual, do NOT change the directory here, because that will affect every
    /// other plugin as well, just use `../`s if you need to work outside `.perseus/`.
    ///
    /// If your plugin uses this action in a way that may confuse other plugins, you should note this in your documentation.
    pub tinker: FunctionalPluginAction<(), ()>,
    /// Actions pertaining to the modification of settings created with the `define_app!` macro.
    pub settings_actions: FunctionalPluginSettingsActions<G>,
    /// Actions pertaining to the build process.
    pub build_actions: FunctionalPluginBuildActions,
    /// Actions pertaining to the export process.
    pub export_actions: FunctionalPluginExportActions,
    /// Actions pertaining to the server.
    pub server_actions: FunctionalPluginServerActions,
    /// Actions pertaining to the client-side code.
    pub client_actions: FunctionalPluginClientActions,
}
impl<G: GenericNode> Default for FunctionalPluginActions<G> {
    fn default() -> Self {
        Self {
            tinker: FunctionalPluginAction::default(),
            settings_actions: FunctionalPluginSettingsActions::<G>::default(),
            build_actions: FunctionalPluginBuildActions::default(),
            export_actions: FunctionalPluginExportActions::default(),
            server_actions: FunctionalPluginServerActions::default(),
            client_actions: FunctionalPluginClientActions::default(),
        }
    }
}

/// The actions a functional plugin can take that pertain to altering the settings exported from the `define_app!` macro.
pub struct FunctionalPluginSettingsActions<G: GenericNode> {
    /// Adds additional static aliases. Note that a static alias is a mapping of a URL path to a filesystem path (relative to the
    /// project root). These will be vetted to ensure they don't access anything outside the project root for security reasons. If they
    /// do, the user's app will not run. Note that these have the power to override the user's static aliases.
    pub add_static_aliases: FunctionalPluginAction<(), HashMap<String, String>>,
    /// Adds additional templates. These will be applied to both the templates map and the templates list (separate entities), and
    /// they must be generic about Sycamore rendering backends. Note that these have the power to override the user's templates.
    pub add_templates: FunctionalPluginAction<(), Vec<crate::Template<G>>>,
    /// Adds additional error pages. This must return a map of HTTP status codes to erro page templates. Note that these have the
    /// power to override the user's error pages.
    pub add_error_pages:
        FunctionalPluginAction<(), HashMap<u16, crate::error_pages::ErrorPageTemplate<G>>>,
}
impl<G: GenericNode> Default for FunctionalPluginSettingsActions<G> {
    fn default() -> Self {
        Self {
            add_static_aliases: FunctionalPluginAction::default(),
            add_templates: FunctionalPluginAction::default(),
            add_error_pages: FunctionalPluginAction::default(),
        }
    }
}

/// The actions a functional plugin can take that pertain to the build process. Note that these actions are not available for the build
/// stage of the export process, and those should be registered separately.
#[derive(Default)]
pub struct FunctionalPluginBuildActions {
    /// Runs before the build process.
    pub before_build: FunctionalPluginAction<(), ()>,
    /// Runs after the build process if it completes successfully.
    pub after_successful_build: FunctionalPluginAction<(), ()>,
    /// Runs after the build process if it fails.
    pub after_failed_build: FunctionalPluginAction<crate::errors::ServerError, ()>,
}
/// The actions a functional plugin can take that pertain to the export process.
#[derive(Default)]
pub struct FunctionalPluginExportActions {
    /// Runs before the export process.
    pub before_export: FunctionalPluginAction<(), ()>,
    /// Runs after the build stage in the export process if it completes successfully.
    pub after_successful_build: FunctionalPluginAction<(), ()>,
    /// Runs after the build stage in the export process if it fails.
    pub after_failed_build: FunctionalPluginAction<crate::errors::ServerError, ()>,
    /// Runs after the export process if it fails.
    pub after_failed_export: FunctionalPluginAction<crate::errors::ServerError, ()>,
    /// Runs if copying the static directory failed.
    pub after_failed_static_copy: FunctionalPluginAction<fs_extra::error::Error, ()>,
    /// Runs if copying a static alias that was a directory failed.
    pub after_failed_static_alias_dir_copy: FunctionalPluginAction<fs_extra::error::Error, ()>,
    /// Runs if copying a static alias that was a file failed.
    pub after_failed_static_alias_file_copy: FunctionalPluginAction<std::io::Error, ()>,
    /// Runs after the export process if it completes successfully.
    pub after_successful_export: FunctionalPluginAction<(), ()>,
}
/// The actions a functional plugin can take that pertain to the server.
#[derive(Default)]
pub struct FunctionalPluginServerActions {
    /// Runs before the server activates. This runs AFTER the current directory has been appropriately set for a standalone binary vs
    /// running in the development environment (inside `.perseus/`).
    pub before_serve: FunctionalPluginAction<(), ()>,
}
/// The actions a functional plugin can take that pertain to the client-side code. These in particular should be as fast as possible.
#[derive(Default)]
pub struct FunctionalPluginClientActions {
    /// Runs before anything else in the browser. Note that this runs after panics have been set to go to the console.
    pub start: FunctionalPluginAction<(), ()>,
}
