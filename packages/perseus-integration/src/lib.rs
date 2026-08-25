#![cfg(engine)]

// Ensure only one server integration is enabled at a time
#[cfg(all(feature = "actix-web", feature = "axum"))]
compile_error!("Only one server integration feature can be enabled at a time");
#[cfg(all(feature = "actix-web", feature = "rocket"))]
compile_error!("Only one server integration feature can be enabled at a time");
#[cfg(all(feature = "actix-web", feature = "warp"))]
compile_error!("Only one server integration feature can be enabled at a time");
#[cfg(all(feature = "axum", feature = "rocket"))]
compile_error!("Only one server integration feature can be enabled at a time");
#[cfg(all(feature = "axum", feature = "warp"))]
compile_error!("Only one server integration feature can be enabled at a time");
#[cfg(all(feature = "rocket", feature = "warp"))]
compile_error!("Only one server integration feature can be enabled at a time");

#[cfg(feature = "actix-web")]
pub use perseus_actix_web::dflt_server;
#[cfg(feature = "axum")]
pub use perseus_axum::dflt_server;
#[cfg(feature = "rocket")]
pub use perseus_rocket::dflt_server;
#[cfg(feature = "warp")]
pub use perseus_warp::dflt_server;
