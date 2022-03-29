use fmterr::fmt_err;
use perseus::{internal::serve::build_error_page, PerseusApp, PluginAction, SsrNode};
use perseus_engine as app;
use std::{env, fs};

#[tokio::main]
async fn main() {
    let exit_code = real_main().await;
    std::process::exit(exit_code)
}

async fn real_main() -> i32 {
    // We want to be working in the root of `.perseus/`
    env::set_current_dir("../").unwrap();
    let app = app::main::<SsrNode>();

    let plugins = app.get_plugins();

    let error_pages = app.get_error_pages();
    // Prepare the HTML shell
    let index_view_str = app.get_index_view_str();
    let root_id = app.get_root();
    let immutable_store = app.get_immutable_store();
    // We assume the app has already been built before running this (so the render config must be available)
    // It doesn't matter if the type parameters here are wrong, this function doesn't use them
    let html_shell =
        PerseusApp::get_html_shell(index_view_str, &root_id, &immutable_store, &plugins).await;
    // Get the error code to build from the arguments to this executable
    let args = env::args().collect::<Vec<String>>();
    let err_code_to_build_for = match args.get(1) {
        Some(arg) => match arg.parse::<u16>() {
            Ok(err_code) => err_code,
            Err(_) => {
                eprintln!("You must provide a valid number as an HTTP error code.");
                return 1;
            }
        },
        None => {
            eprintln!("You must provide an HTTP error code to export an error page for.");
            return 1;
        }
    };
    // Get the output to write to from the second argument
    let output = match args.get(2) {
        Some(output) => output,
        None => {
            eprintln!("You must provide an output location for the exported error page.");
            return 1;
        }
    };
    plugins
        .functional_actions
        .export_error_page_actions
        .before_export_error_page
        .run(
            (err_code_to_build_for, output.to_string()),
            plugins.get_plugin_data(),
        );

    // Build that error page as the server does
    let err_page_str = build_error_page(
        "",
        err_code_to_build_for,
        "",
        None,
        &error_pages,
        &html_shell,
    );

    // Write that to the mandatory second argument (the output location)
    // We'll move out of `.perseus/` first though
    env::set_current_dir("../").unwrap();
    match fs::write(&output, err_page_str) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("{}", fmt_err(&err));
            plugins
                .functional_actions
                .export_error_page_actions
                .after_failed_write
                .run((err, output.to_string()), plugins.get_plugin_data());
            return 1;
        }
    };

    plugins
        .functional_actions
        .export_error_page_actions
        .after_successful_export_error_page
        .run((), plugins.get_plugin_data());
    println!("Static exporting successfully completed!");
    0
}
