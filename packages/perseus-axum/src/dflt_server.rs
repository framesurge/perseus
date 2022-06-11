use perseus::{PerseusAppBase, SsrNode, internal::i18n::TranslationsManager, stores::MutableStore, builder::{get_standalone_and_act, get_host_and_port, get_props}};
use crate::get_router;
use std::net::SocketAddr;
use futures::executor::block_on;

/// Creates and starts the default Perseus server with Axum. This should be run in a `main` function annotated with `#[tokio::main]` (which requires the `macros` and
/// `rt-multi-thread` features on the `tokio` dependency).
pub async fn dflt_server(app: PerseusAppBase<SsrNode, impl MutableStore + 'static, impl TranslationsManager + 'static>) {
    get_standalone_and_act();
    let props = get_props(app);
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
