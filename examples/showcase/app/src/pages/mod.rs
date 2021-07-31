pub mod index;
pub mod about;
pub mod post;

use perseus::{get_templates_map, template::Template};
use sycamore::prelude::GenericNode;
use std::collections::HashMap;

/// Shorthand for the `get_templates_map!` macro from Perseus for our specific app pages.
pub fn get_templates_map<G: GenericNode>() -> HashMap<String, Template<G>> {
	get_templates_map! [
        index::get_page::<G>(),
        about::get_page::<G>(),
        post::get_page::<G>()
    ]
}