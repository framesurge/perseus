use perseus::{Html, RenderFnResultWithCause, Template};
use sycamore::prelude::*;

#[perseus::make_rx(IndexPageStateRx)]
pub struct IndexPageState {
    server_ip: String,
    browser_ip: String,
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
    if G::IS_BROWSER {
        // Spawn a `Future` on this thread to fetch the data (`spawn_local` is re-exported from `wasm-bindgen-futures`)
        // Don't worry, this doesn't need to be sent to JavaScript for execution
        //
        // We want to access the `message` `Signal`, so we'll clone it in (and then we need `move` because this has to be `'static`)
        perseus::spawn_local(cloned!(browser_ip => async move {
            // This interface may seem weird, that's because it wraps the browser's Fetch API
            // We request from a local path here because of CORS restrictions (see the book)
            let body = reqwasm::http::Request::get("/message")
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            browser_ip.set(body);
        }));
    }

    view! {
        p { (format!("IP address of the server was: {}", server_ip.get())) }
        p { (format!("The message at `/message` is: {}", browser_ip.get())) }
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
    // We'll start with a placeholder for the browser's IP, which will be fetched on the client-side
    Ok(IndexPageState {
        server_ip: body,
        browser_ip: "fetching...".to_string(),
    })
}
