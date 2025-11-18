use perseus::errors::ClientError;
use perseus::prelude::*;
use sycamore::prelude::*;

pub fn get_error_views() -> ErrorViews {
    ErrorViews::new(|err, _err_info, _err_pos| {
        match err {
            ClientError::ServerError { status, message: _ } => match status {
                404 => (
                    view! {
                        title { "Page not found" }
                    },
                    view! {
                        p { "Sorry, that page doesn't seem to exist." }
                    },
                ),
                // 4xx is a client error
                _ if (400..500).contains(&status) => (
                    view! {
                        title { "Error" }
                    },
                    view! {
                        p { "There was something wrong with the last request, please try reloading the page." }
                    },
                ),
                // 5xx is a server error
                _ => (
                    view! {
                        title { "Error" }
                    },
                    view! {
                        p { "Sorry, our server experienced an internal error. Please try reloading the page." }
                    },
                ),
            },
            ClientError::Panic(_) => (
                view! {
                    title { "Critical error" }
                },
                view! {
                    p { "Sorry, but a critical internal error has occurred. This has been automatically reported to our team, who'll get on it as soon as possible. In the mean time, please try reloading the page." }
                },
            ),
            ClientError::FetchError(_) => (
                view! {
                    title { "Error" }
                },
                view! {
                    p { "A network error occurred, do you have an internet connection? (If you do, try reloading the page.)" }
                },
            ),
            _ => (
                view! {
                    title { "Error" }
                },
                view! {
                    p { (format!("An internal error has occurred: '{}'.", err)) }
                },
            ),
        }
    })
}
