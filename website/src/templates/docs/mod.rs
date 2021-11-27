mod container;
pub mod generation; // This needs to be public so that we can reuse the `parse_md_to_html` function
mod get_file_at_version;
mod icons;
mod template;

pub use template::get_template;
