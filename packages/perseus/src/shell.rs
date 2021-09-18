use crate::error_pages::ErrorPageData;
use crate::errors::*;
use crate::serve::PageData;
use crate::template::Template;
use crate::ClientTranslationsManager;
use crate::ErrorPages;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use sycamore::prelude::Template as SycamoreTemplate;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Element, Request, RequestInit, RequestMode, Response};

/// Fetches the given resource. This should NOT be used by end users, but it's required by the CLI.
#[doc(hidden)]
pub async fn fetch(url: &str) -> Result<Option<String>> {
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

/// Gets the render configuration from the JS global variable `__PERSEUS_RENDER_CFG`, which should be inlined by the server. This will
/// return `None` on any error (not found, serialization failed, etc.), which should reasonably lead to a `panic!` in the caller.
pub fn get_render_cfg() -> Option<HashMap<String, String>> {
    let val_opt = web_sys::window().unwrap().get("__PERSEUS_RENDER_CFG");
    let js_obj = match val_opt {
        Some(js_obj) => js_obj,
        None => return None,
    };
    // The object should only actually contain the string value that was injected
    let cfg_str = match js_obj.as_string() {
        Some(cfg_str) => cfg_str,
        None => return None,
    };
    let render_cfg = match serde_json::from_str::<HashMap<String, String>>(&cfg_str) {
        Ok(render_cfg) => render_cfg,
        Err(_) => return None,
    };

    Some(render_cfg)
}

/// Gets the initial state injected by the server, if there was any. This is used to differentiate initial loads from subsequent ones,
/// which have different log chains to prevent double-trips (a common SPA problem).
pub fn get_initial_state() -> InitialState {
    let val_opt = web_sys::window().unwrap().get("__PERSEUS_INITIAL_STATE");
    let js_obj = match val_opt {
        Some(js_obj) => js_obj,
        None => return InitialState::NotPresent,
    };
    // The object should only actually contain the string value that was injected
    let state_str = match js_obj.as_string() {
        Some(state_str) => state_str,
        None => return InitialState::NotPresent,
    };
    // On the server-side, we encode a `None` value directly (otherwise it will be some convoluted stringified JSON)
    if state_str == "None" {
        InitialState::Present(None)
    } else if state_str.starts_with("error-") {
        let err_page_data_str = state_str.strip_prefix("error-").unwrap();
        // There will be error page data encoded after `error-`
        let err_page_data = match serde_json::from_str::<ErrorPageData>(err_page_data_str) {
            Ok(render_cfg) => render_cfg,
            // If there's a serialization error, we'll create a whole new error (500)
            Err(err) => ErrorPageData {
                url: "[current]".to_string(),
                status: 500,
                err: format!(
                    "couldn't serialize error from server: '{}'",
                    err.to_string()
                ),
            },
        };
        InitialState::Error(err_page_data)
    } else {
        InitialState::Present(Some(state_str))
    }
}

/// A representation of whether or not the initial state was present. If it was, it could be `None` (some templates take no state), and
/// if not, then this isn't an initial load, and we need to request the page from the server. It could also be an error that the server
/// has rendered.
pub enum InitialState {
    /// A non-error initial state has been injected.
    Present(Option<String>),
    /// An initial state ahs been injected that indicates an error.
    Error(ErrorPageData),
    /// No initial state has been injected (or if it has, it's been deliberately unset).
    NotPresent,
}

// We have to rely on JS to set a global variable unfortunately (should file an error on `web_sys` about this)
#[wasm_bindgen(module = "/src/unset_initial_state.js")]
extern "C" {
    fn unset_initial_state();
}

/// Fetches the information for the given page and renders it. This should be provided the actual path of the page to render (not just the
/// broader template). Asynchronous Wasm is handled here, because only a few cases need it.
// TODO handle exceptions higher up
pub fn app_shell(
    path: String,
    template: Template<DomNode>,
    locale: String,
    translations_manager: Rc<RefCell<ClientTranslationsManager>>,
    error_pages: Rc<ErrorPages<DomNode>>,
    container: Element,
) {
    // Check if this was an initial load and we already have the state
    let initial_state = get_initial_state();
    match initial_state {
        // If we do have an initial state, then we have everything we need for immediate hydration (no double trips)
        // The state is here, and the HTML has already been injected for us (including head metadata)
        InitialState::Present(state) => {
            // Unset the initial state variable so we perform subsequent renders correctly
            unset_initial_state();
            wasm_bindgen_futures::spawn_local(cloned!((container) => async move {
                // Now that the user can see something, we can get the translator
                let mut translations_manager_mut = translations_manager.borrow_mut();
                // This gets an `Rc<Translator>` that references the translations manager, meaning no cloning of translations
                let translator = translations_manager_mut.get_translator_for_locale(&locale).await;
                let translator = match translator {
                    Ok(translator) => translator,
                    Err(err) => {
                        // Directly eliminate the HTMl sent in from the server before we render an error page
                        container.set_inner_html("");
                        match err.kind() {
                            // These errors happen because we couldn't get a translator, so they certainly don't get one
                            ErrorKind::AssetNotOk(url, status, _) => return error_pages.render_page(url, status, &err.to_string(), None, &container),
                            ErrorKind::AssetSerFailed(url, _) => return error_pages.render_page(url, &500, &err.to_string(), None, &container),
                            ErrorKind::LocaleNotSupported(locale) => return error_pages.render_page(&format!("/{}/...", locale), &404, &err.to_string(),None,  &container),
                            // No other errors should be returned
                            _ => panic!("expected 'AssetNotOk'/'AssetSerFailed'/'LocaleNotSupported' error, found other unacceptable error")
                        }
                    }
                };

                // Hydrate that static code using the acquired state
                // BUG (Sycamore): this will double-render if the component is just text (no nodes)
                sycamore::hydrate_to(
                    // This function provides translator context as needed
                    || template.render_for_template(state, Rc::clone(&translator)),
                    &container
                );
            }));
        }
        // If we have no initial state, we should proceed as usual, fetching the content and state from the server
        InitialState::NotPresent => {
            // Spawn a Rust futures thread in the background to fetch the static HTML/JSON
            wasm_bindgen_futures::spawn_local(cloned!((container) => async move {
                // Get the static page data
                let asset_url = format!("/.perseus/page/{}/{}?template_name={}", locale, path.to_string(), template.get_path());
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
                                    container.set_inner_html(&page_data.content);

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

                                    // Render the document head
                                    let head_str = template.render_head_str(page_data.state.clone(), Rc::clone(&translator));
                                    // Get the current head
                                    let head_elem = web_sys::window()
                                        .unwrap()
                                        .document()
                                        .unwrap()
                                        .query_selector("head")
                                        .unwrap()
                                        .unwrap();
                                    let head_html = head_elem.inner_html();
                                    // We'll assume that there's already previously interpolated head in addition to the hardcoded stuff, but it will be separated by the server-injected delimiter comment
                                    // Thus, we replace the stuff after that delimiter comment with the new head
                                    let head_parts: Vec<&str> = head_html.split("<!--PERSEUS_INTERPOLATED_HEAD_BEGINS-->").collect();
                                    let new_head = format!("{}\n<!--PERSEUS_INTERPOLATED_HEAD_BEGINS-->\n{}", head_parts[0], head_str);
                                    head_elem.set_inner_html(&new_head);

                                    // Hydrate that static code using the acquired state
                                    // BUG (Sycamore): this will double-render if the component is just text (no nodes)
                                    sycamore::hydrate_to(
                                        // This function provides translator context as needed
                                        || template.render_for_template(page_data.state, Rc::clone(&translator)),
                                        &container
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
        }
        // Nothing should be done if an error was sent down
        InitialState::Error(ErrorPageData { url, status, err }) => {
            // Hydrate the currently static error page
            // Right now, we don't provide translators to any error pages that have come from the server
            error_pages.hydrate_page(&url, &status, &err, None, &container);
        }
    };
}
