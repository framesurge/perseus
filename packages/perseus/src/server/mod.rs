//! This module contains the necessary primitives to run Perseus as a server, regardless of framework. This module aims to provide as many abstractions as possible
//! to minimize work when maintaining multiple server-framework integrations. Apart from building your own integrations, you should never need to use this module.

mod build_error_page;
mod get_render_cfg;
mod html_shell;
mod options;
mod page_data;
mod render;

pub use build_error_page::build_error_page;
pub use get_render_cfg::get_render_cfg;
pub use html_shell::HtmlShell;
pub use options::{ServerOptions, ServerProps};
pub use page_data::PageData;
pub use render::{get_page, get_page_for_template};

/// Removes empty elements from a path, which is important due to double slashes. This returns a vector of the path's components;
pub fn get_path_slice(path: &str) -> Vec<&str> {
    let path_slice: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();

    path_slice
}
