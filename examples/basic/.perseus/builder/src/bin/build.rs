use perseus::{internal::build::build_app, PluginAction, SsrNode};
use perseus_engine::app::{
    get_immutable_store, get_locales, get_mutable_store, get_plugins, get_templates_map,
    get_translations_manager,
};

#[tokio::main]
async fn main() {
    let exit_code = real_main().await;
    std::process::exit(exit_code)
}

async fn real_main() -> i32 {
    // We want to be working in the root of `.perseus/`
    std::env::set_current_dir("../").unwrap();
    let plugins = get_plugins::<SsrNode>();

    plugins
        .functional_actions
        .build_actions
        .before_build
        .run((), plugins.get_plugin_data());

    let immutable_store = get_immutable_store(&plugins);
    let mutable_store = get_mutable_store();
    // We can't proceed without a translations manager
    let translations_manager = get_translations_manager().await;
    let locales = get_locales(&plugins);

    // Build the site for all the common locales (done in parallel)
    // All these parameters can be modified by `define_app!` and plugins, so there's no point in having a plugin opportunity here
    let templates_map = get_templates_map::<SsrNode>(&plugins);
    let res = build_app(
        &templates_map,
        &locales,
        (&immutable_store, &mutable_store),
        &translations_manager,
        // We use another binary to handle exporting
        false,
    )
    .await;
    if let Err(err) = res {
        let err_msg = format!("Static generation failed: '{}'.", &err);
        plugins
            .functional_actions
            .build_actions
            .after_failed_build
            .run(err, plugins.get_plugin_data());
        eprintln!("{}", err_msg);
        1
    } else {
        plugins
            .functional_actions
            .build_actions
            .after_successful_build
            .run((), plugins.get_plugin_data());
        println!("Static generation successfully completed!");
        0
    }
}
