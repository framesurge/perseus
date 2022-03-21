use fmterr::fmt_err;
use perseus::{
    internal::build::{build_app, BuildProps},
    PluginAction, SsrNode,
};
use perseus_engine as app;

#[tokio::main]
async fn main() {
    let exit_code = real_main().await;
    std::process::exit(exit_code)
}

async fn real_main() -> i32 {
    // We want to be working in the root of `.perseus/`
    std::env::set_current_dir("../").unwrap();
    let app = app::main::<SsrNode>();
    let plugins = app.get_plugins();

    plugins
        .functional_actions
        .build_actions
        .before_build
        .run((), plugins.get_plugin_data());

    let immutable_store = app.get_immutable_store();
    let mutable_store = app.get_mutable_store();
    let locales = app.get_locales();
    // Generate the global state
    let gsc = app.get_global_state_creator();
    let global_state = match gsc.get_build_state().await {
        Ok(global_state) => global_state,
        Err(err) => {
            let err_msg = fmt_err(&err);
            plugins
                .functional_actions
                .build_actions
                .after_failed_global_state_creation
                .run(err, plugins.get_plugin_data());
            eprintln!("{}", err_msg);
            return 1;
        }
    };

    // Build the site for all the common locales (done in parallel)
    // All these parameters can be modified by `define_app!` and plugins, so there's no point in having a plugin opportunity here
    let templates_map = app.get_templates_map();

    // We have to get the translations manager last, because it consumes everything
    let translations_manager = app.get_translations_manager().await;

    let res = build_app(BuildProps {
        templates: &templates_map,
        locales: &locales,
        immutable_store: &immutable_store,
        mutable_store: &mutable_store,
        translations_manager: &translations_manager,
        global_state: &global_state,
        exporting: false,
    })
    .await;
    if let Err(err) = res {
        let err_msg = fmt_err(&err);
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
