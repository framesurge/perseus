use crate::error_pages::ErrorPageData;
use crate::errors::*;
use crate::i18n::ClientTranslationsManager;
use crate::router::{RouteVerdict, RouterLoadState, RouterState};
use crate::server::PageData;
use crate::state::PageStateStore;
use crate::state::{FrozenApp, GlobalState, ThawPrefs};
use crate::template::{PageProps, Template, TemplateNodeType};
use crate::utils::get_path_prefix_client;
use crate::ErrorPages;
use fmterr::fmt_err;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use sycamore::prelude::*;
use sycamore::rt::Reflect; // We can piggyback off Sycamore to avoid bringing in `js_sys`
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Element, Request, RequestInit, RequestMode, Response};

/// Fetches the given resource. This should NOT be used by end users, but it's required by the CLI.
#[doc(hidden)]
pub async fn fetch(url: &str) -> Result<Option<String>, ClientError> {
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
        // We strip the prefix and escape any tab/newline control characters (inserted by `fmterr`)
        // Any others are user-inserted, and this is documented
        let err_page_data_str = state_str
            .strip_prefix("error-")
            .unwrap()
            .replace("\n", "\\n")
            .replace("\t", "\\t");
        // There will be error page data encoded after `error-`
        let err_page_data = match serde_json::from_str::<ErrorPageData>(&err_page_data_str) {
            Ok(render_cfg) => render_cfg,
            // If there's a serialization error, we'll create a whole new error (500)
            Err(err) => ErrorPageData {
                url: "[current]".to_string(),
                status: 500,
                err: format!("couldn't serialize error from server: '{}'", err),
            },
        };
        InitialState::Error(err_page_data)
    } else {
        InitialState::Present(Some(state_str))
    }
}

/// Gets the global state injected by the server, if there was any. If there are errors in this, we can return `None` and not worry about it, they'll be handled by the initial state.
pub fn get_global_state() -> Option<String> {
    let val_opt = web_sys::window().unwrap().get("__PERSEUS_GLOBAL_STATE");
    let js_obj = match val_opt {
        Some(js_obj) => js_obj,
        None => return None,
    };
    // The object should only actually contain the string value that was injected
    let state_str = match js_obj.as_string() {
        Some(state_str) => state_str,
        None => return None,
    };
    // On the server-side, we encode a `None` value directly (otherwise it will be some convoluted stringified JSON)
    match state_str.as_str() {
        "None" => None,
        state_str => Some(state_str.to_string()),
    }
}

