use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "IndexPageStateRx")]
struct IndexPageState {
    server_ip: String,
    browser_ip: Option<String>,
}

#[perseus::template]
fn index_page<'a, G: Html>(
    cx: Scope<'a>,
    IndexPageStateRx {
        server_ip,
        browser_ip,
    }: IndexPageStateRx<'a>,
) -> View<G> {
    // This will only run in the browser
    // `reqwasm` wraps browser-specific APIs, so we don't want it running on the
    // server If the browser IP has already been fetched (e.g. if we've come
    // here for the second time in the same session), we won't bother re-fetching
    #[cfg(target_arch = "wasm32")]
    // Because we only have `reqwasm` on the client-side, we make sure this is only *compiled* in
    // the browser as well
    if browser_ip.get().is_none() {
        // Spawn a `Future` on this thread to fetch the data (`spawn_local` is
        // re-exported from `wasm-bindgen-futures`) Don't worry, this doesn't
        // need to be sent to JavaScript for execution
        //
        // We want to access the `message` `Signal`, so we'll clone it in (and then we
        // need `move` because this has to be `'static`)
        perseus::spawn_local_scoped(cx, async {
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
    // We use the wacky `&*` syntax to get the content of the `browser_ip` `Signal`
    // and then we tell Rust to take a reference to that (we can't move it out
    // because it might be used later)
    let browser_ip_display = create_memo(cx, || match &*browser_ip.get() {
        Some(ip) => ip.to_string(),
        None => "fetching".to_string(),
    });

    view! { cx,
        p { (format!("IP address of the server was: {}", server_ip.get())) }
        p { (format!("The message is: {}", browser_ip_display)) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_build_state)
        .template_with_state(index_page)
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> RenderFnResultWithCause<IndexPageState> {
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
    .await?;

    Ok(IndexPageState {
        server_ip: body,
        browser_ip: None,
    })
}
