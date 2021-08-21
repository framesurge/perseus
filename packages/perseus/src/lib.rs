pub mod build;
pub mod config_manager;
pub mod decode_time_str;
pub mod errors;
pub mod serve;
pub mod shell;
pub mod template;

pub use http;
pub use http::Request as HttpRequest;
/// All HTTP requests use empty bodies for simplicity of passing them around. They'll never need payloads (value in path requested).
pub type Request = HttpRequest<()>;