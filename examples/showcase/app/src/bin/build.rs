use futures::executor::block_on;
use perseus::{build_templates_for_locale, FsConfigManager, SsrNode, Translator};
use perseus_showcase_app::pages;

fn main() {
    let config_manager = FsConfigManager::new("./dist".to_string());

    let fut = build_templates_for_locale(
        vec![
            pages::index::get_page::<SsrNode>(),
            pages::about::get_page::<SsrNode>(),
            pages::post::get_page::<SsrNode>(),
            pages::new_post::get_page::<SsrNode>(),
            pages::ip::get_page::<SsrNode>(),
            pages::time::get_page::<SsrNode>(),
            pages::time_root::get_page::<SsrNode>(),
            pages::amalgamation::get_page::<SsrNode>(),
        ],
        Translator::empty(),
        &config_manager,
    );
    block_on(fut).expect("Static generation failed!");

    println!("Static generation successfully completed!");
}
