mod components;
mod error_pages;
mod templates;

use perseus::{define_app, plugins::Plugins};
use perseus_size_opt::{perseus_size_opt, SizeOpts};

define_app! {
    templates: [
        templates::index::get_template::<G>(),
        templates::comparisons::get_template::<G>(),
        templates::docs::get_template::<G>()
    ],
    error_pages: error_pages::get_error_pages(),
    locales: {
        default: "en-US",
        other: []
    },
    plugins: Plugins::new().plugin(perseus_size_opt(), SizeOpts::default())
}
