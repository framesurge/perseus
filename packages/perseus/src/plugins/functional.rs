use super::*;
#[cfg(engine)]
use crate::errors::Error;
use crate::errors::PluginError;
use std::any::Any;
use std::collections::HashMap;
#[cfg(engine)]
use std::sync::Arc;

/// An action which can be taken by many plugins. When run, a functional action
/// will return a map of plugin names to their return types.
pub struct FunctionalPluginAction<A, R> {
    /// The runners that will be called when this action is run.
    runners: HashMap<String, Runner<A, R>>,
}
impl<A, R> PluginAction<A, R, HashMap<String, R>> for FunctionalPluginAction<A, R> {
    fn run(
        &self,
        action_data: A,
        plugin_data: &HashMap<String, Box<dyn Any + Send + Sync>>,
    ) -> Result<HashMap<String, R>, PluginError> {
        let mut returns = HashMap::new();
        for (plugin_name, runner) in &self.runners {
            let ret = runner(
                &action_data,
                // We must have data registered for every active plugin (even if it's empty)
                &**plugin_data.get(plugin_name).unwrap_or_else(|| {
                    panic!("no plugin data for registered plugin {}", plugin_name)
                }),
            )
            .map_err(|err| PluginError {
                name: plugin_name.to_string(),
                source: err,
            })?;
            returns.insert(plugin_name.to_string(), ret);
        }

        Ok(returns)
    }
    fn register_plugin(
        &mut self,
        name: &str,
        runner: impl Fn(&A, &(dyn Any + Send + Sync)) -> Result<R, Box<dyn std::error::Error + Send + Sync>>
            + Send
            + Sync
            + 'static,
    ) {
        self.register_plugin_box(name, Box::new(runner))
    }
    fn register_plugin_box(&mut self, name: &str, runner: Runner<A, R>) {
        self.runners.insert(name.to_string(), runner);
    }
}
// Using a default implementation allows us to avoid the action data having to
// implement `Default` as well, which is frequently infeasible
impl<A, R> Default for FunctionalPluginAction<A, R> {
    fn default() -> Self {
        Self {
            runners: HashMap::default(),
        }
    }
}
impl<A, R> std::fmt::Debug for FunctionalPluginAction<A, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionalPluginAction")
            .field("runners", &"HashMap<String, Runner>")
            .finish()
    }
}

/// Actions designed to be compatible with other plugins such that two plugins
/// can execute the same action.
#[derive(Debug, Default)]
pub struct FunctionalPluginActions {
    /// The all-powerful action that can modify the Perseus engine itself.
    /// Because modifying the code you're running doesn't work with compiled
    /// languages like Rust, this has its own command in the CLI, `perseus
    /// tinker`. This is best used for modifying `.perseus/Cargo.toml` or
    /// other files. Ensure that you add signal comments so you don't apply the
    /// same modifications twice! This will be executed in the context of
    /// `.perseus/`. As usual, do NOT change the directory here, because that
    /// will affect every other plugin as well, just use `../`s if you need
    /// to work outside `.perseus/`.
    ///
    /// If your plugin uses this action in a way that may confuse other plugins,
    /// you should note this in your documentation.
    pub tinker: FunctionalPluginAction<(), ()>,
    /// Actions pertaining to the modification of settings created with
    /// `PerseusApp`.
    pub settings_actions: FunctionalPluginSettingsActions,
    /// Actions pertaining to the build process.
    #[cfg(engine)]
    pub build_actions: FunctionalPluginBuildActions,
    /// Actions pertaining to the export process.
    #[cfg(engine)]
    pub export_actions: FunctionalPluginExportActions,
    /// Actions pertaining to the process of exporting an error page.
    #[cfg(engine)]
    pub export_error_page_actions: FunctionalPluginExportErrorPageActions,
    /// Actions pertaining to the server.
    pub server_actions: FunctionalPluginServerActions,
    /// Actions pertaining to the client-side code.
    pub client_actions: FunctionalPluginClientActions,
}

/// Functional actions that pertain to altering the settings exported from
/// `PerseusApp`.
#[derive(Debug, Default)]
pub struct FunctionalPluginSettingsActions {
    /// Adds additional static aliases. Note that a static alias is a mapping of
    /// a URL path to a filesystem path (relative to the project root).
    /// These will be vetted to ensure they don't access anything outside the
    /// project root for security reasons. If they do, the user's app will
    /// not run. Note that these have the power to override the user's static
    /// aliases.
    pub add_static_aliases: FunctionalPluginAction<(), HashMap<String, String>>,
    /// Actions pertaining to the HTML shell, in their own category for
    /// cleanliness (as there are quite a few).
    pub html_shell_actions: FunctionalPluginHtmlShellActions,
}

