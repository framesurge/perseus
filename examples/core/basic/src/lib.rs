mod error_pages;
mod templates;

use perseus::{Html, PerseusApp};

// pub fn get_app<G: Html>() -> PerseusApp<G> {
//     PerseusApp::new()
//         .template(crate::templates::index::get_template)
//         .template(crate::templates::about::get_template)
//         .error_pages(crate::error_pages::get_error_pages)
// }

// #[perseus::engine_main]
// async fn main() {
//     use perseus::builder::{get_op, run_dflt_engine};

//     let op = get_op().unwrap();
//     let exit_code = run_dflt_engine(op, get_app, perseus_warp::dflt_server).await;
//     std::process::exit(exit_code);
// }

// #[perseus::browser_main]
// pub fn main() -> perseus::ClientReturn {
//     use perseus::run_client;

//     run_client(get_app)
// }

// #[perseus::main(perseus_warp::dflt_server)]
#[perseus::main_export]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template)
        .template(crate::templates::about::get_template)
        .error_pages(crate::error_pages::get_error_pages)
}
