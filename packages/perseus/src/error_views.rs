use sycamore::{prelude::{Scope, create_scope_immediate, try_use_context}, utils::hydrate::{with_hydration_context, with_no_hydration_context}, view::View, web::{Html, SsrNode}};
use crate::{errors::{ClientError, ExportError}, i18n::Translator, reactor::{Reactor, RenderMode}, state::TemplateState};

/// The error handling system of an app. In Perseus, errors come in several forms,
/// all of which must be handled. This system provides a way to do this automatically,
/// maximizing your app's error tolerance, including against panics.
pub struct ErrorViews<G: Html> {
    /// The central function that parses the error provided and returns a tuple of views to deal with it:
    /// the first view is the document metadata, and the second the body of the error.
    handler: Box<dyn Fn(Scope, &ClientError, ErrorContext, ErrorPosition) -> (View<G>, View<G>) + Send + Sync>,
    /// A function for determining if a subsequent load error should occupy the entire page
    /// or not. If this returns `true`, the whole page will be taken over (e.g. for a 404),
    /// but, if it returns `false`, a small popup will be created on the current page (e.g.
    /// for an internal error unrelated to the page itself).
    ///
    /// This is left to user discretion in the case of subsequent loads. For initial loads, we
    /// will render a page-wide error only if it came from the engine, otherwise just a popup
    /// over the prerendered content so the user can proceed with visibility, but not interactivity.
    subsequent_load_determinant: Box<dyn Fn(&ClientError) -> bool + Send + Sync>,
}
impl<G: Html> ErrorViews<G> {
    /// Sets the universal error handler. This will be provided a [`ClientError`] to match against,
    /// along with an [`ErrorContext`], which tells you what you have available to you (since, in
    /// some critical errors, you might not even have a translator).
    ///
    /// The function given to this should return a tuple of two `View`s: the first to be placed in document `<head>`, and the second
    /// for the body. For views with `ErrorPosition::Popup` or `ErrorPosition::Widget`, the head view will be ignored,
    /// and would usually be returned as `View::empty()`.
    pub fn handler_fn(&mut self, val: impl Fn(Scope, &ClientError, ErrorContext, ErrorPosition) -> (View<G>, View<G>) + Send + Sync + 'static) -> &mut Self {
        self.handler = Box::new(val);
        self
    }
    /// Sets the function that determines if an error on a *subsequent load* should be presented to
    /// the user as taking up the whole page, or just being in a little popup. Usually, you can leave
    /// this as the default, which will display any internal errors as popups, and any errors from
    /// the server (e.g. a 404 not found) as full pages.
    ///
    /// You could use this to create extremely unorthodox patterns like rendering a popup on the current
    /// page if the user clicks a link that goes to a 404, if you really wanted.
    ///
    /// For widgets, returning `true` from the function you provide to this will take up the whole widget,
    /// as opposed to the whole page.
    ///
    /// *Note: if you want all your errors to take up the whole page no matter what (not recommended, see
    /// the book for why!), you should leave this function as the default and simply style `#__perseus_error_popup`
    /// to take up the whole page.*
    pub fn subsequent_load_determinant_fn(&mut self, val: impl Fn(&ClientError) -> bool + Send + Sync + 'static) -> &mut Self {
        self.subsequent_load_determinant = Box::new(val);
        self
    }


    /// Returns `true` if the given error, which must have occurred during a subsequent load,
    /// should be displayed as a popup, as opposed to occupying the entire page/widget.
    pub(crate) fn subsequent_err_should_be_popup(&self, err: &ClientError) -> bool {
        !(self.subsequent_load_determinant)(err)
    }
}
#[cfg(target_arch = "wasm32")]
impl<G: Html> ErrorView<G> {
    /// Invokes the user's handling function, producing head/body views for the given error. From the given scope,
    /// this will determine the conditions under which the error can be rendered.
    pub(crate) fn handle(&self, cx: Scope, err: &ClientError, pos: ErrorPosition) -> (String, View<G>) {
        let reactor = try_use_context::<Reactor<G>>(cx);
        // From the given scope, we can perfectly determine the capabilities this error view will have
        let info = match reactor {
            Some(reactor) => match reactor.try_get_translator() {
                Some(_) => ErrorContext::Full,
                None => ErrorContext::WithReactor,
            },
            None => ErrorContext::Static,
        };

        let (head_view, body_view) = (self.handler)(cx, err, info, pos);
        // Stringify the head view with no hydration markers
        let head_str = sycamore::render_to_string(|_|
                                                  with_no_hydration_context(|| head_view)
        );

        (head_str, body_view)
    }
}
#[cfg(not(target_arch = "wasm32"))]
impl ErrorViews<SsrNode> {
    /// Renders an error view on the engine-side. This takes an optional translator. This will return
    /// a tuple of `String`ified views for the head and body. For widget errors, the former should be
    /// discarded.
    ///
    /// Since the only kind of error that can be sent from the server to the client falls under
    /// a `ClientError::ServerError`, which always takes up the whole page, and since we presumably
    /// don't have any actual content to render, this will, expectedly, take up the whole page/widget.
    pub(crate) fn render_to_string(
        &self,
        err: ServerErrorData,
        global_state: TemplateState,
        translator: Option<&Translator>,
        is_widget: bool
    ) -> (String, String) {
        // We need to create an engine-side reactor
        let reactor = Reactor::<SsrNode>::engine(global_state, RenderMode::Error, translator);
        let mut body_str;
        let mut head_str;
        create_scope_immediate(|cx| {
            reactor.add_self_to_cx(cx);
            // Depending on whether or not we had a translator, we can figure out the capabilities
            let err_cx = match translator {
                Some(_) => ErrorContext::Full,
                None => ErrorContext::WithReactor,
            };
            let pos = match is_widget {
                true => ErrorPosition::Widget,
                false => ErrorPosition::Page,
            };
            let (head_view, body_view) = with_hydration_context(|| (self.handler)(cx, &ClientError::ServerError { status: err.status, message: err.msg }, err_cx, pos));

            head_str = sycamore::render_to_string(|_| head_view);
            body_str = sycamore::render_to_string(|_| body_view);
        });

        (head_str, body_str)
    }
}

