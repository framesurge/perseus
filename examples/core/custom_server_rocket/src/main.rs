mod templates;

use perseus::prelude::*;

#[cfg(engine)]
mod api {
    use rocket::get;

    #[get("/hello")]
    pub fn hello() -> String {
        "Hello from an api".to_string()
    }
}

#[cfg(engine)]
pub async fn dflt_server<
    M: perseus::stores::MutableStore + 'static,
    T: perseus::i18n::TranslationsManager + 'static,
>(
    turbine: &'static perseus::turbine::Turbine<M, T>,
    opts: perseus::server::ServerOptions,
    (host, port): (String, u16),
) {
    use perseus_rocket::perseus_base_app;
    use rocket::routes;

    let addr = host.parse().expect("Invalid address provided to bind to.");

    let mut app = perseus_base_app(turbine, opts).await;
    app = app.mount("/api", routes![api::hello]);

    let config = rocket::Config {
        port,
        address: addr,
        ..Default::default()
    };
    app = app.configure(config);

    if let Err(err) = app.launch().await {
        eprintln!("Error lauching Rocket app: {}.", err);
    }
}

#[perseus::main(dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .template(crate::templates::about::get_template())
        .error_views(ErrorViews::unlocalized_development_default())
}
