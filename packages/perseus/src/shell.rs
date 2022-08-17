use crate::error_pages::ErrorPageData;
use crate::errors::*;
use crate::i18n::ClientTranslationsManager;
use crate::page_data::PageData;
use crate::router::{RouteVerdict, RouterLoadState, RouterState};
use crate::template::{PageProps, Template, TemplateNodeType};
use crate::utils::get_path_prefix_client;
use crate::ErrorPages;
use fmterr::fmt_err;
use std::collections::HashMap;
use std::rc::Rc;
use sycamore::prelude::*;
use sycamore::rt::Reflect; // We can piggyback off Sycamore to avoid bringing in `js_sys`
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Element, Request, RequestInit, RequestMode, Response};

/// Fetches the given resource. This should NOT be used by end users, but it's
/// required by the CLI.
pub(crate) async fn fetch(url: &str) -> Result<Option<String>, ClientError> {
    let js_err_handler = |err: JsValue| ClientError::Js(format!("{:?}", err));
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
    // existed
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
        }
        .into())
    }
}

/// Properties for the app shell. These should be constructed literally when
/// working with the app shell.
// #[derive(Debug)]
pub(crate) struct ShellProps<'a> {
    /// The app's reactive scope.
    pub cx: Scope<'a>,
    /// The path we're rendering for (not the template path, the full path,
    /// though parsed a little).
    pub path: String,
    /// The template to render for.
    pub template: Rc<Template<TemplateNodeType>>,
    /// Whether or not the router returned an incremental match (if this page
    /// exists on a template using incremental generation and it wasn't defined
    /// at build time).
    pub was_incremental_match: bool,
    /// The locale we're rendering in.
    pub locale: String,
    /// The router state.
    pub router_state: RouterState,
    /// A *client-side* translations manager to use (this manages caching
    /// translations).
    pub translations_manager: ClientTranslationsManager,
    /// The error pages, for use if something fails.
    pub error_pages: Rc<ErrorPages<TemplateNodeType>>,
    /// The current route verdict. This will be stored in context so that it can
    /// be used for possible reloads. Eventually, this will be made obsolete
    /// when Sycamore supports this natively.
    pub route_verdict: RouteVerdict<TemplateNodeType>,
}
