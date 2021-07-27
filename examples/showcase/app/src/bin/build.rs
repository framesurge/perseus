use perseus_showcase_app::{
    pages,
    config_manager::{FsConfigManager, ConfigManager},
    build_pages
};

fn main() {
    let config_manager = FsConfigManager::new();

    build_pages!([
        pages::index::get_page(),
        pages::about::get_page(),
        pages::post::get_page()
    ], &config_manager);

    println!("Static generation successfully completed!");
}
