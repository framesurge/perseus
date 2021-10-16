use perseus::{ErrorPages, GenericNode};
use sycamore::template;

pub fn get_error_pages<G: GenericNode>() -> ErrorPages<G> {
    let mut error_pages = ErrorPages::new(|_, _, _, _| {
        template! {
            p { "Another error occurred." }
        }
    });
    error_pages.add_page(404, |_, _, err, _| {
        template! {
            p { "Page not found." }
            p { (err) }
        }
    });
    error_pages.add_page(400, |_, _, _, _| {
        template! {
            p { "Client error occurred..." }
        }
    });

    error_pages
}
