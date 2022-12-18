use crate::{errors::*, reactor::Reactor};
#[cfg(not(target_arch = "wasm32"))]
use crate::{i18n::Translator, reactor::RenderMode, state::TemplateState};
use fmterr::fmt_err;
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
use sycamore::prelude::create_scope_immediate;
#[cfg(target_arch = "wasm32")]
use sycamore::prelude::{create_child_scope, try_use_context, ScopeDisposer};
use sycamore::{
    prelude::{view, Scope},
    utils::hydrate::with_no_hydration_context,
    view::View,
    web::{Html, SsrNode},
};

/// The error handling system of an app. In Perseus, errors come in several
/// forms, all of which must be handled. This system provides a way to do this
/// automatically, maximizing your app's error tolerance, including against
/// panics.
pub struct ErrorViews<G: Html> {
    /// The central function that parses the error provided and returns a tuple
    /// of views to deal with it: the first view is the document metadata,
    /// and the second the body of the error.
    #[allow(clippy::type_complexity)]
    handler: Box<
        dyn Fn(Scope, ClientError, ErrorContext, ErrorPosition) -> (View<SsrNode>, View<G>)
            + Send
            + Sync,
    >,
    /// A function for determining if a subsequent load error should occupy the
    /// entire page or not. If this returns `true`, the whole page will be
    /// taken over (e.g. for a 404), but, if it returns `false`, a small
    /// popup will be created on the current page (e.g. for an internal
    /// error unrelated to the page itself).
    ///
    /// This is left to user discretion in the case of subsequent loads. For
    /// initial loads, we will render a page-wide error only if it came from
    /// the engine, otherwise just a popup over the prerendered content so
    /// the user can proceed with visibility, but not interactivity.
    subsequent_load_determinant: Box<dyn Fn(&ClientError) -> bool + Send + Sync>,
    /// A verbatim copy of the user's handler, intended for panics. This is
    /// needed because we have to extract it completely and give it to the
    /// standard library in a thread-safe manner (even though Wasm is
    /// single-threaded).
    ///
    /// This will be extracted by the `PerseusApp` creation process and put in a
    /// place where it can be safely extracted. The replacement function
    /// will panic if called, so this should **never** be manually executed.
    #[cfg(target_arch = "wasm32")]
    panic_handler: Arc<
        dyn Fn(Scope, ClientError, ErrorContext, ErrorPosition) -> (View<SsrNode>, View<G>)
            + Send
            + Sync,
    >,
}
impl<G: Html> ErrorViews<G> {
    /// Creates an error handling system for your app with the given handler
    /// function. This will be provided a [`ClientError`] to match against,
    /// along with an [`ErrorContext`], which tells you what you have available
    /// to you (since, in some critical errors, you might not even have a
    /// translator).
    ///
    /// The function given to this should return a tuple of two `View`s: the
    /// first to be placed in document `<head>`, and the second
    /// for the body. For views with `ErrorPosition::Popup` or
    /// `ErrorPosition::Widget`, the head view will be ignored,
    /// and would usually be returned as `View::empty()`.
    pub fn new(
        handler: impl Fn(Scope, ClientError, ErrorContext, ErrorPosition) -> (View<SsrNode>, View<G>)
            + Send
            + Sync
            + Clone
            + 'static,
    ) -> Self {
        Self {
            #[allow(clippy::redundant_clone)]
            handler: Box::new(handler.clone()),
            // Sensible defaults are fine here
            subsequent_load_determinant: Box::new(|err| {
                match err {
                    // Any errors from the server should take up the whole page
                    ClientError::ServerError { .. } => true,
                    // Anything else is internal-ish (e.g. a fetch failure would be a network
                    // failure, so we keep the user where they are)
                    _ => false,
                }
            }),
            #[cfg(target_arch = "wasm32")]
            panic_handler: Arc::new(handler),
        }
    }
    /// Sets the function that determines if an error on a *subsequent load*
    /// should be presented to the user as taking up the whole page, or just
    /// being in a little popup. Usually, you can leave this as the default,
    /// which will display any internal errors as popups, and any errors from
    /// the server (e.g. a 404 not found) as full pages.
    ///
    /// You could use this to create extremely unorthodox patterns like
    /// rendering a popup on the current page if the user clicks a link that
    /// goes to a 404, if you really wanted.
    ///
    /// For widgets, returning `true` from the function you provide to this will
    /// take up the whole widget, as opposed to the whole page.
    ///
    /// *Note: if you want all your errors to take up the whole page no matter
    /// what (not recommended, see the book for why!), you should leave this
    /// function as the default and simply style `#__perseus_error_popup` to
    /// take up the whole page.*
    pub fn subsequent_load_determinant_fn(
        &mut self,
        val: impl Fn(&ClientError) -> bool + Send + Sync + 'static,
    ) -> &mut Self {
        self.subsequent_load_determinant = Box::new(val);
        self
    }

