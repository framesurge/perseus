use perseus::{Html, RenderFnResultWithCause, Template};
use sycamore::prelude::*;

#[perseus::make_rx(IndexPageStateRx)]
pub struct IndexPageState {
    server_ip: String,
    browser_ip: Option<String>,
}

#[perseus::template_rx(IndexPage)]
pub fn index_page(
    IndexPageStateRx {
        server_ip,
        browser_ip,
    }: IndexPageStateRx,
) -> View<G> {
    // This will only run in the browser
    // `reqwasm` wraps browser-specific APIs, so we don't want it running on the server
    // If the browser IP has already been fetched (e.g. if we've come here for the second time in the same session), we won't bother re-fetching
    if G::IS_BROWSER && browser_ip.get().is_none() {
        // Spawn a `Future` on this thread to fetch the data (`spawn_local` is re-exported from `wasm-bindgen-futures`)
        // Don't worry, this doesn't need to be sent to JavaScript for execution
        //
        // We want to access the `message` `Signal`, so we'll clone it in (and then we need `move` because this has to be `'static`)
        perseus::spawn_local(cloned!(browser_ip => async move {
            // This interface may seem weird, that's because it wraps the browser's Fetch API
            // We request from a local path here because of CORS restrictions (see the book)
            let body = reqwasm::http::Request::get("/.perseus/static/message.txt")
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            browser_ip.set(Some(body));
        }));
    }

    // If the future hasn't finished yet, we'll display a placeholder
    // We use the wacky `&*` syntax to get the content of the `browser_ip` `Signal` and then we tell Rust to take a reference to that (we can't move it out because it might be used later)
    let browser_ip_display = match &*browser_ip.get() {
        Some(ip) => ip.to_string(),
        None => "fetching".to_string(),
    };

    view! {
        p { (format!("IP address of the server was: {}", server_ip.get())) }
        p { (format!("The message is: {}", browser_ip_display)) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_build_state)
        .template(index_page)
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<IndexPageState> {
    // We'll cache the result with `try_cache_res`, which means we only make the request once, and future builds will use the cached result (speeds up development)
    let body = perseus::cache_fallible_res(
        "ipify",
        || async {
            // This just gets the IP address of the machine that built the app
            let res = ureq::get("https://api.ipify.org").call()?.into_string()?;
            Ok::<String, ureq::Error>(res)
        },
        false,
    )
    .await?;
    Ok(IndexPageState {
        server_ip: body,
        browser_ip: None,
    })
}
