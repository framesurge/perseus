use perseus::{Html, RenderFnResultWithCause, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct IndexProps {
    ip: String,
}

#[perseus::template(IndexPage)]
#[component(IndexPage<G>)]
pub fn index_page(IndexProps { ip }: IndexProps) -> View<G> {
    // This will store the message that we get
    // Until we've got it, we'll display `fetching...`
    let message = Signal::new("fetching...".to_string());

    // This will only run in the browser
    // `reqwasm` wraps browser-specific APIs, so we don't want it running on the server
    if G::IS_BROWSER {
        // Spawn a `Future` on this thread to fetch the data (`spawn_local` is re-exported from `wasm-bindgen-futures`)
        // Don't worry, this doesn't need to be sent to JavaScript for execution
        //
        // We want to access the `message` `Signal`, so we'll clone it in (and then we need `move` because this has to be `'static`)
        perseus::spawn_local(cloned!(message => async move {
            // This interface may seem weird, that's because it wraps the browser's Fetch API
            // We request from a local path here because of CORS restrictions (see the book)
            let body = reqwasm::http::Request::get("/message")
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            message.set(body);
        }));
    }

    view! {
        p { (format!("IP address of the builder was: {}", ip)) }
        p { (format!("The message at `/message` is: {}", message.get())) }
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
) -> RenderFnResultWithCause<IndexProps> {
    // This just gets the IP address of the machine that built the app
    let body: String = ureq::get("https://api.ipify.org").call()?.into_string()?;
    Ok(IndexProps { ip: body })
}
