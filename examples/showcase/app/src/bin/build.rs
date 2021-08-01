use perseus::{
    config_manager::FsConfigManager,
    build::build_templates
};
use perseus_showcase_app::pages;
use sycamore::prelude::SsrNode;

fn main() {
    let config_manager = FsConfigManager::new();

    build_templates(vec![
        pages::index::get_page::<SsrNode>(),
        pages::about::get_page::<SsrNode>(),
        pages::post::get_page::<SsrNode>(),
        pages::new_post::get_page::<SsrNode>(),
        pages::ip::get_page::<SsrNode>()
    ], &config_manager).expect("Static generation failed!");

    println!("Static generation successfully completed!");
}
