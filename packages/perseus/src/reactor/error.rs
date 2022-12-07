use sycamore::{prelude::Scope, view::View, web::Html};
use crate::{error_pages::ErrorPages, errors::ClientError};
use super::Reactor;

impl<G: Html> Reactor<G> {
    /// Takes the given error and returns a [`View`] to handle it gracefully.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn handle_error(&self, cx: Scope, err: ClientError) -> View<G> {
        Self::handle_critical_error(cx, err, &self.error_pages)
    }

    /// A version of `.handle_error()` intended to be used when the `Reactor` could
    /// not be constructed. This will render a [`View`], but there will be no router.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn handle_critical_error(cx: Scope, err: ClientError, error_pages: &ErrorPages<G>) -> View<G> {
        todo!()
    }
}
