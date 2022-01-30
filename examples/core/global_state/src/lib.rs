mod error_pages;
mod global_state;
mod templates;

use perseus::define_app;
define_app! {
    templates: [
        crate::templates::index::get_template::<G>(),
        crate::templates::about::get_template::<G>()
    ],
    error_pages: crate::error_pages::get_error_pages(),
    global_state_creator: crate::global_state::get_global_state_creator()
}
