use crate::configurer;
use actix_web::{App, HttpServer};
use futures::executor::block_on;
use perseus::{
    internal::i18n::TranslationsManager, internal::serve::ServerProps, stores::MutableStore,
};

/// Creates and starts the default Perseus server using Actix Web. This should
/// be run in a `main()` function annotated with `#[tokio::main]` (which
/// requires the `macros` and `rt-multi-thread` features on the `tokio`
/// dependency).
pub async fn dflt_server<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    props: ServerProps<M, T>,
    (host, port): (String, u16),
) {
    // TODO Fix issues here
    HttpServer::new(move ||
        App::new()
            .configure(
                block_on(
                    configurer(
                        props.clone()
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
