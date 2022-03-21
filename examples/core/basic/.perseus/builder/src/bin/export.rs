use fmterr::fmt_err;
use fs_extra::dir::{copy as copy_dir, CopyOptions};
use perseus::{
    internal::{
        build::{build_app, BuildProps},
        export::{export_app, ExportProps},
        get_path_prefix_server,
    },
    PluginAction, SsrNode,
};
use perseus_engine as app;
use std::fs;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let exit_code = real_main().await;
    std::process::exit(exit_code)
}

async fn real_main() -> i32 {
    // We want to be working in the root of `.perseus/`
    std::env::set_current_dir("../").unwrap();
    let app = app::main::<SsrNode>();

    let plugins = app.get_plugins();

    // Building and exporting must be sequential, but that can be done in parallel with static directory/alias copying
    let exit_code = build_and_export().await;
    if exit_code != 0 {
        return exit_code;
    }
    // After that's done, we can do two copy operations in parallel at least
    let exit_code_1 = tokio::task::spawn_blocking(copy_static_dir);
    let exit_code_2 = tokio::task::spawn_blocking(copy_static_aliases);
    // These errors come from any panics in the threads, which should be propagated up to a panic in the main thread in this case
    exit_code_1.await.unwrap();
    exit_code_2.await.unwrap();

    plugins
        .functional_actions
        .export_actions
        .after_successful_export
        .run((), plugins.get_plugin_data());
    println!("Static exporting successfully completed!");
    0
}

async fn build_and_export() -> i32 {
    let app = app::main::<SsrNode>();
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
            let err_msg = fmt_err(&err);
            plugins
                .functional_actions
                .export_actions
                .after_failed_global_state_creation
                .run(err, plugins.get_plugin_data());
            eprintln!("{}", err_msg);
            return 1;
        }
    };
    let templates_map = app.get_templates_map();
    let index_view = app.get_index_view().await;
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
        let err_msg = fmt_err(&err);
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
        let err_msg = fmt_err(&err);
        plugins
            .functional_actions
            .export_actions
            .after_failed_export
            .run(err, plugins.get_plugin_data());
        eprintln!("{}", err_msg);
        return 1;
    }

    0
}

fn copy_static_dir() -> i32 {
    let app = app::main::<SsrNode>();
    let plugins = app.get_plugins();
    // Loop through any static aliases and copy them in too
    // Unlike with the server, these could override pages!
    // We'll copy from the alias to the path (it could be a directory or a file)
    // Remember: `alias` has a leading `/`!
    for (alias, path) in app.get_static_aliases() {
        let from = PathBuf::from(path);
        let to = format!("dist/exported{}", alias);

        if from.is_dir() {
            if let Err(err) = copy_dir(&from, &to, &CopyOptions::new()) {
                let err_msg = format!(
                    "couldn't copy static alias directory from '{}' to '{}': '{}'",
                    from.to_str().map(|s| s.to_string()).unwrap(),
                    to,
                    fmt_err(&err)
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
                "couldn't copy static alias file from '{}' to '{}': '{}'",
                from.to_str().map(|s| s.to_string()).unwrap(),
                to,
                fmt_err(&err)
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

    0
}

fn copy_static_aliases() -> i32 {
    let app = app::main::<SsrNode>();
    let plugins = app.get_plugins();
    // Copy the `static` directory into the export package if it exists
    // If the user wants extra, they can use static aliases, plugins are unnecessary here
    let static_dir = PathBuf::from("../static");
    if static_dir.exists() {
        if let Err(err) = copy_dir(&static_dir, "dist/exported/.perseus/", &CopyOptions::new()) {
            let err_msg = format!("couldn't copy static directory: '{}'", fmt_err(&err));
            plugins
                .functional_actions
                .export_actions
                .after_failed_static_copy
                .run(err.to_string(), plugins.get_plugin_data());
            eprintln!("{}", err_msg);
            return 1;
        }
    }

    0
}
