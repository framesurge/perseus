use app::{get_config_manager, get_locales, get_templates_vec, get_templates_map, get_translations_manager};
use futures::executor::block_on;
use perseus::{build_app, export_app, SsrNode};

fn main() {
    let exit_code = real_main();
    std::process::exit(exit_code)
}

fn real_main() -> i32 {
    let config_manager = get_config_manager();
    let translations_manager = block_on(get_translations_manager());
    let locales = get_locales();

    // Build the site for all the common locales (done in parallel), denying any non-exportable features
    let build_fut = build_app(
        get_templates_vec::<SsrNode>(),
        &locales,
        &config_manager,
        &translations_manager,
        // We use another binary to handle normal building
        true
    );
    if let Err(err) = block_on(build_fut) {
        eprintln!("Static exporting failed: '{}'.", err);
        return 1
    }
    // Turn the build artifacts into self-contained static files
    let export_fut = export_app(
        get_templates_map(),
        "../index.html",
        &locales,
        &config_manager
    );
    if let Err(err) = block_on(export_fut) {
        eprintln!("Static exporting failed: '{}'.", err);
        1
    } else {
        println!("Static exporting successfully completed!");
        0
    }
}
