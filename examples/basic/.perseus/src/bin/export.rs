use app::{
    get_immutable_store, get_locales, get_mutable_store, get_static_aliases, get_templates_map,
    get_templates_vec, get_translations_manager, APP_ROOT,
};
use fs_extra::dir::{copy as copy_dir, CopyOptions};
use futures::executor::block_on;
use perseus::{build_app, export_app, SsrNode};
use std::fs;
use std::path::PathBuf;

fn main() {
    let exit_code = real_main();
    std::process::exit(exit_code)
}

fn real_main() -> i32 {
    let immutable_store = get_immutable_store();
    // We don't need this in exporting, but the build process does
    let mutable_store = get_mutable_store();
    let translations_manager = block_on(get_translations_manager());
    let locales = get_locales();

    // Build the site for all the common locales (done in parallel), denying any non-exportable features
    let build_fut = build_app(
        get_templates_vec::<SsrNode>(),
        &locales,
        (&immutable_store, &mutable_store),
        &translations_manager,
        // We use another binary to handle normal building
        true,
    );
    if let Err(err) = block_on(build_fut) {
        eprintln!("Static exporting failed: '{}'.", err);
        return 1;
    }
    // Turn the build artifacts into self-contained static files
    let export_fut = export_app(
        get_templates_map(),
        "../index.html",
        &locales,
        APP_ROOT,
        &immutable_store,
        &translations_manager,
    );
    if let Err(err) = block_on(export_fut) {
        eprintln!("Static exporting failed: '{}'.", err);
        return 1;
    }

    // Copy the `static` directory into the export package if it exists
    // We don't use a config manager here because static files are always handled on-disk in Perseus (for now)
    let static_dir = PathBuf::from("../static");
    if static_dir.exists() {
        if let Err(err) = copy_dir(&static_dir, "dist/exported/.perseus/", &CopyOptions::new()) {
            eprintln!(
                "Static exporting failed: 'couldn't copy static directory: '{}''",
                err.to_string()
            );
            return 1;
        }
    }
    // Loop through any static aliases and copy them in too
    // Unlike with the server, these could override pages!
    // We'll copy from the alias to the path (it could be a directory or a file)
    // Remember: `alias` has a leading `/`!
    for (alias, path) in get_static_aliases() {
        let from = PathBuf::from(path);
        let to = format!("dist/exported{}", alias);

        if from.is_dir() {
            if let Err(err) = copy_dir(&from, &to, &CopyOptions::new()) {
                eprintln!(
                    "Static exporting failed: 'couldn't copy static alias directory: '{}''",
                    err.to_string()
                );
                return 1;
            }
        } else if let Err(err) = fs::copy(&from, &to) {
            eprintln!(
                "Static exporting failed: 'couldn't copy static alias file: '{}''",
                err.to_string()
            );
            return 1;
        }
    }

    println!("Static exporting successfully completed!");
    0
}
