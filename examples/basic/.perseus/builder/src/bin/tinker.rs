use perseus::{plugins::PluginAction, SsrNode};
use perseus_engine::app::get_plugins;

fn main() {
    let exit_code = real_main();
    std::process::exit(exit_code)
}

fn real_main() -> i32 {
    // We want to be working in the root of `.perseus/`
    std::env::set_current_dir("../").unwrap();

    let plugins = get_plugins::<SsrNode>();
    // Run all the tinker actions
    plugins
        .functional_actions
        .tinker
        .run((), plugins.get_plugin_data());

    println!("Tinkering complete!");
    0
}
