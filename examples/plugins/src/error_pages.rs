use perseus::{ErrorPages, GenericNode};
use sycamore::template;

pub fn get_error_pages<G: GenericNode>() -> ErrorPages<G> {
    let mut error_pages = ErrorPages::new(|url, status, err, _| {
        template! {
            p { (format!("An error with HTTP code {} occurred at '{}': '{}'.", status, url, err)) }
        }
    });
    error_pages.add_page(404, |_, _, _, _| {
        template! {
            p { "Page not found." }
        }
    });

    error_pages
}
