mod error_pages;
mod plugin;
mod templates;

use perseus::define_app;
use perseus::plugins::Plugins;

define_app! {
    templates: [
        crate::templates::index::get_template::<G>(),
        crate::templates::about::get_template::<G>()
    ],
    error_pages: crate::error_pages::get_error_pages(),
    plugins: Plugins::new()
        .plugin(plugin::get_test_plugin(), ())
}
