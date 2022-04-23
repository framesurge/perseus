use perseus::state::{Freeze, IdbFrozenStateStore, PageThawPrefs, ThawPrefs};
use perseus::{Html, RenderFnResultWithCause, Template};
use sycamore::prelude::*;

use crate::global_state::AppStateRx;

#[perseus::make_rx(IndexPropsRx)]
pub struct IndexProps {
    username: String,
}

#[perseus::template_rx]
pub fn index_page<G: Html>(cx: Scope, state: IndexPropsRx, global_state: AppStateRx) -> View<G> {
    let username = state.username;
    let username_2 = username.clone(); // This is necessary until Sycamore's new reactive primitives are released
    let test = global_state.test;
    let test_2 = test.clone();

    // This is not part of our data model
    let freeze_status = create_rc_signal(String::new());
    let thaw_status = create_rc_signal(String::new());
    let render_ctx = perseus::get_render_ctx!(cx);

    view! { cx,
        // For demonstration, we'll let the user modify the page's state and the global state arbitrarily
        p(id = "page_state") { (format!("Greetings, {}!", username.get())) }
        input(id = "set_page_state", bind:value = username_2, placeholder = "Username")
        p(id = "global_state") { (test.get()) }
        input(id = "set_global_state", bind:value = test_2, placeholder = "Global state")

        // When the user visits this and then comes back, they'll still be able to see their username (the previous state will be retrieved from the global state automatically)
        a(href = "about", id = "about-link") { "About" }
        br()

        button(id = "freeze_button", on:click = move |_|
            // The IndexedDB API is asynchronous, so we'll spawn a future
            wasm_bindgen_futures::spawn_local(async move {
                // We do this here (rather than when we get the render context) so that it's updated whenever we press the button
                let frozen_state = render_ctx.freeze();
                let idb_store = match IdbFrozenStateStore::new().await {
                    Ok(idb_store) => idb_store,
                    Err(_) => {
                        freeze_status.set("Error.".to_string());
                        return;
                    }
                };
                match idb_store.set(&frozen_state).await {
                    Ok(_) => freeze_status.set("Saved.".to_string()),
                    Err(_) => freeze_status.set("Error.".to_string())
                };
            })
        ) { "Freeze to IndexedDB" }
        p { (freeze_status.get()) }

        button(id = "thaw_button", on:click = move |_|
            // The IndexedDB API is asynchronous, so we'll spawn a future
            wasm_bindgen_futures::spawn_local(async move {
                let idb_store = match IdbFrozenStateStore::new().await {
                    Ok(idb_store) => idb_store,
                    Err(_) => {
                        thaw_status.set("Error.".to_string());
                        return;
                    }
                };
                let frozen_state = match idb_store.get().await {
                    Ok(Some(frozen_state)) => frozen_state,
                    Ok(None) => {
                        thaw_status.set("No state stored.".to_string());
                        return;
                    }
                    Err(_) => {
                        thaw_status.set("Error.".to_string());
                        return;
                    }
                };

                // You would probably set your thawing preferences differently
                match render_ctx.thaw(&frozen_state, ThawPrefs { page: PageThawPrefs::IncludeAll, global_prefer_frozen: true }) {
                    Ok(_) => thaw_status.set("Thawed.".to_string()),
                    Err(_) => thaw_status.set("Error.".to_string())
                }
            })
        ) { "Thaw from IndexedDB" }
        p { (thaw_status.get()) }
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
    Ok(IndexProps {
        username: "".to_string(),
    })
}
