use crate::PathMaybeWithLocale;
use crate::translator::Translator;
#[cfg(target_arch = "wasm32")]
use crate::utils::replace_head;
use crate::Html;
use crate::SsrNode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use sycamore::prelude::Scope;
use sycamore::utils::hydrate::with_no_hydration_context;
use sycamore::view;
use sycamore::view::View;

/// The function to a template the user must provide for error pages. This is
/// passed the status code, the error message, the URL of the problematic asset,
/// and a translator if one is available . Many error pages are generated when a
/// translator is not available or couldn't be instantiated, so you'll need to
/// rely on symbols or the like in these cases.
pub type ErrorPageTemplate<G> =
    Box<dyn Fn(Scope, ErrorPageLocation, u16, String, Option<Rc<Translator>>) -> View<G> + Send + Sync>;
/// The function the user must provide to render the document `<head>`
/// associated with a certain error page. Note that this will only be rendered
/// on the server-side, and will be completely unreactive, being directly
/// interpolated into the document metadata on the client-side if the error page
/// is loaded.
pub type ErrorPageHeadTemplate = ErrorPageTemplate<SsrNode>;

/// A representation of the views configured in an app for responding to errors.
///
/// On the web, errors occur frequently beyond app logic, usually in
/// communication with servers, which will return [HTTP status codes](https://httpstatuses.com/) that indicate
/// a success or failure. If a non-success error code is received, then Perseus
/// will automatically render the appropriate error page, based on that status
/// code. If no page has been explicitly constructed for that status code, then
/// the fallback page will be used.
///
/// Each error page is a closure returning a [`View`] that takes four
/// parameters: a reactive scope, the URL the user was on when the error
/// occurred (which they'll still be on, no route change occurs when rendering
/// an error page), the status code itself, a `String` of the actual error
/// message, and a [`Translator`] (which may not be available if the error
/// occurred before translations data could be fetched and processed, in which
/// case you should try to display language-agnostic information).
///
/// The second closure to each error page is for the document `<head>` that will
/// be rendered in conjunction with that error page. Importantly, this is
/// completely unreactive, and is rendered to a string on the engine-side.
///
/// In development, you can get away with not defining any error pages for your
/// app, as Perseus has a simple inbuilt default, though, when you try to go to
/// production (e.g. with `perseus deploy`), you'll receive an error message in
/// building. In other words, you must define your own error pages for release
/// mode.
pub struct ErrorPages<G: Html> {
    status_pages: HashMap<u16, (ErrorPageTemplate<G>, ErrorPageHeadTemplate)>,
    fallback: (ErrorPageTemplate<G>, ErrorPageHeadTemplate),
}
impl<G: Html> std::fmt::Debug for ErrorPages<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ErrorPages").finish()
    }
}
impl<G: Html> ErrorPages<G> {
    /// Creates a new definition of error pages with just a fallback page, which
    /// will be used when an error occurs whose status code has not been
    /// explicitly handled by some other error page.
    pub fn new(
        fallback_page: impl Fn(Scope, ErrorPageLocation, u16, String, Option<Rc<Translator>>) -> View<G>
            + Send
            + Sync
            + 'static,
        fallback_head: impl Fn(Scope, ErrorPageLocation, u16, String, Option<Rc<Translator>>) -> View<SsrNode>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self {
            status_pages: HashMap::default(),
            fallback: (Box::new(fallback_page), Box::new(fallback_head)),
        }
    }
    /// Adds a new page for the given status code. If a page was already defined
    /// for the given code, it will be updated by replacement, through the
    /// mechanics of the internal `HashMap`. While there is no requirement
    /// for this to be a valid HTTP status code, there would be no point in
    /// defining a handler for a status code not on [this list](https://httpstatuses.com)
    pub fn add_page(
        &mut self,
        status: u16,
        page: impl Fn(Scope, ErrorPageLocation, u16, String, Option<Rc<Translator>>) -> View<G>
            + Send
            + Sync
            + 'static,
        head: impl Fn(Scope, ErrorPageLocation, u16, String, Option<Rc<Translator>>) -> View<SsrNode>
            + Send
            + Sync
            + 'static,
    ) {
        self.status_pages
            .insert(status, (Box::new(page), Box::new(head)));
    }
    /// Adds a new page for the given status code. If a page was already defined
    /// for the given code, it will be updated by the mechanics of
    /// the internal `HashMap`. This differs from `.add_page()` in that it takes
    /// a `Box`, which can be useful for plugins.
    pub fn add_page_boxed(
        &mut self,
        status: u16,
        page: ErrorPageTemplate<G>,
        head: ErrorPageHeadTemplate,
    ) {
        self.status_pages.insert(status, (page, head));
    }
    /// Gets the internal template function to render.
    fn get_template_fn(&self, status: u16) -> &ErrorPageTemplate<G> {
        // Check if we have an explicitly defined page for this status code
        // If not, we'll render the fallback page
        match self.status_pages.contains_key(&status) {
            true => &self.status_pages.get(&status).unwrap().0,
            false => &self.fallback.0,
        }
    }
    /// Gets the `View<G>` to render the content.
    pub fn get_view(
        &self,
        cx: Scope,
        loc: ErrorPageLocation,
        status: u16,
        err: &str,
        translator: Option<Rc<Translator>>,
    ) -> View<G> {
        let template_fn = self.get_template_fn(status);
        template_fn(cx, loc, status, err.to_string(), translator)
    }
    /// Gets the `View<G>` to render the content and automatically renders and
    /// replaces the document `<head>` appropriately.
    #[cfg(target_arch = "wasm32")]
    pub fn get_view_and_render_head(
        &self,
        cx: Scope,
        loc: ErrorPageLocation,
        status: u16,
        err: &str,
        translator: Option<Rc<Translator>>,
    ) -> View<G> {
        let head = self.render_head(url, status, err, translator.clone());
        replace_head(&head);
        self.get_view(cx, url, status, err, translator)
    }
    /// Renders the head of an error page to a `String`.
    ///
    /// This is needed on the browser-side to render error pages that occur
    /// abruptly.
    pub fn render_head(
        &self,
        loc: ErrorPageLocation,
        status: u16,
        err: &str,
        translator: Option<Rc<Translator>>,
    ) -> String {
        let head_fn = match self.status_pages.contains_key(&status) {
            true => &self.status_pages.get(&status).unwrap().1,
            false => &self.fallback.1,
        };
        sycamore::render_to_string(|cx| {
            with_no_hydration_context(|| {
                head_fn(cx, loc, status, err.to_string(), translator)
            })
        })
    }
}
// #[cfg(target_arch = "wasm32")]
// impl ErrorPages<DomNode> {
//     /// Renders the appropriate error page to the given DOM container.
//     pub fn render_page(
//         &self,
//         cx: Scope,
//         url: &str,
//         status: u16,
//         err: &str,
//         translator: Option<Rc<Translator>>,
//         container: &Element,
//     ) {
//         let template_fn = self.get_template_fn(status);
//         // Render that to the given container
//         sycamore::render_to(
//             |_| template_fn(cx, loc, status, err.to_string(),
// translator),             container,
//         );
//     }
// }
// #[cfg(target_arch = "wasm32")]
// impl ErrorPages<HydrateNode> {
//     /// Hydrates the appropriate error page to the given DOM container. This
// is     /// used for when an error page is rendered by the server and then
// needs     /// interactivity.
//     pub fn hydrate_page(
//         &self,
//         cx: Scope,
//         url: &str,
//         status: u16,
//         err: &str,
//         translator: Option<Rc<Translator>>,
//         container: &Element,
//     ) {
//         let template_fn = self.get_template_fn(status);
//         let hydrate_view = template_fn(cx, loc, status,
// err.to_string(), translator);         // TODO Now convert that `HydrateNode`
// to a `DomNode`         let dom_view = hydrate_view;
//         // Render that to the given container
//         sycamore::hydrate_to(|_| dom_view, container);
//     }
//     /// Renders the appropriate error page to the given DOM container. This
// is     /// implemented on `HydrateNode` to avoid having to have two `Html`
// type     /// parameters everywhere (one for templates and one for error
// pages).     // TODO Convert from a `HydrateNode` to a `DomNode`
//     pub fn render_page(
//         &self,
//         cx: Scope,
//         url: &str,
//         status: u16,
//         err: &str,
//         translator: Option<Rc<Translator>>,
//         container: &Element,
//     ) {
//         let template_fn = self.get_template_fn(status);
//         // Render that to the given container
//         sycamore::hydrate_to(
//             |_| template_fn(cx, loc, status, err.to_string(),
// translator),             container,
//         );
//     }
// }
#[cfg(not(target_arch = "wasm32"))]
impl ErrorPages<SsrNode> {
    /// Renders the error page to a string. This should then be hydrated on the
    /// client-side. No reactive scope is provided to this function, it uses an
    /// internal one.
    pub fn render_to_string(
        &self,
        loc: ErrorPageLocation,
        status: u16,
        err: &str,
        translator: Option<Rc<Translator>>,
    ) -> String {
        let template_fn = self.get_template_fn(status);
        // Render that to the given container
        sycamore::render_to_string(|cx| {
            template_fn(cx, loc, status, err.to_string(), translator)
        })
    }
    /// Renders the error page to a string, using the given reactive scope. Note
    /// that this function is not used internally, and `.render_to_string()`
    /// should cover all uses. This is included for completeness.
    pub fn render_to_string_scoped(
        &self,
        cx: Scope,
        loc: ErrorPageLocation,
        status: u16,
        err: &str,
        translator: Option<Rc<Translator>>,
    ) -> String {
        let template_fn = self.get_template_fn(status);
        // Render that to the given container
        sycamore::render_to_string(|_| {
            template_fn(cx, loc, status, err.to_string(), translator)
        })
    }
}
// We provide default error pages to speed up development, but they have to be
// added before moving to production (or we'll `panic!`)
impl<G: Html> Default for ErrorPages<G> {
    #[cfg(debug_assertions)]
    fn default() -> Self {
        let mut error_pages = Self::new(
            |cx, _, status, err, _| {
                view! { cx,
                    p { (format!("An error with HTTP code {} occurred: '{}'.", status, err)) }
                }
            },
            |cx, _, _, _, _| {
                view! { cx,
                    title { "Error" }
                }
            },
        );
        // 404 is the most common by far, so we add a little page for that too
        error_pages.add_page(
            404,
            |cx, _, _, _, _| {
                view! { cx,
                    p { "Page not found." }
                }
            },
            |cx, _, _, _, _| {
                view! { cx,
                    title { "Not Found" }
                }
            },
        );

        error_pages
    }
    #[cfg(not(debug_assertions))]
    fn default() -> Self {
        panic!("you must provide your own error pages in production")
    }
}

/// A representation of an error page, particularly for storage in transit so
/// that server-side rendered error pages can be hydrated on the client-side.
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorPageData {
    /// The location at which the error occurred.
    pub location: ErrorPageLocation,
    /// THe HTTP status code that corresponds with the error.
    pub status: u16,
    /// The actual error message as a string.
    pub err: String,
}

/// The possible locations that might be provided to an error page.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ErrorPageLocation {
    /// A properly formed Perseus path.
    Path(PathMaybeWithLocale),
    /// The current path. Note that this will not always be used for the
    /// current path if it can expressed fully. This most commonly appears
    /// in error pages that have been exported, and in severe serialization
    /// errors.
    Current,
    /// This error is not page-specific, and represents a critical violation
    /// of Perseus' trans-network invariants. For instance, we assume that the
    /// server will give us correctly formed state. If this doesn't happen, we can't
    /// take any action. An invariant violation like this would cause a panic
    /// if it weren't network-based, since it's entirely plausible for there to be
    /// some kind of network error that leads to malformed state being received.
    Core,
}
