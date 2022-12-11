mod core; // So called because this contains what is essentially the core exposed logic of Perseus
#[cfg(not(target_arch = "wasm32"))]
mod default_headers;
// mod render_ctx;
mod capsule;
mod fn_types;
#[cfg(not(target_arch = "wasm32"))]
mod states;
mod templates_map;
mod widget_component;

pub use self::core::*;
pub use fn_types::*; /* There are a lot of render function traits in here, there's no
                      * point in spelling them all out */
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use default_headers::default_headers;
// pub use render_ctx::RenderCtx;
// pub(crate) use render_ctx::{RenderMode, RenderStatus};
pub use capsule::Capsule;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use states::States;
pub use templates_map::{ArcCapsuleMap, ArcTemplateMap, CapsuleMap, TemplateMap};
pub use widget_component::Widget;
