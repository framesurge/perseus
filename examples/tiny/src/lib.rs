use perseus::{define_app, ErrorPages, Template};
use std::rc::Rc;
use sycamore::template;
define_app! {
    templates: [
        Template::<G>::new("index").template(Rc::new(|_| {
            template! {
                p { "Hello World!" }
            }
        }))
    ],
    error_pages: ErrorPages::new(Rc::new(|url, status, err, _| {
        template! {
            p { (format!("An error with HTTP code {} occurred at '{}': '{}'.", status, url, err)) }
        }
    }))
}
