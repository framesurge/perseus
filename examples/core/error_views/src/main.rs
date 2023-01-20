mod error_views;
mod templates;

use perseus::prelude::*;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        // The same convention of a function to return the needed `struct` is
        // used for both templates and error views
        .error_views(crate::error_views::get_error_views())
        // This lets you specify a siimpel function to be executed when a panic
        // occurs. This will happen *before* the error is formatted and sent to
        // your usual error views system. This is best used for crash analytics,
        // although you must be careful to make sure this function cannot panic
        // itself, or no error message will be sent to the user, and the app will
        // appear to freeze entirely.
        //
        // Note that Perseus automatically writes an explanatory message to the console
        // before any further panic action is taken, just in case the visual message
        // doesn't work out for whatever reason, so there's no need to do that here.
        .panic_handler(|_panic_info| perseus::web_log!("The app has panicked."))
}
