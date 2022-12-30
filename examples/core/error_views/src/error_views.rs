use perseus::errors::ClientError;
use perseus::prelude::*;
use sycamore::prelude::*;

// Like templates, error views are generic over `G`, so that they can be
// rendered ahead of time on the engine-side when an error occurs
pub fn get_error_views<G: Html>() -> ErrorViews<G> {
    // Creating a set of error views is a matter of creating a single handler
    // function that can respond to any error. This handler takes a Sycamore scope,
    // the actual error (`perseus::errors::ClientError`), some information about
    // the context of the error (e.g. do we have access to a translator?), and the
    // position of the error.
    //
    // The `ErrorContext` is the most important thing to take account of, after the
    // error itself, as it will twll you what you have access to. For example, if
    // an error occurs while trying to set up translations, you won't have access to
    // a translator, which can be a problem if you're using i18n. Alternately, in a
    // critical error, you might not even have access to a `Reactor`, meaning no
    // global state, no preloading, etc. Importantly, if an error view is rendered
    // on the engine-side, it will most likely not have access to a global state,
    // even if it does have a `Reactor`, due to internal constraints.
    //
    // The error position is one of `Page`, `Widget`, or `Popup`, which may impact
    // your styling choices. For example, it would be a little pointless to style
    // your error view for a full-page layout if it's going to be rendered in a
    // tiny little popup. Errors that occur in widgets will replace the widget.
    // Popup errors are usually used for internal problems, such as corrupted state,
    // rather than an error that the user is more likely to be able to do something
    // about, like a 404. On an initial load, when the user first comes to a page on
    // your site, only HTTP errors (e.g. 404 not found, 500 internal server error)
    // will take up the whole page: anything else, like an internal error, will
    // be rendered as a popup, as the actual content of the page will already
    // have been prerendered, and it would be pointless and frustrating for the
    // user to have it replaced with an error message. On subsequent loads, the
    // same rule is upheld by default, but you can control this becavior by
    // setting the `subsequent_load_determinant` on this `ErrorViews` `struct`
    // (see the API docs for further details).
    //
    // This handler is expected to return a tuple of two views: the first one for
    // the `<head>` (usually containing a title only, as it's not a good idea to
    // load extra material like new stylesheets on an error, as it might be a
    // network error), and the second one for the body (to be displayed in
    // `err_pos`).
    ErrorViews::new(|cx, err, _err_info, _err_pos| {
        match err {
            // Errors from the server, like 404s; these are best displayed over the whole
            // page
            ClientError::ServerError {
                status,
                // This is fully formatted with newlines and tabs for the error and its causes
                message: _
            } => match status {
                // This one is usually handled separately
                404 => (
                    view! { cx,
                        title { "Page not found" }
                    },
                    view! { cx,
                        p { "Sorry, that page doesn't seem to exist." }
                    }
                ),
                // If the status is 4xx, it's a client-side problem (which is weird, and might indicate tampering)
                _ if (400..500).contains(&status) => (
                    view! { cx,
                        title { "Error" }
                    },
                    view! { cx,
                        p { "There was something wrong with the last request, please try reloading the page." }
                    }
                ),
                // 5xx is a server error
                _ => (
                    view! { cx,
                        title { "Error" }
                    },
                    view! { cx,
                        p { "Sorry, our server experienced an internal error. Please try reloading the page." }
                    }
                )
            },
            // A panic (yes, you can handle them here!). After this error is displayed, the entire
            // app will terminate, so buttons or other reactive elements are pointless.
            //
            // The argument here is the formatted panic message.
            ClientError::Panic(_) => (
                view! { cx,
                    title { "Critical error" }
                },
                view! { cx,
                    p { "Sorry, but a critical internal error has occurred. This has been automatically reported to our team, who'll get on it as soon as possible. In the mean time, please try reloading the page." }
                }
            ),
            // Network errors (but these could be caused by unexpected server rejections)
            ClientError::FetchError(_) => (
                view! { cx,
                    title { "Error" }
                },
                view! { cx,
                    p { "A network error occurred, do you have an internet connection? (If you do, try reloading the page.)" }
                }
            ),

            // Usually, everything below here will just be handled with a wildcard
            // for simplicity.

            // An internal failure within Perseus (these can very rarely happen due
            // to network errors or corruptions)
            ClientError::InvariantError(_) |
            // Only if you're using plugins
            ClientError::PluginError(_) |
            // Only if you're using state freezing
            ClientError::ThawError(_) |
            // Severe failures in working with the browser (this doesn't do a lot
            // right now, but it will in future, as Perseus supports PWAs etc.)
            ClientError::PlatformError(_) |
            // Only if you're using preloads (these are usually better
            // caught at the time of the function's execution, but sometimes
            // you'll just want to leave them to a popup error)
            ClientError::PreloadError(_) => (
                view! { cx,
                    title { "Error" }
                },
                view! { cx,
                    p { (format!("An internal error has occurred: '{}'.", err)) }
                }
            )
        }
    })
}