/// Marks a checkpoint in the code and alerts any tests that it's been reached by creating an element that represents it. The preferred
/// solution would be emitting a DOM event, but the WebDriver specification currently doesn't support waiting on those (go figure). This
/// will only create a custom element if the `__PERSEUS_TESTING` JS global variable is set to `true`.
///
/// This adds a `<div id="__perseus_checkpoint-<event-name>" />` to the `<div id="__perseus_checkpoints"></div>` element, creating the
/// latter if it doesn't exist. Each checkpoint must have a unique name, and if the same checkpoint is executed twice, it'll be added
/// with a `-<number>` after it, starting from `0`. In this way, we have a functional checkpoints queue for signalling to test code!
/// Note that the checkpoint queue is NOT cleared on subsequent loads.
///
/// Note: this is not just for internal usage, it's highly recommended that you use this for your own checkpoints as well! Just make
/// sure your tests don't conflict with any internal Perseus checkpoint names (preferably prefix yours with `custom-` or the like, as
/// Perseus' checkpoints may change at any time, but won't ever use that namespace).
///
/// WARNING: your checkpoint names must not include hyphens! This will result in a `panic!`.
pub fn checkpoint(name: &str) {
    if name.contains('-') {
        panic!("checkpoint must not contain hyphens, use underscores instead (hyphens are used as an internal delimiter)");
    }

    let val_opt = web_sys::window().unwrap().get("__PERSEUS_TESTING");
    let js_obj = match val_opt {
        Some(js_obj) => js_obj,
        None => return,
    };
    // The object should only actually contain the string value that was injected
    let is_testing = match js_obj.as_bool() {
        Some(cfg_str) => cfg_str,
        None => return,
    };
    if !is_testing {
        return;
    }

    // If we're here, we're testing
    // We dispatch a console warning to reduce the likelihood of literal 'testing in prod'
    crate::web_log!("Perseus is in testing mode. If you're an end-user and seeing this message, please report this as a bug to the website owners!");
    // Create a custom element that can be waited for by the WebDriver
    // This will be removed by the next checkpoint
    let document = web_sys::window().unwrap().document().unwrap();
    let container_opt = document.query_selector("#__perseus_checkpoints").unwrap();
    let container = match container_opt {
        Some(container_i) => container_i,
        None => {
            // If the container doesn't exist yet, create it
            let container = document.create_element("div").unwrap();
            container.set_id("__perseus_checkpoints");
            document
                .query_selector("body")
                .unwrap()
                .unwrap()
                .append_with_node_1(&container)
                .unwrap();
            container
        }
    };

    // Get the number of checkpoints that already exist with the same ID
    // We prevent having to worry about checkpoints whose names are subsets of others by using the hyphen as a delimiter
    let num_checkpoints = document
        .query_selector_all(&format!("[id^=__perseus_checkpoint-{}-]", name))
        .unwrap()
        .length();
    // Append the new checkpoint
    let checkpoint = document.create_element("div").unwrap();
    checkpoint.set_id(&format!(
        "__perseus_checkpoint-{}-{}",
        name, num_checkpoints
    ));
    container.append_with_node_1(&checkpoint).unwrap();
}

/// A representation of whether or not the initial state was present. If it was, it could be `None` (some templates take no state), and
/// if not, then this isn't an initial load, and we need to request the page from the server. It could also be an error that the server
/// has rendered.
#[derive(Debug)]
pub enum InitialState {
    /// A non-error initial state has been injected.
    Present(Option<String>),
    /// An initial state ahs been injected that indicates an error.
    Error(ErrorPageData),
    /// No initial state has been injected (or if it has, it's been deliberately unset).
    NotPresent,
}

/// Properties for the app shell. These should be constructed literally when working with the app shell.
#[derive(Debug)]
pub struct ShellProps {
    /// The path we're rendering for (not the template path, the full path, though parsed a little).
    pub path: String,
    /// The template to render for.
    pub template: Rc<Template<TemplateNodeType>>,
    /// Whether or not the router returned an incremental match (if this page exists on a template using incremental generation and it wasn't defined at build time).
    pub was_incremental_match: bool,
    /// The locale we're rendering in.
    pub locale: String,
    /// The router state.
    pub router_state: RouterState,
    /// The template state store.
    pub page_state_store: PageStateStore,
    /// A *client-side* translations manager to use (this manages caching translations).
    pub translations_manager: Rc<RefCell<ClientTranslationsManager>>,
    /// The error pages, for use if something fails.
    pub error_pages: Rc<ErrorPages<DomNode>>,
    /// The container responsible for the initial render from the server (non-interactive, this may need to be wiped).
    pub initial_container: Element,
    /// The container for reactive content.
    pub container_rx_elem: Element,
    /// The global state store. Brekaing it out here prevents it being overriden every time a new template loads.
    pub global_state: GlobalState,
    /// A previous frozen state to be gradully rehydrated. This should always be `None`, it only serves to provide continuity across templates.
    pub frozen_app: Rc<RefCell<Option<(FrozenApp, ThawPrefs)>>>,
    /// The current route verdict. This will be stored in context so that it can be used for possible reloads. Eventually,
    /// this will be made obsolete when Sycamore supports this natively.
    pub route_verdict: RouteVerdict<TemplateNodeType>,
    /// Whether or not this page is the very first to have been rendered since the browser loaded the app.
    pub is_first: Rc<Cell<bool>>,
    #[cfg(all(feature = "live-reload", debug_assertions))]
    /// An indicator `Signal` used to allow the root to instruct the app that we're about to reload because of an instruction from the live reloading server.
    pub live_reload_indicator: RcSignal<bool>,
}

