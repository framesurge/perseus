use perseus::t;
use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, KeyboardEvent};

#[component]
pub fn SearchBar<G: Html>(cx: Scope) -> View<G> {
    let search = create_signal(cx, String::new());

    view! { cx,
        input(
            class = "p-2 border rounded-md mb-2 focus:outline-indigo-500 search-bar-bg max-w-full",
            placeholder = t!("search", cx),
            bind:value = search,
            // When the user presses enter, we should submit their search to Google in a new tab
            on:keyup = move |ev: Event| {
                let event: KeyboardEvent = ev.unchecked_into();
                if event.key() == "Enter" {
                    let search = search.get();
                    if !search.is_empty() {
                        #[cfg(target_arch = "wasm32")]
                        search_site(&search);
                    }
                }
            }
        )
    }
}

/// Searches the site using Google as a proxy.
// BUG This should be Framesurge instead, but search engines have been slow to index...
#[cfg(target_arch = "wasm32")]
fn search_site(search: &str) {
    use js_sys::encode_uri_component;

    let search_query = format!("site:arctic-hen7.github.io/perseus/en-US/docs {}", search);
    let search_query = encode_uri_component(&search_query).to_string();
    let search_url = format!("https://google.com/search?q={}", search_query);
    // Open that in a new tab
    let window = web_sys::window().unwrap();
    window
        .open_with_url_and_target(&search_url, "_blank")
        .unwrap();
}
