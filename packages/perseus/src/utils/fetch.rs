use crate::errors::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

/// Fetches the given resource. This is heavily intertwined with the Perseus error
/// management system, and should not be used by end users.
pub(crate) async fn fetch(url: &str, ty: AssetType) -> Result<Option<String>, ClientError> {
    let js_err_handler = |err: JsValue| FetchError::Js(format!("{:?}", err));
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
    // If the status is 404, we should return that the request worked but no file
    // existed (there is a `NotFound` error type, but that's only used for preloading)
    if res.status() == 404 {
        return Ok(None);
    }
    // Get the body thereof
    let body_promise = res.text().map_err(js_err_handler)?;
    let body = JsFuture::from(body_promise).await.map_err(js_err_handler)?;

    // Convert that into a string (this will be `None` if it wasn't a string in the
    // JS)
    let body_str = body.as_string();
    let body_str = match body_str {
        Some(body_str) => body_str,
        None => {
            return Err(FetchError::NotString {
                url: url.to_string(),
                ty,
            }
            .into())
        }
    };
    // Handle non-200 error codes
    if res.status() == 200 {
        Ok(Some(body_str))
    } else {
        Err(FetchError::NotOk {
            url: url.to_string(),
            status: res.status(),
            err: body_str,
            ty
        }
        .into())
    }
}
