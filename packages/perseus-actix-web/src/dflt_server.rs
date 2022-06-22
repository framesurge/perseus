use crate::configurer;
use actix_web::{App, HttpServer};
use futures::executor::block_on;
use perseus::{
    builder::{get_host_and_port, get_props, get_standalone_and_act},
    internal::i18n::TranslationsManager,
    stores::MutableStore,
    PerseusAppBase, SsrNode,
};

/// Creates and starts the default Perseus server using Actix Web. This should be run in a `main()` function annotated with `#[tokio::main]` (which requires the `macros` and
/// `rt-multi-thread` features on the `tokio` dependency).
pub async fn dflt_server<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    app: impl Fn() -> PerseusAppBase<SsrNode, M, T> + 'static + Send + Sync + Clone,
) {
    get_standalone_and_act();
    let (host, port) = get_host_and_port();

    HttpServer::new(move ||
        App::new()
            .configure(
                block_on(
                    configurer(
                        get_props(
                            app()
                        )
                    )
                )
            )
    )
        .bind((host, port))
        .expect("Couldn't bind to given address. Maybe something is already running on the selected port?")
        .run()
        .await
        .expect("Server failed.") // TODO Improve error message here
}
