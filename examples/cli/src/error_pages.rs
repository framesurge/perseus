use perseus::ErrorPages;
use sycamore::template;

pub fn get_error_pages() -> ErrorPages {
    let mut error_pages = ErrorPages::new(Box::new(|_, _, _, _| {
        template! {
            p { "Another error occurred." }
        }
    }));
    error_pages.add_page(
        404,
        Box::new(|_, _, _, _| {
            template! {
                p { "Page not found." }
            }
        }),
    );
    error_pages.add_page(
        400,
        Box::new(|_, _, _, _| {
            template! {
                p { "Client error occurred..." }
            }
        }),
    );

    error_pages
}
