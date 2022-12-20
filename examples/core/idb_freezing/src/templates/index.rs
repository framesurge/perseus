use crate::global_state::AppStateRx;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "IndexPropsRx")]
struct IndexProps {
    username: String,
}

fn index_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a IndexPropsRx) -> View<G> {
    // This is not part of our data model
    let freeze_status = create_signal(cx, String::new());
    let thaw_status = create_signal(cx, String::new());
    // It's faster to get this only once and rely on reactivity
    // But it's unused when this runs on the server-side because of the target-gate
    // below
    let reactor = Reactor::<G>::from_cx(cx);
    let global_state = reactor.get_global_state::<AppStateRx>(cx);

    view! { cx,
        // For demonstration, we'll let the user modify the page's state and the global state arbitrarily
        p(id = "page_state") { (format!("Greetings, {}!", state.username.get())) }
        input(id = "set_page_state", bind:value = state.username, placeholder = "Username")
        p(id = "global_state") { (global_state.test.get()) }
        input(id = "set_global_state", bind:value = global_state.test, placeholder = "Global state")

        // When the user visits this and then comes back, they'll still be able to see their username (the previous state will be retrieved from the global state automatically)
        a(href = "about", id = "about-link") { "About" }
        br()

        button(id = "freeze_button", on:click = move |_| {
            // The IndexedDB API is asynchronous, so we'll spawn a future
            #[cfg(target_arch = "wasm32")] // The freezing types are only available in the browser
            perseus::spawn_local_scoped(cx, async {
                use perseus::state::{IdbFrozenStateStore, Freeze, PageThawPrefs, ThawPrefs};
                // We do this here (rather than when we get the reactor) so that it's updated whenever we press the button
                let frozen_state = reactor.freeze();
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
        }) { "Freeze to IndexedDB" }
        p { (freeze_status.get()) }

        button(id = "thaw_button", on:click = move |_| {
            // The IndexedDB API is asynchronous, so we'll spawn a future
            #[cfg(target_arch = "wasm32")] // The freezing types are only available in the browser
            perseus::spawn_local_scoped(cx, async move {
                use perseus::state::{IdbFrozenStateStore, Freeze, PageThawPrefs, ThawPrefs};
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
                match reactor.thaw(&frozen_state, ThawPrefs { page: PageThawPrefs::IncludeAll, global_prefer_frozen: true }) {
                    Ok(_) => thaw_status.set("Thawed.".to_string()),
                    Err(_) => thaw_status.set("Error.".to_string())
                }
            })
        }) { "Thaw from IndexedDB" }
        p { (thaw_status.get()) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .build()
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> IndexProps {
    IndexProps {
        username: "".to_string(),
    }
}
