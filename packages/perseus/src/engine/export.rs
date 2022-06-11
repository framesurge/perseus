use fs_extra::dir::{copy as copy_dir, CopyOptions};
use crate::errors::ServerError;
use crate::{PerseusApp, PluginAction, Plugins, SsrNode, internal::get_path_prefix_server};
use crate::build::{build_app, BuildProps};
use crate::export::{export_app, ExportProps};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use std::rc::Rc;

use crate::{PerseusAppBase, i18n::TranslationsManager, stores::MutableStore};
use crate::errors::*;


/// Exports the app to static files, given a `PerseusApp`. This is engine-agnostic, using the `exported` subfolder in the immutable store as a destination directory. By default
/// this will end up at `dist/exported/` (customizable through `PerseusApp`).
///
/// Note that this expects to be run in the root of the project.
pub async fn export<M: MutableStore, T: TranslationsManager>(app: PerseusAppBase<SsrNode, M, T>) -> Result<(), Rc<EngineError>> {
    let plugins = app.get_plugins();
    let static_aliases = app.get_static_aliases();
    // This won't have any trailing slashes (they're stripped by the immutable store initializer)
    let dest = format!("{}/exported", app.get_immutable_store().get_path());
    let static_dir = app.get_static_dir();

    build_and_export(app).await?;
    // After that's done, we can do two copy operations in parallel at least
    copy_static_aliases(&plugins, &static_aliases, &dest)?;
    copy_static_dir(&plugins, &static_dir, &dest)?;

    plugins
        .functional_actions
        .export_actions
        .after_successful_export
        .run((), plugins.get_plugin_data());

    Ok(())
}

/// Performs the building and exporting processes using the given app. This is fully engine-agnostic, using only the data provided in the given `PerseusApp`.
async fn build_and_export<M: MutableStore, T: TranslationsManager>(app: PerseusAppBase<SsrNode, M, T>) -> Result<(), Rc<EngineError>> {
    let plugins = app.get_plugins();

    plugins
        .functional_actions
        .build_actions
        .before_build
        .run((), plugins.get_plugin_data());

    let immutable_store = app.get_immutable_store();
    // We don't need this in exporting, but the build process does
    let mutable_store = app.get_mutable_store();
    let locales = app.get_locales();
    // Generate the global state
    let gsc = app.get_global_state_creator();
    let global_state = match gsc.get_build_state().await {
        Ok(global_state) => global_state,
        Err(err) => {
            let err: Rc<EngineError> = Rc::new(ServerError::GlobalStateError(err).into());
            plugins
                .functional_actions
                .export_actions
                .after_failed_global_state_creation
                .run(err.clone(), plugins.get_plugin_data());
            return Err(err);
        }
    };
    let templates_map = app.get_templates_map();
    let index_view_str = app.get_index_view_str();
    let root_id = app.get_root();
    // This consumes `self`, so we get it finally
    let translations_manager = app.get_translations_manager().await;

    // Build the site for all the common locales (done in parallel), denying any non-exportable features
    // We need to build and generate those artifacts before we can proceed on to exporting
    let build_res = build_app(BuildProps {
        templates: &templates_map,
        locales: &locales,
        immutable_store: &immutable_store,
        mutable_store: &mutable_store,
        translations_manager: &translations_manager,
        global_state: &global_state,
        exporting: true,
    })
    .await;
    if let Err(err) = build_res {
        let err: Rc<EngineError> = Rc::new(err.into());
        plugins
            .functional_actions
            .export_actions
            .after_failed_build
            .run(err.clone(), plugins.get_plugin_data());
        return Err(err);
    }
    plugins
        .functional_actions
        .export_actions
        .after_successful_build
        .run((), plugins.get_plugin_data());
    // The app has now been built, so we can safely instantiate the HTML shell (which needs access to the render config, generated in the above build step)
    // It doesn't matter if the type parameters here are wrong, this function doesn't use them
    let index_view =
        PerseusApp::get_html_shell(index_view_str, &root_id, &immutable_store, &plugins).await;
    // Turn the build artifacts into self-contained static files
    let export_res = export_app(ExportProps {
        templates: &templates_map,
        html_shell: index_view,
        locales: &locales,
        immutable_store: &immutable_store,
        translations_manager: &translations_manager,
        path_prefix: get_path_prefix_server(),
        global_state: &global_state,
    })
    .await;
    if let Err(err) = export_res {
        let err: Rc<EngineError> = Rc::new(err.into());
        plugins
            .functional_actions
            .export_actions
            .after_failed_export
            .run(err.clone(), plugins.get_plugin_data());
        return Err(err);
    }

    Ok(())
}

/// Copies the static aliases into a distribution directory at `dest` (no trailing `/`). This should be the root of the destination directory for the exported files.
/// Because this provides a customizable destination, it is fully engine-agnostic.
///
/// The error type here is a tuple of the location the asset was copied from, the location it was copied to, and the error in that process (which could be from `io` or
/// `fs_extra`).
fn copy_static_aliases(plugins: &Plugins<SsrNode>, static_aliases: &HashMap<String, String>, dest: &str) -> Result<(), Rc<EngineError>> {
    // Loop through any static aliases and copy them in too
    // Unlike with the server, these could override pages!
    // We'll copy from the alias to the path (it could be a directory or a file)
    // Remember: `alias` has a leading `/`!
    for (alias, path) in static_aliases {
        let from = PathBuf::from(path);
        let to = format!("{}{}", dest, alias);

        if from.is_dir() {
            if let Err(err) = copy_dir(&from, &to, &CopyOptions::new()) {
                let err = EngineError::CopyStaticAliasDirErr { source: err, to: to.to_string(), from: path.to_string() };
                let err = Rc::new(err);
                plugins
                    .functional_actions
                    .export_actions
                    .after_failed_static_alias_dir_copy
                    .run(err.clone(), plugins.get_plugin_data());
                return Err(err);
            }
        } else if let Err(err) = fs::copy(&from, &to) {
            let err = EngineError::CopyStaticAliasFileError { source: err, to: to.to_string(), from: path.to_string() };
            let err = Rc::new(err);
            plugins
                .functional_actions
                .export_actions
                .after_failed_static_alias_file_copy
                .run(err.clone(), plugins.get_plugin_data());
            return Err(err);
        }
    }

    Ok(())
}

/// Copies the directory containing static data to be put in `/.perseus/static/` (URL). This takes in both the location of the static directory and the destination
/// directory for exported files.
fn copy_static_dir(plugins: &Plugins<SsrNode>, static_dir_raw: &str, dest: &str) -> Result<(), Rc<EngineError>> {
    // Copy the `static` directory into the export package if it exists
    // If the user wants extra, they can use static aliases, plugins are unnecessary here
    let static_dir = PathBuf::from(static_dir_raw);
    if static_dir.exists() {
        if let Err(err) = copy_dir(&static_dir, format!("{}/.perseus/", dest), &CopyOptions::new()) {
            let err = EngineError::CopyStaticDirError{ source: err, path: static_dir_raw.to_string(), dest: dest.to_string() };
            let err = Rc::new(err);
            plugins
                .functional_actions
                .export_actions
                .after_failed_static_copy
                .run(err.clone(), plugins.get_plugin_data());
            return Err(err);
        }
    }

    Ok(())
}