    /// Returns `true` if the given error, which must have occurred during a
    /// subsequent load, should be displayed as a popup, as opposed to
    /// occupying the entire page/widget.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn subsequent_err_should_be_popup(&self, err: &ClientError) -> bool {
        !(self.subsequent_load_determinant)(err)
    }

    /// Force-sets the unlocalized defaults. If you really want to use the
    /// default error pages in production, this will allow you to (where
    /// they would normally fail if you simply specified nothing).
    ///
    /// **Warning:** these defaults are completely unlocalized, unstyled, and
    /// intended for development! You will be able to use these by not
    /// specifying any `.error_views()` on your `PerseusApp` in development,
    /// and you should only use this function if you're doing production
    /// testing of Perseus, and you don't particularly want to write
    /// your own error pages.
    ///
    /// Note that this is used throughout the Perseus examples for brevity.
    pub fn unlocalized_development_default() -> Self {
        // Because this is an unlocalized, extremely simple default, we don't care about
        // capabilities or positioning
        Self::new(|cx, err, _, _| {
            match err {
                // Special case for 404 due to its frequency
                ClientError::ServerError { status, .. } if status == 404 => (
                    view! { cx,
                        title { "Page not found" }
                    },
                    view! { cx,
                        p { "Page not found." }
                    },
                ),
                err => {
                    let err_msg = fmt_err(&err);
                    (
                        view! { cx,
                                title { "Error" }
                        },
                        view! { cx,
                                (format!("An error occurred: {}", err_msg))
                        },
                    )
                }
            }
        })
    }
}
#[cfg(target_arch = "wasm32")]
impl<G: Html> ErrorViews<G> {
    /// Invokes the user's handling function, producing head/body views for the
    /// given error. From the given scope, this will determine the
    /// conditions under which the error can be rendered.
    pub(crate) fn handle<'a>(
        &self,
        cx: Scope<'a>,
        err: ClientError,
        pos: ErrorPosition,
    ) -> (String, View<G>, ScopeDisposer<'a>) {
        let reactor = try_use_context::<Reactor<G>>(cx);
        // From the given scope, we can perfectly determine the capabilities this error
        // view will have
        let info = match reactor {
            Some(reactor) => match reactor.try_get_translator() {
                Some(_) => ErrorContext::Full,
                None => ErrorContext::WithReactor,
            },
            None => ErrorContext::Static,
        };

        let mut body_view = View::empty();
        let mut head_str = String::new();
        let disposer = create_child_scope(cx, |child_cx| {
            let (head_view, body_view_local) = (self.handler)(child_cx, err, info, pos);
            body_view = body_view_local;
            // Stringify the head view with no hydration markers
            head_str = sycamore::render_to_string(|_| with_no_hydration_context(|| head_view));
        });

        (head_str, body_view, disposer)
    }
    /// Extracts the panic handler from within the error views. This should
    /// generally only be called by `PerseusApp`'s error views instantiation
    /// system.
    pub(crate) fn take_panic_handler(
        &mut self,
    ) -> Arc<
        dyn Fn(Scope, ClientError, ErrorContext, ErrorPosition) -> (View<SsrNode>, View<G>)
            + Send
            + Sync,
    > {
        std::mem::replace(
            &mut self.panic_handler,
            Arc::new(|_, _, _, _| unreachable!()),
        )
    }
}
#[cfg(not(target_arch = "wasm32"))]
impl ErrorViews<SsrNode> {
    /// Renders an error view on the engine-side. This takes an optional
    /// translator. This will return a tuple of `String`ified views for the
    /// head and body. For widget errors, the former should be discarded.
    ///
    /// Since the only kind of error that can be sent from the server to the
    /// client falls under a `ClientError::ServerError`, which always takes
    /// up the whole page, and since we presumably don't have any actual
    /// content to render, this will, expectedly, take up the whole page.
    ///
    /// This cannot be used for widgets (use `.handle_widget()` instead).
    ///
    /// # Hydration
    ///
    /// At present, due to the difficulties of controlling hydration contexts
    /// in a fine-grained manner, Perseus does not hydrate error views
    /// whatsoever. This is compounded by the problem of exported error
    /// views, which do not have access to locales, whereas their
    /// browser-side-rendered counterparts do. To avoid hydration mismatches
    /// and unnecessary development panics, hydration is therefore disabled
    /// for error views.
    pub(crate) fn render_to_string(
        &self,
        err: ServerErrorData,
        translator: Option<&Translator>,
    ) -> (String, String) {
        // We need to create an engine-side reactor
        let reactor =
            Reactor::<SsrNode>::engine(TemplateState::empty(), RenderMode::Error, translator);
        let mut body_str = String::new();
        let mut head_str = String::new();
        create_scope_immediate(|cx| {
            reactor.add_self_to_cx(cx);
            // Depending on whether or not we had a translator, we can figure out the
            // capabilities
            let err_cx = match translator {
                // On the engine-side, we don't get global state (see docs for
                // `ErrorContext::FullNoGlobal`)
                Some(_) => ErrorContext::FullNoGlobal,
                None => ErrorContext::WithReactor,
            };
            // NOTE: No hydration context
            let (head_view, body_view) = (self.handler)(
                cx,
                ClientError::ServerError {
                    status: err.status,
                    message: err.msg,
                },
                err_cx,
                ErrorPosition::Page,
            );

            head_str = sycamore::render_to_string(|_| with_no_hydration_context(|| head_view));
            body_str = sycamore::render_to_string(|_| body_view);
        });

        (head_str, body_str)
    }
}
impl<G: Html> ErrorViews<G> {
    /// Renders an error view for the given widget, using the given scope. This
    /// will *not* create a new child scope, it will simply use the one it is
    /// given.
    ///
    /// Since this only handles widgets, it will automatically discard the head.
    ///
    /// This assumes the reactor has already been fully set up with a translator
    /// on the given context, and hence this will always use
    /// `ErrorContext::Full` (since widgets shoudl not be rendered if a
    /// translator cannot be found, and certainly not if a reactor could not
    /// be instantiated).
    pub(crate) fn handle_widget(&self, err: ClientError, cx: Scope) -> View<G> {
        let (_head, body) = (self.handler)(cx, err, ErrorContext::Full, ErrorPosition::Page);
        body
    }
}

