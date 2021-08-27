mod help;
mod prepare;
pub mod errors;

pub const PERSEUS_VERSION: &str = "0.1.0";
pub use help::help;
pub use prepare::{prepare, check_env};
