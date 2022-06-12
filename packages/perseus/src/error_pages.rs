use crate::translator::Translator;
use crate::Html;
#[cfg(target_arch = "wasm32")]
use crate::{DomNode, HydrateNode};
#[cfg(not(target_arch = "wasm32"))]
use crate::SsrNode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use sycamore::prelude::Scope;
use sycamore::view;
use sycamore::view::View;
#[cfg(target_arch = "wasm32")]
use web_sys::Element;

/// The callback to a template the user must provide for error pages. This is passed the status code, the error message, the URL of the
/// problematic asset, and a translator if one is available . Many error pages are generated when a translator is not available or
/// couldn't be instantiated, so you'll need to rely on symbols or the like in these cases.
pub type ErrorPageTemplate<G> =
    Box<dyn Fn(Scope, String, u16, String, Option<Rc<Translator>>) -> View<G> + Send + Sync>;

/// A type alias for the `HashMap` the user should provide for error pages.
pub struct ErrorPages<G: Html> {
    status_pages: HashMap<u16, ErrorPageTemplate<G>>,
    fallback: ErrorPageTemplate<G>,
}
impl<G: Html> std::fmt::Debug for ErrorPages<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ErrorPages").finish()
    }
}
impl<G: Html> ErrorPages<G> {
    /// Creates a new definition of error pages with just a fallback.
    pub fn new(
        fallback: impl Fn(Scope, String, u16, String, Option<Rc<Translator>>) -> View<G>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self {
            status_pages: HashMap::default(),
            fallback: Box::new(fallback),
        }
    }
    /// Adds a new page for the given status code. If a page was already defined for the given code, it will be updated by the mechanics of
    /// the internal `HashMap`.
    pub fn add_page(
        &mut self,
        status: u16,
        page: impl Fn(Scope, String, u16, String, Option<Rc<Translator>>) -> View<G>
            + Send
            + Sync
            + 'static,
    ) {
        self.status_pages.insert(status, Box::new(page));
    }
    /// Adds a new page for the given status code. If a page was already defined for the given code, it will be updated by the mechanics of
    /// the internal `HashMap`. This differs from `.add_page()` in that it takes an `Rc`, which is useful for plugins.
    pub fn add_page_rc(&mut self, status: u16, page: ErrorPageTemplate<G>) {
        self.status_pages.insert(status, page);
    }
    /// Gets the internal template function to render.
    fn get_template_fn(&self, status: u16) -> &ErrorPageTemplate<G> {
        // Check if we have an explicitly defined page for this status code
        // If not, we'll render the fallback page
        match self.status_pages.contains_key(&status) {
            true => self.status_pages.get(&status).unwrap(),
            false => &self.fallback,
        }
    }
    // /// Gets the template for a page without rendering it into a container.
    // pub fn get_template_for_page(
    //     &self,
    //     cx: Scope,
    //     url: &str,
    //     status: u16,
    //     err: &str,
    //     translator: Option<Rc<Translator>>,
    // ) -> View<G> {
    //     let template_fn = self.get_template_fn(status);

    //     template_fn(cx, url.to_string(), status, err.to_string(), translator)
    // }
}
#[cfg(target_arch = "wasm32")]
impl ErrorPages<DomNode> {
    /// Renders the appropriate error page to the given DOM container.
    pub fn render_page(
        &self,
        cx: Scope,
        url: &str,
        status: u16,
        err: &str,
        translator: Option<Rc<Translator>>,
        container: &Element,
    ) {
        let template_fn = self.get_template_fn(status);
        // Render that to the given container
        sycamore::render_to(
            |_| template_fn(cx, url.to_string(), status, err.to_string(), translator),
            container,
        );
    }
}
#[cfg(target_arch = "wasm32")]
impl ErrorPages<HydrateNode> {
    /// Hydrates the appropriate error page to the given DOM container. This is used for when an error page is rendered by the server
    /// and then needs interactivity.
    pub fn hydrate_page(
        &self,
        cx: Scope,
        url: &str,
        status: u16,
        err: &str,
        translator: Option<Rc<Translator>>,
        container: &Element,
    ) {
        let template_fn = self.get_template_fn(status);
        let hydrate_view = template_fn(cx, url.to_string(), status, err.to_string(), translator);
        // TODO Now convert that `HydrateNode` to a `DomNode`
        let dom_view = hydrate_view;
        // Render that to the given container
        sycamore::hydrate_to(
            |_| dom_view,
            container,
        );
    }
    /// Renders the appropriate error page to the given DOM container. This is implemented on `HydrateNode` to avoid having to have two `Html` type parameters everywhere
    /// (one for templates and one for error pages).
    // TODO Convert from a `HydrateNode` to a `DomNode`
    pub fn render_page(
        &self,
        cx: Scope,
        url: &str,
        status: u16,
        err: &str,
        translator: Option<Rc<Translator>>,
        container: &Element,
    ) {
        let template_fn = self.get_template_fn(status);
        // Render that to the given container
        sycamore::hydrate_to(
            |_| template_fn(cx, url.to_string(), status, err.to_string(), translator),
            container,
        );
    }
}
#[cfg(not(target_arch = "wasm32"))]
impl ErrorPages<SsrNode> {
    /// Renders the error page to a string. This should then be hydrated on the client-side. No reactive scope is provided to this function, it uses an internal one.
    pub fn render_to_string(
        &self,
        url: &str,
        status: u16,
        err: &str,
        translator: Option<Rc<Translator>>,
    ) -> String {
        let template_fn = self.get_template_fn(status);
        // Render that to the given container
        sycamore::render_to_string(|cx| {
            template_fn(cx, url.to_string(), status, err.to_string(), translator)
        })
    }
    /// Renders the error page to a string, using the given reactive scope. Note that this function is not used internally, and `.render_to_string()` should cover all uses. This is included for
    /// completeness.
    pub fn render_to_string_scoped(
        &self,
        cx: Scope,
        url: &str,
        status: u16,
        err: &str,
        translator: Option<Rc<Translator>>,
    ) -> String {
        let template_fn = self.get_template_fn(status);
        // Render that to the given container
        sycamore::render_to_string(|_| {
            template_fn(cx, url.to_string(), status, err.to_string(), translator)
        })
    }
}
// We provide default error pages to speed up development, but they have to be added before moving to production (or we'll `panic!`)
impl<G: Html> Default for ErrorPages<G> {
    #[cfg(debug_assertions)]
    fn default() -> Self {
        let mut error_pages = Self::new(|cx, url, status, err, _| {
            view! { cx,
                p { (format!("An error with HTTP code {} occurred at '{}': '{}'.", status, url, err)) }
            }
        });
        // 404 is the most common by far, so we add a little page for that too
        error_pages.add_page(404, |cx, _, _, _, _| {
            view! { cx,
                p { "Page not found." }
            }
        });

        error_pages
    }
    #[cfg(not(debug_assertions))]
    fn default() -> Self {
        panic!("you must provide your own error pages in production")
    }
}

/// A representation of an error page, particularly for storage in transit so that server-side rendered error pages can be hydrated on
/// the client-side.
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorPageData {
    /// The URL for the error.
    pub url: String,
    /// THe HTTP status code that corresponds with the error.
    pub status: u16,
    /// The actual error message as a string.
    pub err: String,
}
