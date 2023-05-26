#![doc = include_str!("../README.proj.md")]

/*!
## Packages

This is the API documentation for the `perseus-rocket` package, which allows Perseus apps to run on Rocket. Note that Perseus mostly uses [the book](https://framesurge.sh/perseus/en-US) for
documentation, and this should mostly be used as a secondary reference source. You can also find full usage examples [here](https://github.com/framesurge/perseus/tree/main/examples).
*/

#![cfg(engine)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

use std::{io::Cursor, path::Path};

use perseus::{
    i18n::TranslationsManager,
    path::PathMaybeWithLocale,
    server::ServerOptions,
    stores::MutableStore,
    turbine::{ApiResponse as PerseusApiResponse, Turbine},
};
use rocket::{
    fs::{FileServer, NamedFile},
    get,
    http::{Method, Status},
    response::Responder,
    route::{Handler, Outcome},
    routes,
    tokio::fs::File,
    Build, Data, Request, Response, Rocket, Route, State,
};

// ----- Newtype wrapper for response implementation -----

#[derive(Debug)]
struct ApiResponse(PerseusApiResponse);
impl From<PerseusApiResponse> for ApiResponse {
    fn from(val: PerseusApiResponse) -> Self {
        Self(val)
    }
}
impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut resp_build = Response::build();
        resp_build
            .status(rocket::http::Status {
                code: self.0.status.into(),
            })
            .sized_body(self.0.body.len(), Cursor::new(self.0.body));

        for h in self.0.headers.iter() {
            // Headers that contain non-visible ascii characters are chopped off here in
            // order to make the conversion
            if let Ok(value) = h.1.to_str() {
                resp_build.raw_header(h.0.to_string(), value.to_string());
            }
        }

        resp_build.ok()
    }
}

// ----- Simple routes -----

#[get("/bundle.js")]
async fn get_js_bundle(opts: &State<ServerOptions>) -> std::io::Result<NamedFile> {
    NamedFile::open(&opts.js_bundle).await
}

#[get("/bundle.wasm")]
async fn get_wasm_bundle(opts: &State<ServerOptions>) -> std::io::Result<NamedFile> {
    NamedFile::open(&opts.wasm_bundle).await
}

#[get("/bundle.wasm.js")]
async fn get_wasm_js_bundle(opts: &State<ServerOptions>) -> std::io::Result<NamedFile> {
    NamedFile::open(&opts.wasm_js_bundle).await
}

// ----- Turbine dependant route handlers -----

async fn perseus_locale<'r, M, T>(req: &'r Request<'_>, turbine: &Turbine<M, T>) -> Outcome<'r>
where
    M: MutableStore + 'static,
    T: TranslationsManager + 'static,
{
    match req.routed_segment(1) {
        Some(locale) => Outcome::from(req, ApiResponse(turbine.get_translations(locale).await)),
        _ => Outcome::Failure(Status::BadRequest),
    }
}

async fn perseus_localized_initial_consts<'r, M, T>(
    req: &'r Request<'_>,
    turbine: &Turbine<M, T>,
) -> Outcome<'r>
where
    M: MutableStore + 'static,
    T: TranslationsManager + 'static,
{
    match req.routed_segment(1) {
        Some(locale) => {
            let locale = match locale.strip_suffix(".js") {
                Some(locale) => locale,
                None => return Outcome::Failure(Status::BadRequest),
            };
            Outcome::from(req, ApiResponse(turbine.get_initial_consts(locale).await))
        }
        _ => Outcome::Failure(Status::BadRequest),
    }
}

async fn perseus_unlocalized_initial_consts<'r, M, T>(
    req: &'r Request<'_>,
    turbine: &Turbine<M, T>,
) -> Outcome<'r>
where
    M: MutableStore + 'static,
    T: TranslationsManager + 'static,
{
    Outcome::from(req, ApiResponse(turbine.get_initial_consts("").await))
}

