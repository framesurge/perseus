use perseus::prelude::*;
use sycamore::prelude::*;

use crate::global_state::AppStateRx;

fn about_page<G: Html>(cx: Scope) -> View<G> {
    // This is not part of our data model
    let freeze_status = create_signal(cx, String::new());
    // It's faster to get this only once and rely on reactivity
    // But it's unused when this runs on the server-side because of the target-gate
    // below
    let reactor = Reactor::<G>::from_cx(cx);
    let global_state = reactor.get_global_state::<AppStateRx>(cx);

    view! { cx,
        p(id = "global_state") { (global_state.test.get()) }

        // When the user visits this and then comes back, they'll still be able to see their username (the previous state will be retrieved from the global state automatically)
        a(href = "", id = "index-link") { "Index" }
        br()

        // We'll let the user freeze from here to demonstrate that the frozen state also navigates back to the last route
        button(id = "freeze_button", on:click = move |_| {
            // The IndexedDB API is asynchronous, so we'll spawn a future
            #[cfg(target_arch = "wasm32")]
            perseus::spawn_local_scoped(cx, async move {
                use perseus::state::{IdbFrozenStateStore, Freeze};
                // We do this here (rather than when we get the render context) so that it's updated whenever we press the button
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
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").view(about_page).build()
}
