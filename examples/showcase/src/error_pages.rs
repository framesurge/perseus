use perseus::{ErrorPages, GenericNode};
use std::rc::Rc;
use sycamore::template;

pub fn get_error_pages<G: GenericNode>() -> ErrorPages<G> {
    let mut error_pages = ErrorPages::new(Rc::new(|_, _, _, _| {
        template! {
            p { "Another error occurred." }
        }
    }));
    error_pages.add_page(
        404,
        Rc::new(|_, _, _, _| {
            template! {
                p { "Page not found." }
            }
        }),
    );
    error_pages.add_page(
        400,
        Rc::new(|_, _, _, _| {
            template! {
                p { "Client error occurred..." }
            }
        }),
    );

    error_pages
}
