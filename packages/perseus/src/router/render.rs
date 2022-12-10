use std::cell::Cell;

use sycamore::{generic_node::GenericNode, prelude::{create_scope, create_signal}, utils::hydrate::with_hydration_context, web::{DomNode, HydrateNode}};
use web_sys::Node;

use crate::{error_views::ErrorPosition, errors::ClientError, reactor::{Failsafe, InitialView}, template::TemplateNodeType};

/// Renders the given Perseus app, handling all errors automatically.
///
/// While [`Reactor`] is the core of the browser-side logic, this function is what
/// uses the reactor to show the user actual content. Note that almost everything
/// in this function is event-driven through reactivity.
///
/// This will automatically set up panic handling infrastructure.
pub fn render<M: MutableStore, T: TranslationsManager>(
    mut app: PerseusAppBase<TemplateNodeType, M, T>,
) {
    let panic_handler = app.take_panic_handler();

    checkpoint("begin");

    // Handle panics (this works for unwinds and aborts)
    // TODO New system
    std::panic::set_hook(Box::new(move |panic_info| {
        // Print to the console in development
        #[cfg(debug_assertions)]
        console_error_panic_hook::hook(panic_info);
        // If the user wants a little warning dialogue, create that
        if let Some(panic_handler) = &panic_handler {
            panic_handler(panic_info);
        }
    }));



    // Everything here is event-driven: we want one handler for prepared views that
    // are ready to be rendered, and another for mini-errors that should display over
    // the current content (so a browser-only error doesn't mean the user can't read
    // an article, for example).
    //
    // Importantly, the view handler will be used for whole-page errors, such as
    // `ClientError::ServerError`, when we know the user is seeing an error anyway.
    //
    // The scope created here will be used for the app's lifetime, and is never disposed
    // of. We render/hydrate using this scope, bypassing Sycamore's usual internal scope
    // creation.
    let _ = create_scope(|cx| {
        // This will run inside a hydration context if we're using hydration
        let core = move || {
            // Set up the error handling mechanism (this will set all handlers automatically)
            let failsafe = Failsafe::new(cx, app.get_error_views());

            let page_view = create_signal(cx, View::empty());
            // We'll add to this...
            let error = create_signal::<Option<ClientError>>(cx, None);
            // ...and this will be updated (but sometimes it'll be a page error, so not always)
            let popup_error_view = create_signal(cx, View::empty());
            let is_first = Cell::new(true);
            // This will be used to track the special case of having an initial load that
            // fails with a server-side error, in which case we should *render* (not hydrate)
            // a page-wide error view. If this is found to be `true`, then we'll render the
            let initial_load_server_err = Cell::new(false);

            // Error handler
            create_effect(cx, || {
                let error = error.get();
                if let Some(err) = &*error {
                    // This is the one type of error that is guaranteed to be a page-wide error
                    if let ClientError::ServerError { .. } = err {
                        let err_view = todo!();
                        page_view.set(err_view);
                    } else {
                        // On an initial load, any other error will be a popup. On a subsequent load,
                        // we'll ask the user what they want (e.g. a 404 should probably be a new page,
                        // but a 502 is arguable).
                        let pagewide = if is_first.get() {
                            false
                        } else {
                            todo!()
                        };
                        if pagewide {
                            let err_view = todo!();
                            page_view.set(err_view);
                        } else {
                            let err_view = todo!();
                            popup_error_view.set(err_view);
                        }
                    }
                }
            });

            // We now have a mechanism for handling any error, so we can start the fallible work

            // 1. Create the reactor (error => popup)
            // 2. Instantiate the handlers (error => popup)
            // 3. Get the initial view (error => popup OR full page)
            let initial_return = match Reactor::try_from(app)
                .map(|reactor| reactor.start(cx)) {
                    Ok(_) => {
                        // This is the app-wide scope, so this will be available to all pages
                        reactor.add_self_to_cx(cx);
                        // Try to get the initial view (if we've gotten this far, this will
                        // determine whether or not the app fails)
                        match reactor.get_initial_view(cx) {
                            // This will either be a redirect or a view to render
                            Ok(initial_view_res) => InitialReturn::View(initial_view_res),
                            // TODO Return the view here, don't pop it up
                            Err(ClientError::ServerError { .. }) => failsafe.report_err(cx, &err, ErrorPosition::Page),
                            Err(err) => failsafe.report_err(cx, &err, ErrorPosition::Popup),
                        }
                    },
                    // Hopefully, the user can see some content, so we'll render a popup
                    Err(err) => failsafe.report_err(cx, &err, ErrorPosition::Popup)
                };

            match initial_return {
                InitialReturn::View(_) | InitialReturn::PageError(_) => {
                    // For any type of page-wide error, we nee
                },
                // There will be no router for an initial load popup error
                InitialReturn::PopupError(err_view) => {
                    popup_error_root.set_inner_html("");
                    render_or_hydrate(cx, err_view, popup_error_root);
                },
            }
        };
        // If we're using hydration, everything has to be done inside a hydration context
        #[cfg(feature = "hydrate")]
        with_hydration_context(|| core());
        #[cfg(not(feature = "hydrate"))]
        core();
    });


}

/// The possible outcomes of an initial render.
enum InitialReturn<'app, G: Html> {
    /// There's a full view ready to be rendered or hydrated,
    /// depending on feature flags. We can proceed to subsequent
    /// loads and the like.
    ///
    /// Note that this might also actually be a redirect.
    View(InitialView<'app, G>),
    /// We're leaving the view as it was rendered on the engine,
    /// and just creating a little popup for some error that prevented
    /// the render.
    PopupError(View<G>),
    /// There was an engine-originating error that implies the prerendered
    /// page is showing an error, so we'll render the error we have to
    /// the whole page. Because all the rest of Perseus' machinery is
    /// operational in the case of this kind of error, the page is still
    /// perfectly reactive, just on an error, meaning the router etc.
    /// will all work.
    PageError(View<G>),
}
