use perseus::{plugins::PluginAction, SsrNode};
use perseus_engine as app;

fn main() {
    let exit_code = real_main();
    std::process::exit(exit_code)
}

fn real_main() -> i32 {
    // We want to be working in the root of `.perseus/`
    std::env::set_current_dir("../").unwrap();

    let plugins = app::main::<SsrNode>().get_plugins();
    // Run all the tinker actions
    // Note: this is deliberately synchronous, tinker actions that need a multithreaded async runtime should probably
    // be making their own engines!
    plugins
        .functional_actions
        .tinker
        .run((), plugins.get_plugin_data());

    println!("Tinkering complete!");
    0
}
