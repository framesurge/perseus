use crate::errors::*;
use crate::serve::PageData;
use crate::template::Template;
use crate::ClientTranslationsManager;
use crate::Translator;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use sycamore::prelude::Template as SycamoreTemplate;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

pub(crate) async fn fetch(url: &str) -> Result<Option<String>> {
    let js_err_handler = |err: JsValue| ErrorKind::JsErr(format!("{:?}", err));
    let mut opts = RequestInit::new();
    opts.method("GET").mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts).map_err(js_err_handler)?;

    let window = web_sys::window().unwrap();
    // Get the response as a future and await it
    let res_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(js_err_handler)?;
    // Turn that into a proper response object
    let res: Response = res_value.dyn_into().unwrap();
    // If the status is 404, we should return that the request worked but no file existed
    if res.status() == 404 {
        return Ok(None);
    }
    // Get the body thereof
    let body_promise = res.text().map_err(js_err_handler)?;
    let body = JsFuture::from(body_promise).await.map_err(js_err_handler)?;

    // Convert that into a string (this will be `None` if it wasn't a string in the JS)
    let body_str = body.as_string();
    let body_str = match body_str {
        Some(body_str) => body_str,
        None => bail!(ErrorKind::AssetNotString(url.to_string())),
    };
    // Handle non-200 error codes
    if res.status() == 200 {
        Ok(Some(body_str))
    } else {
        bail!(ErrorKind::AssetNotOk(
            url.to_string(),
            res.status(),
            body_str
        ))
    }
}

/// The callback to a template the user must provide for error pages. This is passed the status code, the error message, the URL of the
/// problematic asset, and a translator if one is available . Many error pages are generated when a translator is not available or
/// couldn't be instantiated, so you'll need to rely on symbols or the like in these cases.
pub type ErrorPageTemplate<G> =
    Box<dyn Fn(&str, &u16, &str, Option<Rc<Translator>>) -> SycamoreTemplate<G>>;

/// A type alias for the `HashMap` the user should provide for error pages.
pub struct ErrorPages {
    status_pages: HashMap<u16, ErrorPageTemplate<DomNode>>,
    fallback: ErrorPageTemplate<DomNode>,
}
impl ErrorPages {
    /// Creates a new definition of error pages with just a fallback.
    pub fn new(fallback: ErrorPageTemplate<DomNode>) -> Self {
        Self {
            status_pages: HashMap::default(),
            fallback,
        }
    }
    /// Adds a new page for the given status code. If a page was already defined for the given code, it will be updated by the mechanics of
    /// the internal `HashMap`.
    pub fn add_page(&mut self, status: u16, page: ErrorPageTemplate<DomNode>) {
        self.status_pages.insert(status, page);
    }
    /// Renders the appropriate error page to the given DOM container.
    pub fn render_page(
        &self,
        url: &str,
        status: &u16,
        err: &str,
        translator: Option<Rc<Translator>>,
        container: &NodeRef<DomNode>,
    ) {
        // Check if we have an explicitly defined page for this status code
        // If not, we'll render the fallback page
        let template_fn = match self.status_pages.contains_key(status) {
            true => self.status_pages.get(status).unwrap(),
            false => &self.fallback,
        };
        // Render that to the given container
        sycamore::render_to(
            || template_fn(url, status, err, translator),
            &container.get::<DomNode>().inner_element(),
        );
    }
    /// Gets the template for a page without rendering it into a container.
    pub fn get_template_for_page(
        &self,
        url: &str,
        status: &u16,
        err: &str,
        translator: Option<Rc<Translator>>,
    ) -> SycamoreTemplate<DomNode> {
        // Check if we have an explicitly defined page for this status code
        // If not, we'll render the fallback page
        let template_fn = match self.status_pages.contains_key(status) {
            true => self.status_pages.get(status).unwrap(),
            false => &self.fallback,
        };

        template_fn(url, status, err, translator)
    }
}

/// Fetches the information for the given page and renders it. This should be provided the actual path of the page to render (not just the
/// broader template).
// TODO handle exceptions higher up
pub fn app_shell(
    path: String,
    template: Template<DomNode>,
    locale: String,
    translations_manager: Rc<RefCell<ClientTranslationsManager>>,
    error_pages: Rc<ErrorPages>,
) -> SycamoreTemplate<DomNode> {
    // Get the container as a DOM element
    let container = NodeRef::new();
    // Spawn a Rust futures thread in the background to fetch the static HTML/JSON
    wasm_bindgen_futures::spawn_local(cloned!((container) => async move {
        // Get the static page data
        let asset_url = format!("/.perseus/page/{}/{}", locale, path.to_string());
        // If this doesn't exist, then it's a 404 (we went here by explicit navigation, but it may be an unservable ISR page or the like)
        let page_data_str = fetch(&asset_url).await;
        match page_data_str {
            Ok(page_data_str) => match page_data_str {
                Some(page_data_str) => {
                    // All good, deserialize the page data
                    let page_data = serde_json::from_str::<PageData>(&page_data_str);
                    match page_data {
                        Ok(page_data) => {
                            // We have the page data ready, render everything
                            // Interpolate the HTML directly into the document (we'll hydrate it later)
                            let container_elem = container.get::<DomNode>().unchecked_into::<web_sys::Element>();
                            container_elem.set_inner_html(&page_data.content);

                            // Now that the user can see something, we can get the translator
                            let mut translations_manager_mut = translations_manager.borrow_mut();
                            // This gets an `Rc<Translator>` that references the translations manager, meaning no cloning of translations
                            let translator = translations_manager_mut.get_translator_for_locale(&locale).await;
                            let translator = match translator {
                                Ok(translator) => translator,
                                Err(err) => match err.kind() {
                                    // These errors happen because we couldn't get a translator, so they certainly don't get one
                                    ErrorKind::AssetNotOk(url, status, _) => return error_pages.render_page(url, status, &err.to_string(), None, &container),
                                    ErrorKind::AssetSerFailed(url, _) => return error_pages.render_page(url, &500, &err.to_string(), None, &container),
                                    ErrorKind::LocaleNotSupported(locale) => return error_pages.render_page(&format!("/{}/...", locale), &404, &err.to_string(),None,  &container),
                                    // No other errors should be returned
                                    _ => panic!("expected 'AssetNotOk'/'AssetSerFailed'/'LocaleNotSupported' error, found other unacceptable error")
                                }
                            };

                            // Hydrate that static code using the acquired state
                            // BUG (Sycamore): this will double-render if the component is just text (no nodes)
                            sycamore::hydrate_to(
                                // This function provides translator context as needed
                                || template.render_for_template(page_data.state, Rc::clone(&translator)),
                                &container.get::<DomNode>().inner_element()
                            );
                        },
                        // If the page failed to serialize, an exception has occurred
                        Err(err) => panic!("page data couldn't be serialized: '{}'", err)
                    };
                },
                // No translators ready yet
                None => error_pages.render_page(&asset_url, &404, "page not found", None, &container),
            },
            Err(err) => match err.kind() {
                // No translators ready yet
                ErrorKind::AssetNotOk(url, status, _) => error_pages.render_page(url, status, &err.to_string(), None, &container),
                // No other errors should be returned
                _ => panic!("expected 'AssetNotOk' error, found other unacceptable error")
            }
        };
    }));

    // This is where the static content will be rendered
    // BUG: white flash of death until Sycamore can suspend the router until the static content is ready
    template! {
        div(ref = container)
    }
}
