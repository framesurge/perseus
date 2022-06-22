mod core; // So called because this contains what is essentially the core exposed logic of Perseus
#[cfg(not(target_arch = "wasm32"))]
mod default_headers;
mod page_props;
mod render_ctx;
#[cfg(not(target_arch = "wasm32"))]
mod states;
mod templates_map;

pub use self::core::*; // There are a lot of render function traits in here, there's no point in spelling them all out
#[cfg(not(target_arch = "wasm32"))]
pub use default_headers::default_headers;
pub use page_props::PageProps;
pub use render_ctx::RenderCtx;
#[cfg(not(target_arch = "wasm32"))]
pub use states::States;
pub use templates_map::{ArcTemplateMap, TemplateMap};