/// The context of an error, which determines what is available to your views.
/// This *must* be checked before using things like translators, which may not
/// be available, depending on the information in here.
pub enum ErrorContext {
    /// Perseus has suffered an unrecoverable error in initialization, and
    /// routing/interactivity is impossible. Your error view will be
    /// rendered to the page, and then Perseus will terminate completely.
    /// This means any buttons, handlers, etc. *will not run*!
    ///
    /// If you're having trouble with this, imagine printing out your error
    /// view. That's the amount of functionality you get (except that the
    /// browser will automatically take over any links). If you want
    /// interactivity, you *could* use `dangerously_set_inner_html` to create
    /// some JS handlers, for instance for offering the user a button to
    /// reload the page.
    Static,
    /// Perseus suffered an error before it was able to create a translator.
    /// Your error view will be rendered inside a proper router, and you'll
    /// have a [`Reactor`] available in context, but using the `t!` or
    /// `link!` macros will lead to a panic. If you present links to other pages
    /// in the app, the user will be able to press them, and these will try
    /// to set up a translator, but this may fail.
    ///
    /// If your app doesn't use internationalization, Perseus does still have a
    /// dummy translator internally, so this doesn't completely evaporate,
    /// but you can ignore it.
    ///
    /// *Note: currently, if the user goes to, say
    /// `/en-US/this-page-does-not-exist`, even though the page is clearly
    /// localized, Perseus will not provide a translator. This will be rectified
    /// in a future version. If the user attempted to switch locales, and
    /// there was an error fetching translations for the new one, the old
    /// translator will be provided here.*
    WithReactor,
    /// Perseus was able to successfully instantiate a reactor and translator,
    /// but this error view is being rendered on the engine-side, and there is
    /// no global state available.
    ///
    /// Although global state could theoretically be provided to error pages
    /// *sometimes*, the complexity and cloning involved make this extremely
    /// nuanced (e.g. exported error pages can't access localized global
    /// state because they don't know their locale, global state might be
    /// only partially built at the time of the error, etc.). In
    /// general, error views rendered on the engine-side will have this (though
    /// not always).
    FullNoGlobal,
    /// Perseus was able to successfully instantiate everything, including a
    /// translator, but then it encountered an error. You have access to all
    /// the usual things you would have in a page here.
    ///
    /// Note that this would also be given to you on the engine-side when you
    /// have a translator available, but when you're still rendering to an
    /// [`SsrNode`].
    Full,
}

/// Where an error is being rendered. Most of the time, you'll use this for
/// determining how you want to style an error view. For instance, you probably
/// don't want giant text saying "Page not found!" if the error is actually
/// going to be rendered inside a tiny little widget.
///
/// Note that you should also always check if you have a `Popup`-style error, in
/// which case there will be no router available, so any links will be handled
/// by the browser's default behavior.
#[derive(Clone, Copy)]
pub enum ErrorPosition {
    /// The error will take up the whole page.
    Page,
    /// The error will be confined to the widget that caused it.
    Widget,
    /// The error is being rendered in a little popup, and no router is
    /// available.
    ///
    /// This is usually reserved for internal errors, where something has gone
    /// severely wrong.
    Popup,
}

/// The information to render an error on the server-side, which is usually
/// associated with an explicit HTTP status code.
///
/// Note that these will never be generated at build-time, any problems there
/// will simply cause an error. However, errors in the build process during
/// incremental generation *will* return one of these.
///
/// This `struct` is embedded in the HTML provided to the client, allowing it to
/// be extracted and rendered.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerErrorData {
    /// The HTTP status code of the error (since these errors are always
    /// transmitted from server to client).
    pub(crate) status: u16,
    /// The actual error message. In error pages that are exported, this will be
    /// simply the `reason-phrase` for the referenced status code,
    /// containing no more information, since it isn't available at
    /// export-time, of course.
    pub(crate) msg: String,
}

// --- Default error views (development only) ---
#[cfg(debug_assertions)] // This will fail production compilation neatly
impl<G: Html> Default for ErrorViews<G> {
    fn default() -> Self {
        Self::unlocalized_development_default()
    }
}