async fn perseus_initial_load_handler<'r, M, T>(
    req: &'r Request<'_>,
    turbine: &Turbine<M, T>,
) -> Outcome<'r>
where
    M: MutableStore + 'static,
    T: TranslationsManager + 'static,
{
    // Since this is a fallback handler, we have to do everything from the request
    // itself
    let path = req.uri().path().to_string();

    let mut http_req = rocket::http::hyper::Request::builder();
    http_req = http_req.method("GET");
    for h in req.headers().iter() {
        http_req = http_req.header(h.name.to_string(), h.value.to_string());
    }

    match http_req.body(()) {
        Ok(r) => Outcome::from(
            req,
            ApiResponse(turbine.get_initial_load(PathMaybeWithLocale(path), r).await),
        ),
        _ => Outcome::Failure(Status::BadRequest),
    }
}

async fn perseus_subsequent_load_handler<'r, M, T>(
    req: &'r Request<'_>,
    turbine: &Turbine<M, T>,
) -> Outcome<'r>
where
    M: MutableStore + 'static,
    T: TranslationsManager + 'static,
{
    let locale_opt = req.routed_segment(1);
    let entity_name_opt = req
        .query_value::<&str>("entity_name")
        .and_then(|res| res.ok());
    let was_incremental_match_opt = req
        .query_value::<bool>("was_incremental_match")
        .and_then(|res| res.ok());

    let (locale, entity_name, was_incremental_match) =
        match (locale_opt, entity_name_opt, was_incremental_match_opt) {
            (Some(l), Some(e), Some(w)) => (l.to_string(), e.to_string(), w),
            _ => return Outcome::Failure(Status::BadRequest),
        };

    let raw_path = req.routed_segments(2..).collect::<Vec<&str>>().join("/");

    let mut http_req = rocket::http::hyper::Request::builder();
    http_req = http_req.method("GET");
    for h in req.headers().iter() {
        http_req = http_req.header(h.name.to_string(), h.value.to_string());
    }

    match http_req.body(()) {
        Ok(r) => Outcome::from(
            req,
            ApiResponse(
                turbine
                    .get_subsequent_load(
                        perseus::path::PathWithoutLocale(raw_path),
                        locale,
                        entity_name,
                        was_incremental_match,
                        r,
                    )
                    .await,
            ),
        ),
        _ => Outcome::Failure(Status::BadRequest),
    }
}

// ----- Rocket handler trait implementation -----

#[derive(Clone)]
enum PerseusRouteKind<'a> {
    Locale,
    LocalizedInitialConsts,
    UnlocalizedInitialConsts,
    StaticAlias(&'a String),
    IntialLoadHandler,
    SubsequentLoadHandler,
}

#[derive(Clone)]
struct RocketHandlerWithTurbine<'a, M, T>
where
    M: MutableStore + 'static,
    T: TranslationsManager + 'static,
{
    turbine: &'a Turbine<M, T>,
    perseus_route: PerseusRouteKind<'a>,
}

#[rocket::async_trait]
impl<M, T> Handler for RocketHandlerWithTurbine<'static, M, T>
where
    M: MutableStore + 'static,
    T: TranslationsManager + 'static,
{
    async fn handle<'r>(&self, req: &'r Request<'_>, _data: Data<'r>) -> Outcome<'r> {
        match self.perseus_route {
            PerseusRouteKind::Locale => perseus_locale(req, self.turbine).await,
            PerseusRouteKind::LocalizedInitialConsts => {
                perseus_localized_initial_consts(req, self.turbine).await
            }
            PerseusRouteKind::UnlocalizedInitialConsts => {
                perseus_unlocalized_initial_consts(req, self.turbine).await
            }
            PerseusRouteKind::StaticAlias(static_alias) => {
                perseus_static_alias(req, static_alias).await
            }
            PerseusRouteKind::IntialLoadHandler => {
                perseus_initial_load_handler(req, self.turbine).await
            }
            PerseusRouteKind::SubsequentLoadHandler => {
                perseus_subsequent_load_handler(req, self.turbine).await
            }
        }
    }
}

