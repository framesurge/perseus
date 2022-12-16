//! This module contains the necessary primitives to run Perseus as a server,
//! regardless of framework. This module aims to provide as many abstractions as
//! possible to minimize work when maintaining multiple server-framework
//! integrations. Apart from building your own integrations, you should never
//! need to use this module (though some plugins may need types in here).

mod html_shell;
mod options;

pub(crate) use html_shell::HtmlShell;
pub use options::ServerOptions;

/// Removes empty elements from a path, which is important due to double
/// slashes. This returns a vector of the path's components;
pub fn get_path_slice(path: &str) -> Vec<&str> {
    let path_slice: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();

    path_slice
}
