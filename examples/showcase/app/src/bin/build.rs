use perseus::{
    config_manager::{FsConfigManager, ConfigManager},
    build_templates
};
use perseus_showcase_app::pages;
use sycamore::prelude::SsrNode;

fn main() {
    let config_manager = FsConfigManager::new();

    build_templates!([
        pages::index::get_page::<SsrNode>(),
        pages::about::get_page::<SsrNode>(),
        pages::post::get_page::<SsrNode>()
    ], &config_manager);

    println!("Static generation successfully completed!");
}
