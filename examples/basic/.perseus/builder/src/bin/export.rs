use fs_extra::dir::{copy as copy_dir, CopyOptions};
use futures::executor::block_on;
use perseus::{
    internal::{build::build_app, export::export_app, get_path_prefix_server},
    PluginAction, SsrNode,
};
use perseus_engine::app::{
    get_app_root, get_immutable_store, get_locales, get_mutable_store, get_plugins,
    get_static_aliases, get_templates_map, get_translations_manager,
};
use std::fs;
use std::path::PathBuf;

fn main() {
    let exit_code = real_main();
    std::process::exit(exit_code)
}

fn real_main() -> i32 {
    // We want to be working in the root of `.perseus/`
    std::env::set_current_dir("../").unwrap();

    let plugins = get_plugins::<SsrNode>();

    plugins
        .functional_actions
        .build_actions
        .before_build
        .run((), plugins.get_plugin_data());

    let immutable_store = get_immutable_store(&plugins);
    // We don't need this in exporting, but the build process does
    let mutable_store = get_mutable_store();
    let translations_manager = block_on(get_translations_manager());
    let locales = get_locales(&plugins);

    // Build the site for all the common locales (done in parallel), denying any non-exportable features
    let templates_map = get_templates_map::<SsrNode>(&plugins);
    let build_fut = build_app(
        &templates_map,
        &locales,
        (&immutable_store, &mutable_store),
        &translations_manager,
        // We use another binary to handle normal building
        true,
    );
    if let Err(err) = block_on(build_fut) {
        let err_msg = format!("Static exporting failed: '{}'.", &err);
        plugins
            .functional_actions
            .export_actions
            .after_failed_build
            .run(err, plugins.get_plugin_data());
        eprintln!("{}", err_msg);
        return 1;
    }
    plugins
        .functional_actions
        .export_actions
        .after_successful_build
        .run((), plugins.get_plugin_data());
    // Turn the build artifacts into self-contained static files
    let app_root = get_app_root(&plugins);
    let export_fut = export_app(
        &templates_map,
        // Perseus always uses one HTML file, and there's no point in letting a plugin change that
        "../index.html",
        &locales,
        &app_root,
        &immutable_store,
        &translations_manager,
        get_path_prefix_server(),
    );
    if let Err(err) = block_on(export_fut) {
        let err_msg = format!("Static exporting failed: '{}'.", &err);
        plugins
            .functional_actions
            .export_actions
            .after_failed_export
            .run(err, plugins.get_plugin_data());
        eprintln!("{}", err_msg);
        return 1;
    }

    // Copy the `static` directory into the export package if it exists
    // If the user wants extra, they can use static aliases, plugins are unnecessary here
    let static_dir = PathBuf::from("../static");
    if static_dir.exists() {
        if let Err(err) = copy_dir(&static_dir, "dist/exported/.perseus/", &CopyOptions::new()) {
            let err_msg = format!(
                "Static exporting failed: 'couldn't copy static directory: '{}''",
                &err
            );
            plugins
                .functional_actions
                .export_actions
                .after_failed_static_copy
                .run(err.to_string(), plugins.get_plugin_data());
            eprintln!("{}", err_msg);
            return 1;
        }
    }
    // Loop through any static aliases and copy them in too
    // Unlike with the server, these could override pages!
    // We'll copy from the alias to the path (it could be a directory or a file)
    // Remember: `alias` has a leading `/`!
    for (alias, path) in get_static_aliases(&plugins) {
        let from = PathBuf::from(path);
        let to = format!("dist/exported{}", alias);

        if from.is_dir() {
            if let Err(err) = copy_dir(&from, &to, &CopyOptions::new()) {
                let err_msg = format!(
                    "Static exporting failed: 'couldn't copy static alias directory: '{}''",
                    err.to_string()
                );
                plugins
                    .functional_actions
                    .export_actions
                    .after_failed_static_alias_dir_copy
                    .run(err.to_string(), plugins.get_plugin_data());
                eprintln!("{}", err_msg);
                return 1;
            }
        } else if let Err(err) = fs::copy(&from, &to) {
            let err_msg = format!(
                "Static exporting failed: 'couldn't copy static alias file: '{}''",
                err.to_string()
            );
            plugins
                .functional_actions
                .export_actions
                .after_failed_static_alias_file_copy
                .run(err, plugins.get_plugin_data());
            eprintln!("{}", err_msg);
            return 1;
        }
    }

    plugins
        .functional_actions
        .export_actions
        .after_successful_export
        .run((), plugins.get_plugin_data());
    println!("Static exporting successfully completed!");
    0
}
