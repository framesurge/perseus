use std::rc::Rc;
use sycamore::{prelude::{RcSignal, Scope, create_rc_signal, try_use_context}, view::{self, View}, web::Html};
use web_sys::Element;
use crate::{error_views::{ErrorPosition, ErrorViews}, errors::ClientError, utils::{render_or_hydrate, replace_head}};
use super::Reactor;

impl<G: Html> Reactor<G> {
    /// This reports an error to the failsafe mechanism, which will handle it appropriately. This will
    /// determine the capabilities the error view will have access to from the scope provided.
    ///
    /// This **does not** handle widget errors (unless they're popups).
    pub(crate) fn report_err(cx: Scope, err: &ClientError) {
        // Determine where this should be placed
        let pos = match try_use_context::<Reactor<G>>(cx) {
            Some(reactor) => match reactor.is_first.get() {
                // On an initial load, we'll use a popup, unless it's a server-given error
                true => match err {
                    ClientError::ServerError { .. } => ErrorPosition::Page,
                    _ => ErrorPosition::Popup,
                },
                // On a subsequent load, this is the responsibility of the user
                false => match self.error_views.subsequent_err_should_be_popup(err) {
                    true => ErrorPosition::Popup,
                    false => ErrorPosition::Page,
                }
            }
            // There's no reactor, so this was critical
            None => ErrorPosition::Popup
        };

        let (head_str, body_view) = self.error_views.handle(cx, err, pos);

        match pos {
            // For page-wide errors, we need to set the head
            ErrorPosition::Page => {
                replace_head(&head_str);
                self.current_view.set(body_view);
            },
            ErrorPosition::Popup => {
                self.popup_err_view.set(body_view);
            },
            // We don't handle widget errors in this function
            ErrorPosition::Widget => unreachable!(),
        }
    }
}
