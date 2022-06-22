use crate::perseus_routes;
use futures::executor::block_on;
use perseus::{
    builder::{get_host_and_port, get_props, get_standalone_and_act},
    internal::i18n::TranslationsManager,
    stores::MutableStore,
    PerseusAppBase, SsrNode,
};
use std::net::SocketAddr;

/// Creates and starts the default Perseus server with Warp. This should be run in a `main` function annotated with `#[tokio::main]` (which requires the `macros` and
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
    let routes = block_on(perseus_routes(props));
    warp::serve(routes).run(addr).await;
}
