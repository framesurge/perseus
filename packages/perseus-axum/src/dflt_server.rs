use crate::get_router;
use perseus::{
    i18n::TranslationsManager, server::ServerOptions, stores::MutableStore, PerseusAppBase, SsrNode,
    turbine::Turbine,
};
use std::net::SocketAddr;

/// Creates and starts the default Perseus server with Axum. This should be run
/// in a `main` function annotated with `#[tokio::main]` (which requires the
/// `macros` and `rt-multi-thread` features on the `tokio` dependency).
pub async fn dflt_server<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    turbine: Turbine<M, T>,
    opts: ServerOptions,
    (host, port): (String, u16),
) {
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address provided to bind to.");
    let app = get_router(turbine, opts).await;
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