/// Fetches the information for the given page and renders it. This should be provided the actual path of the page to render (not just the
/// broader template). Asynchronous Wasm is handled here, because only a few cases need it.
// TODO handle exceptions higher up
pub async fn app_shell(
    ShellProps {
        path,
        template,
        was_incremental_match,
        locale,
        mut router_state,
        page_state_store,
        translations_manager,
        error_pages,
        initial_container,
        container_rx_elem,
        global_state: curr_global_state,
        frozen_app,
        route_verdict,
        is_first,
        #[cfg(all(feature = "live-reload", debug_assertions))]
        live_reload_indicator,
    }: ShellProps,
) {
    checkpoint("app_shell_entry");
    let path_with_locale = match locale.as_str() {
        "xx-XX" => path.clone(),
        locale => format!("{}/{}", locale, &path),
    };
    // Update the router state
    router_state.set_load_state(RouterLoadState::Loading {
        template_name: template.get_path(),
        path: path_with_locale.clone(),
    });
    router_state.set_last_verdict(route_verdict);
    // Get the global state if possible (we'll want this in all cases except errors)
    // If this is a subsequent load, the template macro will have already set up the global state, and it will ignore whatever we naively give it (so we'll give it `None`)
    let global_state = get_global_state();
    // Check if this was an initial load and we already have the state
    let initial_state = get_initial_state();
    match initial_state {
        // If we do have an initial state, then we have everything we need for immediate hydration (no double trips)
        // The state is here, and the HTML has already been injected for us (including head metadata)
        InitialState::Present(state) => {
            checkpoint("initial_state_present");
            // Unset the initial state variable so we perform subsequent renders correctly
            // This monstrosity is needed until `web-sys` adds a `.set()` method on `Window`
            // We don't do this for the global state because it should hang around uninitialized until a template wants it (if we remove it before then, we're stuffed)
            Reflect::set(
                &JsValue::from(web_sys::window().unwrap()),
                &JsValue::from("__PERSEUS_INITIAL_STATE"),
                &JsValue::undefined(),
            )
            .unwrap();
            // We need to move the server-rendered content from its current container to the reactive container (otherwise Sycamore can't work with it properly)
            let initial_html = initial_container.inner_html();
            container_rx_elem.set_inner_html(&initial_html);
            initial_container.set_inner_html("");
            // Make the initial container invisible
            initial_container
                .set_attribute("style", "display: none;")
                .unwrap();
            checkpoint("page_visible");

            // Now that the user can see something, we can get the translator
            let mut translations_manager_mut = translations_manager.borrow_mut();
            // This gets an `Rc<Translator>` that references the translations manager, meaning no cloning of translations
            let translator = translations_manager_mut
                .get_translator_for_locale(&locale)
                .await;
            let translator = match translator {
                Ok(translator) => translator,
                Err(err) => {
                    // Directly eliminate the HTML sent in from the server before we render an error page
                    container_rx_elem.set_inner_html("");
                    match &err {
                        // These errors happen because we couldn't get a translator, so they certainly don't get one
                        ClientError::FetchError(FetchError::NotOk { url, status, .. }) => return error_pages.render_page(url, *status, &fmt_err(&err), None, &container_rx_elem),
                        ClientError::FetchError(FetchError::SerFailed { url, .. }) => return error_pages.render_page(url, 500, &fmt_err(&err), None, &container_rx_elem),
                        ClientError::LocaleNotSupported { .. } => return error_pages.render_page(&format!("/{}/...", locale), 404, &fmt_err(&err), None, &container_rx_elem),
                        // No other errors should be returned
                        _ => panic!("expected 'AssetNotOk'/'AssetSerFailed'/'LocaleNotSupported' error, found other unacceptable error")
                    }
                }
            };

            let path = template.get_path();
            // Hydrate that static code using the acquired state
            let router_state_2 = router_state.clone();
            // BUG (Sycamore): this will double-render if the component is just text (no nodes)
            let page_props = PageProps {
                path: path_with_locale.clone(),
                state,
                global_state,
            };
            #[cfg(not(feature = "hydrate"))]
            {
                // If we aren't hydrating, we'll have to delete everything and re-render
                container_rx_elem.set_inner_html("");
                sycamore::render_to(
                    move |cx| {
                        template.render_for_template_client(
                            page_props,
                            cx,
                            translator,
                            false,
                            router_state_2,
                            page_state_store,
                            curr_global_state,
                            frozen_app,
                            is_first,
                            #[cfg(all(feature = "live-reload", debug_assertions))]
                            live_reload_indicator,
                        )
                    },
                    &container_rx_elem,
                );
            }
            #[cfg(feature = "hydrate")]
            sycamore::hydrate_to(
                // This function provides translator context as needed
                |cx| {
                    template.render_for_template_client(
                        page_props,
                        cx,
                        translator,
                        false,
                        router_state_2,
                        page_state_store,
                        curr_global_state,
                        frozen_app,
                        is_first,
                        #[cfg(all(feature = "live-reload", debug_assertions))]
                        live_reload_indicator,
                    )
                },
                &container_rx_elem,
            );
            checkpoint("page_interactive");
            // Update the router state
            router_state.set_load_state(RouterLoadState::Loaded {
                template_name: path,
                path: path_with_locale,
            });
        }
        // If we have no initial state, we should proceed as usual, fetching the content and state from the server
        InitialState::NotPresent => {
            checkpoint("initial_state_not_present");
            // If we're getting data about the index page, explicitly set it to that
            // This can be handled by the Perseus server (and is), but not by static exporting
            let path = match path.is_empty() {
                true => "index".to_string(),
                false => path,
            };
            // Get the static page data
            let asset_url = format!(
                "{}/.perseus/page/{}/{}.json?template_name={}&was_incremental_match={}",
                get_path_prefix_client(),
                locale,
                path,
                template.get_path(),
                was_incremental_match
            );
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
                                container_rx_elem.set_inner_html(&page_data.content);
                                // Interpolate the metadata directly into the document's `<head>`
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
                                let head_parts: Vec<&str> = head_html
                                    .split("<!--PERSEUS_INTERPOLATED_HEAD_BEGINS-->")
                                    .collect();
                                let new_head = format!(
                                    "{}\n<!--PERSEUS_INTERPOLATED_HEAD_BEGINS-->\n{}",
                                    head_parts[0], &page_data.head
                                );
                                head_elem.set_inner_html(&new_head);
                                checkpoint("page_visible");

                                // Now that the user can see something, we can get the translator
                                let mut translations_manager_mut =
                                    translations_manager.borrow_mut();
                                // This gets an `Rc<Translator>` that references the translations manager, meaning no cloning of translations
                                let translator = translations_manager_mut
                                    .get_translator_for_locale(&locale)
                                    .await;
                                let translator = match translator {
                                    Ok(translator) => translator,
                                    Err(err) => match &err {
                                        // These errors happen because we couldn't get a translator, so they certainly don't get one
                                        ClientError::FetchError(FetchError::NotOk { url, status, .. }) => return error_pages.render_page(url, *status, &fmt_err(&err), None, &container_rx_elem),
                                        ClientError::FetchError(FetchError::SerFailed { url, .. }) => return error_pages.render_page(url, 500, &fmt_err(&err), None, &container_rx_elem),
                                        ClientError::LocaleNotSupported { locale } => return error_pages.render_page(&format!("/{}/...", locale), 404, &fmt_err(&err), None, &container_rx_elem),
                                        // No other errors should be returned
                                        _ => panic!("expected 'AssetNotOk'/'AssetSerFailed'/'LocaleNotSupported' error, found other unacceptable error")
                                    }
                                };

                                // Hydrate that static code using the acquired state
                                let router_state_2 = router_state.clone();
                                // BUG (Sycamore): this will double-render if the component is just text (no nodes)
                                let page_props = PageProps {
                                    path: path_with_locale.clone(),
                                    state: page_data.state,
                                    global_state,
                                };
                                let template_name = template.get_path();
                                #[cfg(not(feature = "hydrate"))]
                                {
                                    // If we aren't hydrating, we'll have to delete everything and re-render
                                    container_rx_elem.set_inner_html("");
                                    sycamore::render_to(
                                        move |cx| {
                                            template.render_for_template_client(
                                                page_props,
                                                cx,
                                                translator,
                                                false,
                                                router_state_2.clone(),
                                                page_state_store,
                                                curr_global_state,
                                                frozen_app,
                                                is_first,
                                                #[cfg(all(
                                                    feature = "live-reload",
                                                    debug_assertions
                                                ))]
                                                live_reload_indicator,
                                            )
                                        },
                                        &container_rx_elem,
                                    );
                                }
                                #[cfg(feature = "hydrate")]
                                sycamore::hydrate_to(
                                    // This function provides translator context as needed
                                    move |cx| {
                                        template.render_for_template_client(
                                            page_props,
                                            cx,
                                            translator,
                                            false,
                                            router_state_2,
                                            page_state_store,
                                            curr_global_state,
                                            frozen_app,
                                            is_first,
                                            #[cfg(all(feature = "live-reload", debug_assertions))]
                                            live_reload_indicator,
                                        )
                                    },
                                    &container_rx_elem,
                                );
                                checkpoint("page_interactive");
                                // Update the router state
                                router_state.set_load_state(RouterLoadState::Loaded {
                                    template_name,
                                    path: path_with_locale,
                                });
                            }
                            // If the page failed to serialize, an exception has occurred
                            Err(err) => panic!("page data couldn't be serialized: '{}'", err),
                        };
                    }
                    // No translators ready yet
                    None => error_pages.render_page(
                        &asset_url,
                        404,
                        "page not found",
                        None,
                        &container_rx_elem,
                    ),
                },
                Err(err) => match &err {
                    // No translators ready yet
                    ClientError::FetchError(FetchError::NotOk { url, status, .. }) => error_pages
                        .render_page(url, *status, &fmt_err(&err), None, &container_rx_elem),
                    // No other errors should be returned
                    _ => panic!("expected 'AssetNotOk' error, found other unacceptable error"),
                },
            };
        }
        // Nothing should be done if an error was sent down
        InitialState::Error(ErrorPageData { url, status, err }) => {
            checkpoint("initial_state_error");
            // We need to move the server-rendered content from its current container to the reactive container (otherwise Sycamore can't work with it properly)
            // If we're not hydrating, there's no point in moving anything over, we'll just fully re-render
            #[cfg(feature = "hydrate")]
            {
                let initial_html = initial_container.inner_html();
                container_rx_elem.set_inner_html(&initial_html);
            }
            initial_container.set_inner_html("");
            // Make the initial container invisible
            initial_container
                .set_attribute("style", "display: none;")
                .unwrap();
            // Hydrate the currently static error page
            // Right now, we don't provide translators to any error pages that have come from the server
            // We render this rather than hydrating because otherwise we'd need a `HydrateNode` at the plugins level, which is way too inefficient
            #[cfg(not(feature = "hydrate"))]
            container_rx_elem.set_inner_html("");
            error_pages.render_page(&url, status, &err, None, &container_rx_elem);
        }
    };
}
