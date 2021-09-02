use crate::errors::*;
use crate::serve::PageData;
use crate::template::TemplateFn;
use std::collections::HashMap;
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

/// The callback to a template the user must provide for error pages. This is passed the status code, the error message, and the URL of
/// the problematic asset.
pub type ErrorPageTemplate<G> = Box<dyn Fn(&str, &u16, &str) -> SycamoreTemplate<G>>;

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
    pub fn add_page(&mut self, status: u16, page: ErrorPageTemplate<DomNode>) {
        self.status_pages.insert(status, page);
    }
    pub fn render_page(&self, url: &str, status: &u16, err: &str, container: &NodeRef<DomNode>) {
        // Check if we have an explicitly defined page for this status code
        // If not, we'll render the fallback page
        let template_fn = match self.status_pages.contains_key(status) {
            true => self.status_pages.get(status).unwrap(),
            false => &self.fallback,
        };
        // Render that to the given container
        sycamore::render_to(
            || template_fn(url, status, err),
            &container.get::<DomNode>().inner_element(),
        );
    }
    /// Gets the template for a page without rendering it into a container.
    pub fn get_template_for_page(
        &self,
        url: &str,
        status: &u16,
        err: &str,
    ) -> SycamoreTemplate<DomNode> {
        // Check if we have an explicitly defined page for this status code
        // If not, we'll render the fallback page
        let template_fn = match self.status_pages.contains_key(status) {
            true => self.status_pages.get(status).unwrap(),
            false => &self.fallback,
        };

        template_fn(url, status, err)
    }
}

/// Fetches the information for the given page and renders it. This should be provided the actual path of the page to render (not just the
/// broader template).
// TODO handle exceptions higher up
pub fn app_shell(
    path: String,
    template_fn: TemplateFn<DomNode>,
    error_pages: ErrorPages,
) -> Template<DomNode> {
    // Get the container as a DOM element
    let container = NodeRef::new();
    // Spawn a Rust futures thread in the background to fetch the static HTML/JSON
    wasm_bindgen_futures::spawn_local(cloned!((container) => async move {
        // Get the static page data
        let asset_url = format!("/.perseus/page/{}", path.to_string());
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

                            // Hydrate that static code using the acquired state
                            // BUG (Sycamore): this will double-render if the component is just text (no nodes)
                            sycamore::hydrate_to(
                                || template_fn(page_data.state),
                                &container.get::<DomNode>().inner_element()
                            );
                        },
                        // If the page failed to serialize, an exception has occurred
                        Err(err) => panic!("page data couldn't be serialized: '{}'", err)
                    };
                },
                None => error_pages.render_page(&asset_url, &404, "page not found", &container),
            },
            Err(err) => match err.kind() {
                ErrorKind::AssetNotOk(url, status, err) => error_pages.render_page(url, status, err, &container),
                // No other errors should be returned
                _ => panic!("expected 'AssetNotOk' error, found other unacceptable error")
            }
        };
    }));

    // This is where the static content will be rendered
    // BUG: white flash of death until Sycamore can suspend the router until the static content is ready
    // PageToRender::Success(
    //     template! {
    //         div(ref = container)
    //     }
    // )
    template! {
        div(ref = container)
    }
}
