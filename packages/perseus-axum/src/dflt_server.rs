use crate::get_router;
use futures::executor::block_on;
use perseus::{
    builder::{get_host_and_port, get_props, get_standalone_and_act},
    internal::i18n::TranslationsManager,
    stores::MutableStore,
    PerseusAppBase, SsrNode,
};
use std::net::SocketAddr;

/// Creates and starts the default Perseus server with Axum. This should be run in a `main` function annotated with `#[tokio::main]` (which requires the `macros` and
/// `rt-multi-thread` features on the `tokio` dependency).
pub async fn dflt_server<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    app: impl Fn() -> PerseusAppBase<SsrNode, M, T> + 'static + Send + Sync + Clone,
) {
    get_standalone_and_act();
    let props = get_props(app());
    let (host, port) = get_host_and_port();
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address provided to bind to.");
    let app = block_on(get_router(props));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
