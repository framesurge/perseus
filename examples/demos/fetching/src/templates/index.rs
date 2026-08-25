use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "IndexPageStateRx")]
struct IndexPageState {
    server_ip: String,
    browser_ip: Option<String>,
}

fn index_page(
    IndexPageStateRx {
        server_ip,
        browser_ip,
    }: IndexPageStateRx,
) -> View {
    // This will only run in the browser
    // `reqwasm` wraps browser-specific APIs, so we don't want it running on the
    // server If the browser IP has already been fetched (e.g. if we've come
    // here for the second time in the same session), we won't bother re-fetching
    #[cfg(client)]
    // Because we only have `reqwasm` on the client-side, we make sure this is only *compiled* in
    // the browser as well
    // In Sycamore 0.9.2, use get_clone() for non-Copy types
    if browser_ip.get_clone().is_none() {
        // Spawn a `Future` on this thread to fetch the data (`spawn_local` is
        // re-exported from `wasm-bindgen-futures`) Don't worry, this doesn't
        // need to be sent to JavaScript for execution
        //
        // We want to access the `message` `Signal`, so we'll clone it in (and then we
        // need `move` because this has to be `'static`)
        spawn_local_scoped(async move {
            // This interface may seem weird, that's because it wraps the browser's Fetch
            // API We request from a local path here because of CORS
            // restrictions (see the book)
            let body = reqwasm::http::Request::get("/.perseus/static/message.txt")
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            browser_ip.set(Some(body));
        });
    }

    // If the future hasn't finished yet, we'll display a placeholder
    let browser_ip_display = create_memo(move || match browser_ip.get_clone().as_ref() {
        Some(ip) => ip.to_string(),
        None => "fetching".to_string(),
    });

    view! {
        p { (format!("IP address of the server was: {}", server_ip.get_clone())) }
        p { (format!("The message is: {}", browser_ip_display)) }
    }
}

pub fn get_template() -> Template {
    Template::build("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .build()
}

#[engine_only_fn]
async fn get_build_state(
    _info: StateGeneratorInfo<()>,
) -> Result<IndexPageState, BlamedError<reqwest::Error>> {
    // We'll cache the result with `try_cache_res`, which means we only make the
    // request once, and future builds will use the cached result (speeds up
    // development)
    let body = perseus::utils::cache_fallible_res(
        "ipify",
        || async {
            // This just gets the IP address of the machine that built the app
            let res = reqwest::get("https://api.ipify.org").await?.text().await?;
            Ok::<String, reqwest::Error>(res)
        },
        false,
    )
    .await?; // Note that `?` is able to convert from `reqwest::Error` ->
             // `BlamedError<reqwest::Error>`

    Ok(IndexPageState {
        server_ip: body,
        browser_ip: None,
    })
}
