use perseus::{define_app, ErrorPages, Template};
use sycamore::template;
define_app! {
    templates: [
        Template::<G>::new("index").template(|_| {
            template! {
                p { "Hello World!" }
            }
        })
    ],
    error_pages: ErrorPages::new(|url, status, err, _| {
        template! {
            p { (format!("An error with HTTP code {} occurred at '{}': '{}'.", status, url, err)) }
        }
    })
}
