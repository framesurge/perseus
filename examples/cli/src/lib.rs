mod error_pages;
mod pages;

use perseus::define_app;

define_app! {
    templates: [
        crate::pages::index::get_page::<G>(),
        crate::pages::about::get_page::<G>()
    ],
    error_pages: crate::error_pages::get_error_pages()
}
