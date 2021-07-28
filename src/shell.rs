use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{JsFuture};
use web_sys::{Request, RequestInit, RequestMode, Response};
use sycamore::prelude::*;
use serde::{Serialize, de::DeserializeOwned};
use crate::errors::*;
use crate::serve::PageData;
use crate::page::TemplateFn;

pub async fn fetch(url: String) -> Result<Option<String>> {
    let js_err_handler = |err: JsValue| ErrorKind::JsErr(format!("{:?}", err));
    let mut opts = RequestInit::new();
    opts
        .method("GET")
        .mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts).map_err(js_err_handler)?;

    let window = web_sys::window().unwrap();
    // Get the response as a future and await it
    let res_value = JsFuture::from(window.fetch_with_request(&request)).await.map_err(js_err_handler)?;
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
    // TODO return error if string serialization fails
    Ok(body.as_string())
}

/// Fetches the information for the given page and renders it. This should be provided the actual path of the page to render (not just the
/// broader template).
// TODO fix static lifetime here
// TODO set up errors here
pub fn app_shell<Props: Serialize + DeserializeOwned + 'static>(path: String, template_fn: TemplateFn<Props, DomNode>) -> Template<DomNode> {
    // Get the container as a DOM element
    let container = NodeRef::new();
    // Spawn a Rust futures thread in the background to fetch the static HTML/JSON
    wasm_bindgen_futures::spawn_local(cloned!((container) => async move {
        // Get the static page data
        // If this doesn't exist, then it's a 404 (but we went here by explicit navigation, so there's been an error, should go to a special 404 page)
        let page_data_str = fetch(format!("/.perseus/page/{}", path.to_string())).await;
        let page_data_str = match page_data_str {
            Ok(page_data) => match page_data {
                Some(page_data) => page_data,
                None => todo!("404 not yet implemented")
            },
            Err(err) => todo!("error unimplemented")
        };
        let page_data = serde_json::from_str::<PageData>(&page_data_str);
        let page_data = match page_data {
            Ok(page_data) => page_data,
            Err(err) => todo!("page data serialization error unimplemented")
        };

        // Interpolate the HTML directly into the document (we'll hydrate it later)
        let container_elem = container.get::<DomNode>().unchecked_into::<web_sys::Element>();
        container_elem.set_inner_html(&page_data.content);

        let state = match page_data.state {
            Some(state_str) => {
                let state_res = serde_json::from_str::<Props>(&state_str);
                match state_res {
                    Ok(state) => Some(state),
                    Err(err) => todo!("serialization error unimplemented")
                }
            },
            None => None
        };

        // Hydrate that static code using the acquired state
        // BUG (Sycamore): this will double-render if the component is just text (no nodes)
        sycamore::hydrate_to(
            || template_fn(state),
            &container.get::<DomNode>().inner_element()
        );
    }));

    // This is where the static content will be rendered
    // BUG: white flash of death until Sycamore can suspend the router until the static content is ready
    template! {
        div(ref = container)
    }
}
