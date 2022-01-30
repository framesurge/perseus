use perseus::{ErrorPages, Html};
use sycamore::view;

pub fn get_error_pages<G: Html>() -> ErrorPages<G> {
    let mut error_pages = ErrorPages::new(|url, status, err, _| {
        view! {
            p { (format!("An error with HTTP code {} occurred at '{}': '{}'.", status, url, err)) }
        }
    });
    error_pages.add_page(404, |_, _, _, _| {
        view! {
            p { "Page not found." }
        }
    });

    error_pages
}
