mod container;
pub mod generation; // This needs to be public so that we can reuse the `parse_md_to_html` function
#[cfg(not(target_arch = "wasm32"))]
mod get_file_at_version;
mod icons;
mod search_bar;
mod template;

#[cfg(not(target_arch = "wasm32"))]
pub use get_file_at_version::get_file_at_version;
pub use template::get_template;