/// The context of an error, which determines what is available to your views.
/// This *must* be checked before using things like translators, which may not
/// be available, depending on the information in here.
pub enum ErrorContext {
    /// Perseus has suffered an unrecoverable error in initialization, and routing/interactivity is
    /// impossible. Your error view will be rendered to the page, and then Perseus will terminate
    /// completely. This means any buttons, handlers, etc. *will not run*!
    ///
    /// If you're having trouble with this, imagine printing out your error view. That's the amount
    /// of functionality you get (except that the browser will automatically take over any links).
    /// If you want interactivity, you *could* use `dangerously_set_inner_html` to create some JS
    /// handlers, for instance for offering the user a button to reload the page.
    Static,
    /// Perseus suffered an error before it was able to create a translator. Your error view will
    /// be rendered inside a proper router, and you'll have a [`Reactor`] available in context, but
    /// using the `t!` or `link!` macros will lead to a panic. If you present links to other pages in
    /// the app, the user will be able to press them, and these will try to set up a translator, but
    /// this may fail.
    ///
    /// If your app doesn't use internationalization, Perseus does still have a dummy translator internally,
    /// so this doesn't completely evaporate, but you can ignore it.
    ///
    /// *Note: currently, if the user goes to, say `/en-US/this-page-does-not-exist`, even though the
    /// page is clearly localized, Perseus will not provide a translator. This will be rectified in
    /// a future version. If the user attempted to switch locales, and there was an error fetching
    /// translations for the new one, the old translator will be provided here.*
    WithReactor,
    /// Perseus was able to successfully instantiate everything, including a translator, but then it encountered
    /// an error. You have access to all the usual things you would have in a page here.
    ///
    /// Note that this would also be given to you on the engine-side when you have a translator available,
    /// but when you're still rendering to an [`SsrNode`].
    Full,
}

/// Where an error is being rendered. Most of the time, you'll use this for determining
/// how you want to style an error view. For instance, you probably don't want giant text
/// saying "Page not found!" if the error is actually going to be rendered inside a tiny little
/// widget.
///
/// Note that you should also always check if you have a `Popup`-style error, in which case
/// there will be no router available, so any links will be handled by the browser's default
/// behavior.
#[derive(Clone, Copy)]
pub enum ErrorPosition {
    /// The error will take up the whole page.
    Page,
    /// The error will be confined to the widget that caused it.
    Widget,
    /// The error is being rendered in a little popup, and no router is available.
    ///
    /// This is usually reserved for internal errors, where something has gone severely wrong.
    Popup
}

/// The information to render an error on the server-side, which is usually associated
/// with an explicit HTTP status code.
///
/// Note that these will never be generated at build-time, any problems there will simply
/// cause an error. However, errors in the build process during incremental generation
/// *will* return one of these.
///
/// This `struct` is embedded in the HTML provided to the client, allowing it to be extracted and rendered.
pub(crate) struct ServerErrorData {
    /// The HTTP status code of the error (since these errors are always transmitted
    /// from server to client).
    pub(crate) status: u16,
    /// The actual error message. In error pages that are exported, this will be simply the
    /// `reason-phrase` for the referenced status code, containing no more information, since
    /// it isn't available at export-time, of course.
    pub(crate) msg: String,
}
#[cfg(not(target_arch = "wasm32"))]
impl ServerErrorData {
    /// Creates the data for a server error based on the given HTTP status code.
    pub(crate) fn from_status(status: u16) -> Result<Self, ExportError> {
        use http::StatusCode;

        let status_code = StatusCode::from_u16(status).map_err(|_| ExportError::InvalidStatusCode)?;
        // If we can't get a reason, thats okay (but, looking at status.rs in `http`, we should always be able to)
        let reason_str = status_code.canonical_reason().unwrap_or_default();

        Ok(Self {
            status,
            msg: reason_str.to_string(),
        })
    }
}
