use crate::perseus_routes;
use perseus::{
    internal::{i18n::TranslationsManager, serve::ServerProps},
    stores::MutableStore,
};
use std::net::SocketAddr;

/// Creates and starts the default Perseus server with Warp. This should be run in a `main` function annotated with `#[tokio::main]` (which requires the `macros` and
/// `rt-multi-thread` features on the `tokio` dependency).
pub async fn dflt_server<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    props: ServerProps<M, T>,
    (host, port): (String, u16),
) {
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address provided to bind to.");
    let routes = perseus_routes(props).await;
    warp::serve(routes).run(addr).await;
}
