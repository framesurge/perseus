use perseus::{ErrorPages, Html};
use sycamore::view;

pub fn get_error_pages<G: Html>() -> ErrorPages<G> {
    let mut error_pages = ErrorPages::new(
        |cx, url, status, err, _| {
            view! { cx,
                p { (format!("An error with HTTP code {} occurred at '{}': '{}'.", status, url, err)) }
            }
        },
        |cx, _, _, _, _| {
            view! { cx,
                title { "Error" }
            }
        },
    );
    error_pages.add_page(
        404,
        |cx, _, _, _, _| {
            view! { cx,
                p { "Page not found." }
            }
        },
        |cx, _, _, _, _| {
            view! { cx,
                title { "Not Found" }
            }
        },
    );

    error_pages
}
