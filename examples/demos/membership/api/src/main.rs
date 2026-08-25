use actix_cors::Cors;
use actix_web::cookie::Key;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use futures::future::LocalBoxFuture;
use serde::Serialize;
// use category::ErrorResponse;
use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use std::{
    error::Error,
    future::{self, Ready},
    net::Ipv4Addr,
};
use time::Duration;

pub mod auth;
pub mod category;
pub mod configs;
pub mod errors;
pub mod mail;
pub mod posts;
pub mod posts_tags;
pub mod tags;
use deadpool_postgres::{Pool, Runtime};
use dotenv::dotenv;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::configs::Config;

const API_KEY_NAME: &str = "todo_apikey";
const API_KEY: &str = "utoipa-rocks";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_server=debug,actix_web=debug");
    std::env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();

    #[derive(OpenApi)]
    #[openapi(
        info(title = "authentication middleware"),
        paths(
            auth::register_user,
            auth::process_login,
            category::category,
            category::add_category,
            category::update_category,
            category::get_category,
            category::delete_category,
            tags::tags,
            tags::add_tags,
            tags::update_tags,
            tags::get_tags,
            tags::delete_tags,
            posts::posts,
            posts::add_posts,
            posts::update_posts,
            posts::get_posts,
            posts::delete_posts,
        ),
        components(
            schemas(auth::CreateUser, errors::ServiceError, category::Category, category::CreateCategory, tags::Tags, tags::CreateTags, posts::Post, posts::CreatePost)
        ),
        tags(
            (name = "Auth", description = "Authentication Mechanism")
        )
    )]
    #[derive(Serialize, Clone, Debug)]
    struct ApiDoc;
    let openapi = ApiDoc::openapi();

    #[derive(Serialize, Debug)]
    struct ApiPath {
        api: ApiDoc,
    }

    let config = Config::from_env().unwrap();
    // let config = configs::Config::new();
    let bind_addr = format!("{}:{}", config.srv_cnf.host, config.srv_cnf.port);
    println!(
        "Starting server at http://{}:{}",
        config.srv_cnf.host, config.srv_cnf.port
    );

    let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_owned());
    let pool = config
        .pg
        .create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
        .unwrap();

    let server = HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    Key::from(config.srv_cnf.secret_key.as_bytes()),
                )
                .session_lifecycle(PersistentSession::default().session_ttl(Duration::days(1)))
                .cookie_name("session".to_owned())
                .cookie_secure(false)
                .cookie_domain(Some(domain.clone()))
                .cookie_path("/".to_owned())
                .build(),
            )
            // enable logger
            .wrap(middleware::Logger::default())
            // .wrap(middleware::Logger::new("%% \n|Origin: %a |Time: %t |Method: %r \r|Status: %s |Size: %b |ReqTime: %D \r|RemoteIP: %{r}a |Request URL: %U %{User-Agent}i"))
            .wrap(cors)
            // .service(web::scope("/categories").configure(category::init_routes))
            .service(web::scope("/auth").configure(auth::init_routes))
            .service(web::scope("/posts").configure(posts::init_routes))
            .service(web::scope("/categories").configure(category::init_routes))
            // .service(web::scope("/posts_tags").configure(posts_tags::init_routes))
            .service(web::scope("/tags").configure(tags::init_routes))
            .service(web::resource("/api.json").route(web::get().to(
                |oapi: web::Data<Pool>| async move {
                    // let json_api = oapi.as_ref().api.clone();
                    // let json_api = openapi.get_ref().openapi().clone(); // Access openapi from app data

                    let json_api = ApiDoc::openapi();

                    HttpResponse::Ok().json(json_api.clone())
                },
            )))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(bind_addr)?
    .bind_uds("/tmp/auth-uds.socket")?
    .run();

    server.await
}
