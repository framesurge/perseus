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
pub use sycamore::{DomNode, SsrNode};

pub use crate::build::{build_template, build_templates};
pub use crate::config_manager::{ConfigManager, FsConfigManager};
pub use crate::decode_time_str::decode_time_str;
pub use crate::serve::{get_page, get_render_cfg};
pub use crate::shell::{app_shell, ErrorPages};
pub use crate::template::{Template, TemplateMap, States, StringResult, StringResultWithCause};
pub use crate::errors::{err_to_status_code, ErrorCause};