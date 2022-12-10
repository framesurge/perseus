mod async_fn_trait;
#[cfg(not(target_arch = "wasm32"))]
mod cache_res;
#[cfg(target_arch = "wasm32")]
mod checkpoint;
mod context;
mod decode_time_str;
#[cfg(target_arch = "wasm32")]
mod fetch;
mod log;
#[cfg(not(target_arch = "wasm32"))]
mod minify;
mod path_prefix;
#[cfg(target_arch = "wasm32")]
mod replace_head;
mod test;
#[cfg(target_arch = "wasm32")]
mod render_or_hydrate;

pub(crate) use async_fn_trait::AsyncFnReturn;
#[cfg(not(target_arch = "wasm32"))]
pub use cache_res::{cache_fallible_res, cache_res};
#[cfg(target_arch = "wasm32")]
pub use checkpoint::checkpoint;
pub(crate) use context::provide_context_signal_replace;
pub use decode_time_str::{ComputedDuration, InvalidDuration, PerseusDuration}; /* These have dummy equivalents for the browser */
#[cfg(target_arch = "wasm32")]
pub(crate) use fetch::fetch;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use minify::minify;
pub use path_prefix::*;
#[cfg(target_arch = "wasm32")]
pub(crate) use replace_head::replace_head;
#[cfg(target_arch = "wasm32")]
pub(crate) use render_or_hydrate::render_or_hydrate;
