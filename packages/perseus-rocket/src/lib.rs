#![doc = include_str!("../README.proj.md")]

/*!
## Packages

This is the API documentation for the `perseus-warp` package, which allows Perseus apps to run on Warp. Note that Perseus mostly uses [the book](https://framesurge.sh/perseus/en-US) for
documentation, and this should mostly be used as a secondary reference source. You can also find full usage examples [here](https://github.com/framesurge/perseus/tree/main/examples).
*/

#![cfg(engine)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

use std::{io::Cursor, path::Path, sync::Arc};

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
            // Headers that contain non-visible ascii characters are chopped off here in order to make the conversion
            if let Ok(value) = h.1.to_str() {
                resp_build.raw_header(h.0.to_string(), value.to_string());
            }
        }

        resp_build.ok()
    }
}

// ----- Simple routes -----

#[get("/.perseus/bundle.js")]
async fn get_js_bundle(opts: &State<ServerOptions>) -> std::io::Result<NamedFile> {
    NamedFile::open(opts.js_bundle.clone()).await
}

#[get("/.perseus/bundle.wasm")]
async fn get_wasm_bundle(opts: &State<ServerOptions>) -> std::io::Result<NamedFile> {
    NamedFile::open(opts.wasm_bundle.clone()).await
}

#[get("/.perseus/bundle.wasm.js")]
async fn get_wasm_js_bundle(opts: &State<ServerOptions>) -> std::io::Result<NamedFile> {
    NamedFile::open(opts.wasm_js_bundle.clone()).await
}

// ----- Turbine dependant route handlers -----

async fn perseus_locale<'r, M, T>(req: &'r Request<'_>, turbine: Arc<&Turbine<M, T>>) -> Outcome<'r>
where
    M: MutableStore + 'static,
    T: TranslationsManager + 'static,
{
    match req.routed_segment(0) {
        Some(locale) => Outcome::from(req, ApiResponse(turbine.get_translations(locale).await)),
        _ => Outcome::Failure(Status::BadRequest),
    }
}

async fn perseus_initial_load_handler<'r, M, T>(
    req: &'r Request<'_>,
    turbine: Arc<&Turbine<M, T>>,
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
    turbine: Arc<&Turbine<M, T>>,
) -> Outcome<'r>
where
    M: MutableStore + 'static,
    T: TranslationsManager + 'static,
{
    let locale_opt = req.routed_segment(0);
    let entity_name_opt = req
        .query_value::<&str>("entity_name")
        .and_then(|res| res.ok());
    let was_incremental_match_opt = req
        .query_value::<bool>("was_incremental_match")
        .and_then(|res| res.ok());

    if entity_name_opt.is_none() || locale_opt.is_none() || was_incremental_match_opt.is_none() {
        return Outcome::Failure(Status::BadRequest);
    }

    // All unwraps are guarded by the condition above
    let entity_name = entity_name_opt.unwrap().to_string();
    let locale = locale_opt.unwrap().to_string();
    let was_incremental_match = was_incremental_match_opt.unwrap();

    let raw_path = req.routed_segments(1..).collect::<Vec<&str>>().join("/");

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

// ----- Rocket Hanlder trait implementation -----

#[derive(Clone)]
enum PerseusRouteKind<'a> {
    Locale,
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
    turbine: Arc<&'a Turbine<M, T>>,
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
            PerseusRouteKind::Locale => perseus_locale(req, self.turbine.clone()).await,
            PerseusRouteKind::StaticAlias(static_alias) => {
                perseus_static_alias(req, static_alias).await
            }
            PerseusRouteKind::IntialLoadHandler => {
                perseus_initial_load_handler(req, self.turbine.clone()).await
            }
            PerseusRouteKind::SubsequentLoadHandler => {
                perseus_subsequent_load_handler(req, self.turbine.clone()).await
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
/// This returns a rocket at the build stage that can be built upon further with more routes, fairings etc...
pub async fn perseus_base_app<M, T>(
    turbine: &'static Turbine<M, T>,
    opts: ServerOptions,
) -> Rocket<Build>
where
    M: MutableStore + 'static,
    T: TranslationsManager + 'static,
{
    let arc_turbine = Arc::new(turbine);

    let get_locale = Route::new(
        Method::Get,
        "/.perseus/translations/<path..>",
        RocketHandlerWithTurbine {
            turbine: arc_turbine.clone(),
            perseus_route: PerseusRouteKind::Locale,
        },
    );

    // Since this route matches everything, its rank has been set to 0,
    // That means that it will be used after routes that have a rank inferior to 0 forward
    // see https://rocket.rs/v0.5-rc/guide/requests/#default-ranking
    let get_initial_load_handler = Route::ranked(
        0,
        Method::Get,
        "/<path..>",
        RocketHandlerWithTurbine {
            turbine: arc_turbine.clone(),
            perseus_route: PerseusRouteKind::IntialLoadHandler,
        },
    );

    let get_subsequent_load_handler = Route::new(
        Method::Get,
        "/.perseus/page/<path..>",
        RocketHandlerWithTurbine {
            turbine: arc_turbine.clone(),
            perseus_route: PerseusRouteKind::SubsequentLoadHandler,
        },
    );

    let mut app = rocket::build()
        .manage(opts.clone())
        .mount(
            "/",
            routes![get_js_bundle, get_wasm_bundle, get_wasm_js_bundle],
        )
        .mount(
            "/",
            vec![
                get_locale,
                get_subsequent_load_handler,
                get_initial_load_handler,
            ],
        );

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
                turbine: arc_turbine.clone(),
                perseus_route: PerseusRouteKind::StaticAlias(static_path),
            },
        );
        static_aliases.push(route)
    }

    app = app.mount("/", static_aliases);

    app
}

// ----- Default server -----

/// Creates and starts the default Perseus server with Rocket. This should be run
/// in a `main` function annotated with `#[tokio::main]` (which requires the
/// `macros` and `rt-multi-thread` features on the `tokio` dependency).
#[cfg(feature = "dflt-server")]
pub async fn dflt_server<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    turbine: &'static Turbine<M, T>,
    opts: ServerOptions,
    (_host, port): (String, u16),
) {
    let mut app = perseus_base_app(turbine, opts).await;

    let mut config = rocket::Config::default();
    config.port = port;
    app = app.configure(config);

    match app.launch().await {
        Err(e) => println!("Error lauching rocket app: {}", e),
        _ => (),
    }
}