async fn perseus_static_alias<'r>(req: &'r Request<'_>, static_alias: &String) -> Outcome<'r> {
    match File::open(static_alias).await {
        Ok(file) => Outcome::from(req, file),
        _ => Outcome::Failure(Status::NotFound),
    }
}

// ----- Integration code -----

/// Configures an Rocket Web app for Perseus.
/// This returns a rocket app at the build stage that can be built upon further
/// with more routes, fairings etc...
pub async fn perseus_base_app<M, T>(
    turbine: &'static Turbine<M, T>,
    opts: ServerOptions,
) -> Rocket<Build>
where
    M: MutableStore + 'static,
    T: TranslationsManager + 'static,
{
    let get_locale = Route::new(
        Method::Get,
        "/translations/<path..>",
        RocketHandlerWithTurbine {
            turbine,
            perseus_route: PerseusRouteKind::Locale,
        },
    );
    let get_localized_initial_consts = Route::new(
        Method::Get,
        "/initial_consts/<path>", // Rocket doesn't seem to let us add `.js` here
        RocketHandlerWithTurbine {
            turbine,
            perseus_route: PerseusRouteKind::LocalizedInitialConsts,
        },
    );
    let get_unlocalized_initial_consts = Route::new(
        Method::Get,
        "/initial_consts.js",
        RocketHandlerWithTurbine {
            turbine,
            perseus_route: PerseusRouteKind::UnlocalizedInitialConsts,
        },
    );

    // Since this route matches everything, its rank has been set to 100,
    // That means that it will be used after routes that have a rank inferior to 100
    // forward, see https://rocket.rs/v0.5-rc/guide/requests/#default-ranking
    let get_initial_load_handler = Route::ranked(
        100,
        Method::Get,
        "/<path..>",
        RocketHandlerWithTurbine {
            turbine,
            perseus_route: PerseusRouteKind::IntialLoadHandler,
        },
    );

    let get_subsequent_load_handler = Route::new(
        Method::Get,
        "/page/<path..>",
        RocketHandlerWithTurbine {
            turbine,
            perseus_route: PerseusRouteKind::SubsequentLoadHandler,
        },
    );

    let mut perseus_routes: Vec<Route> =
        routes![get_js_bundle, get_wasm_js_bundle, get_wasm_bundle];
    perseus_routes.append(&mut vec![
        get_locale,
        get_subsequent_load_handler,
        get_unlocalized_initial_consts,
        get_localized_initial_consts,
    ]);

    let mut app = rocket::build()
        .manage(opts.clone())
        .mount("/.perseus/", perseus_routes)
        .mount("/", vec![get_initial_load_handler]);

    if Path::new(&opts.snippets).exists() {
        app = app.mount("/.perseus/snippets", FileServer::from(opts.snippets))
    }

    if turbine.static_dir.exists() {
        app = app.mount("/.perseus/static", FileServer::from(&turbine.static_dir))
    }

    let mut static_aliases: Vec<Route> = vec![];

    for (url, static_path) in turbine.static_aliases.iter() {
        let route = Route::new(
            Method::Get,
            url,
            RocketHandlerWithTurbine {
                turbine,
                perseus_route: PerseusRouteKind::StaticAlias(static_path),
            },
        );
        static_aliases.push(route)
    }

    app = app.mount("/", static_aliases);

    app
}

// ----- Default server -----

/// Creates and starts the default Perseus server with Rocket. This should be
/// run in a `main` function annotated with `#[tokio::main]` (which requires the
/// `macros` and `rt-multi-thread` features on the `tokio` dependency).
#[cfg(feature = "dflt-server")]
pub async fn dflt_server<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    turbine: &'static Turbine<M, T>,
    opts: ServerOptions,
    (host, port): (String, u16),
) {
    let addr = host.parse().expect("Invalid address provided to bind to.");

    let mut app = perseus_base_app(turbine, opts).await;

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
