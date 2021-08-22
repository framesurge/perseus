pub mod about;
pub mod index;
pub mod ip;
pub mod new_post;
pub mod post;
pub mod time;
pub mod time_root;

use perseus::{get_templates_map, Template};
use std::collections::HashMap;
use sycamore::prelude::GenericNode;

/// Shorthand for the `get_templates_map!` macro from Perseus for our specific app pages.
pub fn get_templates_map<G: GenericNode>() -> HashMap<String, Template<G>> {
    get_templates_map![
        index::get_page::<G>(),
        about::get_page::<G>(),
        post::get_page::<G>(),
        new_post::get_page::<G>(),
        ip::get_page::<G>(),
        time::get_page::<G>(),
        time_root::get_page::<G>()
    ]
}
