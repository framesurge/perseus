use perseus::{define_app, ErrorPages, Template};
use sycamore::view;
define_app! {
    templates: [
        Template::<G>::new("index").template(|_| {
            view! {
                p { "Hello World!" }
            }
        })
    ],
    error_pages: ErrorPages::new(|url, status, err, _| {
        view! {
            p { (format!("An error with HTTP code {} occurred at '{}': '{}'.", status, url, err)) }
        }
    })
}