/// Functional actions that pertain to the HTML shell.
///
/// **IMPORTANT:** The HTML shell's `<head>` contains an *interpolation
/// boundary*, after which all content is wiped between page loads. If you want
/// the code you add (HTML or JS) to persist between pages (which you usually
/// will), make sure to use the `..._before_boundary` actions.
#[derive(Default, Debug)]
pub struct FunctionalPluginHtmlShellActions {
    /// Adds to the additional HTML content in the document `<head>` before the
    /// interpolation boundary.
    pub add_to_head_before_boundary: FunctionalPluginAction<(), Vec<String>>,
    /// Adds JS code (which will be placed into a `<script>` block) before the
    /// interpolation boundary.
    pub add_to_scripts_before_boundary: FunctionalPluginAction<(), Vec<String>>,
    /// Adds to the additional HTML content in the document `<head>` after the
    /// interpolation boundary.
    pub add_to_head_after_boundary: FunctionalPluginAction<(), Vec<String>>,
    /// Adds Js code (which will places into a `<script>` block) after the
    /// interpolation boundary.
    pub add_to_scripts_after_boundary: FunctionalPluginAction<(), Vec<String>>,
    /// Adds arbitrary HTML to the document `<body>` before the Perseus app
    /// markup. This will persist across all pages of the app.
    pub add_to_before_content: FunctionalPluginAction<(), Vec<String>>,
    /// Adds arbitrary HTML to the document `<body>` after the Perseus app
    /// markup. This will persist across all pages of the app.
    pub add_to_after_content: FunctionalPluginAction<(), Vec<String>>,
}

/// Functional actions that pertain to the build process. Note that these
/// actions are not available for the build stage of the export process, and
/// those should be registered separately.
#[cfg(engine)]
#[derive(Default, Debug)]
pub struct FunctionalPluginBuildActions {
    /// Runs before the build process.
    pub before_build: FunctionalPluginAction<(), ()>,
    /// Runs after the build process if it completes successfully.
    pub after_successful_build: FunctionalPluginAction<(), ()>,
    /// Runs after the build process if it fails.
    pub after_failed_build: FunctionalPluginAction<Arc<Error>, ()>,
}
/// Functional actions that pertain to the export process.
#[cfg(engine)]
#[derive(Default, Debug)]
pub struct FunctionalPluginExportActions {
    /// Runs before the export process.
    pub before_export: FunctionalPluginAction<(), ()>,
    /// Runs after the build stage in the export process if it completes
    /// successfully.
    pub after_successful_build: FunctionalPluginAction<(), ()>,
    /// Runs after the build stage in the export process if it fails.
    pub after_failed_build: FunctionalPluginAction<Arc<Error>, ()>,
    /// Runs after the export process if it fails.
    pub after_failed_export: FunctionalPluginAction<Arc<Error>, ()>,
    /// Runs if copying the static directory failed.
    pub after_failed_static_copy: FunctionalPluginAction<Arc<Error>, ()>,
    /// Runs if copying a static alias that was a directory failed.
    pub after_failed_static_alias_dir_copy: FunctionalPluginAction<Arc<Error>, ()>,
    /// Runs if creating the directory structure for a nested static alias
    /// failed in exporting.
    pub after_failed_nested_static_alias_dir_creation: FunctionalPluginAction<Arc<Error>, ()>,
    /// Runs if copying a static alias that was a file failed. The argument to
    /// this is a tuple of the from and to locations of the copy, along with the
    /// error.
    pub after_failed_static_alias_file_copy: FunctionalPluginAction<Arc<Error>, ()>,
    /// Runs after the export process if it completes successfully.
    pub after_successful_export: FunctionalPluginAction<(), ()>,
}
/// Functional actions that pertain to the process of exporting an error page.
#[cfg(engine)]
#[derive(Default, Debug)]
pub struct FunctionalPluginExportErrorPageActions {
    /// Runs before the process of exporting an error page, providing the HTTP
    /// status code to be exported and the output filename (relative to the root
    /// of the project, not to `.perseus/`).
    pub before_export_error_page: FunctionalPluginAction<(u16, String), ()>,
    /// Runs after a error page was exported successfully.
    pub after_successful_export_error_page: FunctionalPluginAction<(), ()>,
    /// Runs if writing to the output file failed. Error and filename are given.
    pub after_failed_write: FunctionalPluginAction<Arc<Error>, ()>,
}
/// Functional actions that pertain to the server.
#[derive(Default, Debug)]
pub struct FunctionalPluginServerActions {
    /// Runs before the server activates. This runs AFTER the current directory
    /// has been appropriately set for a standalone binary vs running in the
    /// development environment (inside `.perseus/`).
    pub before_serve: FunctionalPluginAction<(), ()>,
}
/// Functional actions that pertain to the client-side code. These in particular
/// should be as fast as possible.
#[derive(Default, Debug)]
pub struct FunctionalPluginClientActions {
    /// Runs before anything else in the browser. Note that this runs after
    /// panics have been set to go to the console.
    pub start: FunctionalPluginAction<(), ()>,
    /// Runs in the event of a full Perseus crash. This is not a panic, but
    /// the experience of a critical error that prevents the instantiation of
    /// a reactor, router, or the like. This is an *excellent* opportunity for
    /// analytics to report that your app has completely failed (although the
    /// user will have been neatly prompted).
    ///
    /// If this panics, there isn't any explicit problem, but it's a bit rude
    /// to kick your app when it's down, don't you think? That said, the whole
    /// thing's about to blow up anyway.
    ///
    /// Any error responses here will lead to a panic.
    pub crash: FunctionalPluginAction<(), ()>,
}
