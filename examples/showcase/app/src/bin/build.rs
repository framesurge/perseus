use perseus::{
    config_manager::FsConfigManager,
    build::build_templates
};
use perseus_showcase_app::pages;
use sycamore::prelude::SsrNode;
use futures::executor::block_on;

fn main() {
    let config_manager = FsConfigManager::new();

    let fut = build_templates(vec![
        pages::index::get_page::<SsrNode>(),
        pages::about::get_page::<SsrNode>(),
        pages::post::get_page::<SsrNode>(),
        pages::new_post::get_page::<SsrNode>(),
        pages::ip::get_page::<SsrNode>(),
        pages::time::get_page::<SsrNode>(),
        pages::time_root::get_page::<SsrNode>()
    ], &config_manager);
    block_on(fut).expect("Static generation failed!");

    println!("Static generation successfully completed!");
}
