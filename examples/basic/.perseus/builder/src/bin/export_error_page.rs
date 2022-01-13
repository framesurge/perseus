use fmterr::fmt_err;
use perseus::{
    internal::{
        get_path_prefix_server,
        serve::{build_error_page, get_render_cfg, HtmlShell},
    },
    PluginAction, SsrNode,
};
use perseus_engine::app::{get_app_root, get_error_pages, get_immutable_store, get_plugins};
use std::{env, fs};

#[tokio::main]
async fn main() {
    let exit_code = real_main().await;
    std::process::exit(exit_code)
}

async fn real_main() -> i32 {
    // We want to be working in the root of `.perseus/`
    env::set_current_dir("../").unwrap();

    let plugins = get_plugins::<SsrNode>();
    let error_pages = get_error_pages(&plugins);
    let root_id = get_app_root(&plugins);
    let immutable_store = get_immutable_store(&plugins);
    let render_cfg = match get_render_cfg(&immutable_store).await {
        Ok(render_cfg) => render_cfg,
        Err(err) => {
            eprintln!("{}", fmt_err(&err));
            return 1;
        }
    };
    // Prepare the HTML shell
    let html = match fs::read_to_string("../index.html") {
        Ok(html) => html,
        Err(err) => {
            eprintln!("{}", fmt_err(&err));
            return 1;
        }
    };
    let html_shell = HtmlShell::new(html, &render_cfg, &get_path_prefix_server());
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
    // Build that error page as the server does
    let err_page_str = build_error_page(
        "",
        &err_code_to_build_for,
        "",
        None,
        &error_pages,
        &html_shell,
        &root_id,
    );

    // Write that to the mandatory second argument (the output location)
    // We'll move out of `.perseus/` first though
    env::set_current_dir("../").unwrap();
    match fs::write(output, err_page_str) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("{}", fmt_err(&err));
            return 1;
        }
    };

    plugins
        .functional_actions
        .export_actions
        .after_successful_export
        .run((), plugins.get_plugin_data());
    println!("Static exporting successfully completed!");
    0
}
