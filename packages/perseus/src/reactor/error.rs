use super::Reactor;
use crate::{
    error_views::{ErrorContext, ErrorPosition, ErrorViews},
    errors::ClientError,
    template::TemplateNodeType,
    utils::{render_or_hydrate, replace_head},
};
use std::{panic::PanicInfo, rc::Rc, sync::Arc};
use sycamore::{
    prelude::{
        create_rc_signal, create_scope_immediate, try_use_context, view, RcSignal, Scope,
        ScopeDisposer,
    },
    view::View,
    web::{Html, SsrNode},
};
use web_sys::Element;

impl Reactor<TemplateNodeType> {
    /// This reports an error to the failsafe mechanism, which will handle it
    /// appropriately. This will determine the capabilities the error view
    /// will have access to from the scope provided.
    ///
    /// This returns the disposer for the underlying error scope, which must be
    /// handled appropriately, or a memory leak will occur. Leaking an error
    /// scope is never permissible. A boolean of whether or not the error took
    /// up the whole page or not is also returned, which can be used to guide
    /// what should be done with the disposer.
    ///
    /// Obviously, since this is a method on a reactor, this does not handle
    /// critical errors caused by not being able to create a reactor.
    ///
    /// This **does not** handle widget errors (unless they're popups).
    #[must_use]
    pub(crate) fn report_err<'a>(
        &self,
        cx: Scope<'a>,
        err: &ClientError,
    ) -> (ScopeDisposer<'a>, bool) {
        // Determine where this should be placed
        let pos = match self.is_first.get() {
            // On an initial load, we'll use a popup, unless it's a server-given error
            true => match err {
                ClientError::ServerError { .. } => ErrorPosition::Page,
                _ => ErrorPosition::Popup,
            },
            // On a subsequent load, this is the responsibility of the user
            false => match self.error_views.subsequent_err_should_be_popup(err) {
                true => ErrorPosition::Popup,
                false => ErrorPosition::Page,
            },
        };

        let (head_str, body_view, disposer) = self.error_views.handle(cx, err, pos);

        match pos {
            // For page-wide errors, we need to set the head
            ErrorPosition::Page => {
                replace_head(&head_str);
                self.current_view.set(body_view);
                (disposer, true)
            }
            ErrorPosition::Popup => {
                self.popup_error_view.set(body_view);
                (disposer, false)
            }
            // We don't handle widget errors in this function
            ErrorPosition::Widget => unreachable!(),
        }
    }

    /// Creates the infrastructure necessary to handle a critical error, and
    /// then displays it. This is intended for use if the reactor cannot be
    /// instantiated, and it takes the app-level context to verify this.
    ///
    /// # Panics
    /// This will panic if given a scope in which a reactor exists.
    ///
    /// # Visibility
    /// This is broadly part of Perseus implementation details, and is exposed
    /// only for those foregoing `#[perseus::main]` or
    /// `#[perseus::browser_main]` to build their own custom browser-side
    /// entrypoint (do not do this unless you really need to).
    pub fn handle_critical_error(
        cx: Scope,
        err: &ClientError,
        error_views: &ErrorViews<TemplateNodeType>,
    ) {
        // We do NOT want this called if there is a reactor (but, if it is, we have no
        // clue about the calling situation, so it's safest to just panic)
        assert!(try_use_context::<Reactor<TemplateNodeType>>(cx).is_none(), "attempted to handle 'critical' error, but a reactor was found (this is a programming error)");

        let popup_error_root = Self::create_popup_err_elem();
        // This will determine the `Static` error context (we guaranteed there's no
        // reactor above). We don't care about the head in a popup.
        let (_, err_view, disposer) = error_views.handle(cx, err, ErrorPosition::Popup);
        render_or_hydrate(
            cx,
            view! { cx,
                // This is not reactive, as there's no point in making it so
                (err_view)
            },
            popup_error_root,
        );
        // SAFETY: We're outside the child scope
        unsafe {
            disposer.dispose();
        }
    }
    /// Creates the infrastructure necessary to handle a panic, and then
    /// displays an error created by the user's [`ErrorViews`]. This
    /// function will only panic if certain fundamental functions of the web
    /// APIs are not defined, in which case no error message could ever be
    /// displayed to the user anyway.
    ///
    /// A handler is manually provided to this, because the [`ErrorViews`]
    /// are typically not thread-safe once extracted from `PerseusApp`.
    ///
    /// # Visibility
    /// Under absolutely no circumstances should this function **ever** be
    /// called outside a Perseus panic handler set in the entrypoint! It is
    /// exposed for custom entrypoints only.
    pub fn handle_panic(
        panic_info: &PanicInfo,
        handler: Arc<
            dyn Fn(
                    Scope,
                    &ClientError,
                    ErrorContext,
                    ErrorPosition,
                ) -> (View<SsrNode>, View<TemplateNodeType>)
                + Send
                + Sync,
        >,
    ) {
        let popup_error_root = Self::create_popup_err_elem();

        // The standard library handles all the hard parts here
        let msg = panic_info.to_string();
        // The whole app is about to implode, we are not keeping this scope
        // around
        create_scope_immediate(|cx| {
            let (_head, body) = handler(
                cx,
                &ClientError::Panic(msg),
                ErrorContext::Static,
                ErrorPosition::Popup,
            );
            render_or_hydrate(
                cx,
                view! { cx,
                    // This is not reactive, as there's no point in making it so
                    (body)
                },
                popup_error_root,
            );
        });
    }
}
