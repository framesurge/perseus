#[cfg(not(target_arch = "wasm32"))]
mod async_fn_trait;
#[cfg(not(target_arch = "wasm32"))]
mod cache_res;
#[cfg(target_arch = "wasm32")]
mod checkpoint;
mod decode_time_str;
#[cfg(target_arch = "wasm32")]
mod fetch;
mod log;
#[cfg(not(target_arch = "wasm32"))]
mod minify;
mod path_prefix;
mod render;
#[cfg(target_arch = "wasm32")]
mod replace_head;
mod test;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) use async_fn_trait::AsyncFnReturn;
#[cfg(not(target_arch = "wasm32"))]
pub use cache_res::{cache_fallible_res, cache_res};
#[cfg(target_arch = "wasm32")]
pub use checkpoint::checkpoint;
pub use decode_time_str::{ComputedDuration, InvalidDuration, PerseusDuration}; /* These have dummy equivalents for the browser */
#[cfg(target_arch = "wasm32")]
pub(crate) use fetch::fetch;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use minify::minify;
pub use path_prefix::*;
#[cfg(target_arch = "wasm32")]
pub(crate) use render::render_or_hydrate;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use render::ssr_fallible;
#[cfg(target_arch = "wasm32")]
pub(crate) use replace_head::replace_head;
