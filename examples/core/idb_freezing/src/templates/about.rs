use perseus::state::{Freeze, IdbFrozenStateStore};
use perseus::{Html, Template};
use sycamore::prelude::*;

use crate::global_state::AppStateRx;

#[perseus::template_rx]
pub fn about_page(_: (), global_state: AppStateRx) -> View<G> {
    let test = global_state.test;
    // This is not part of our data model
    let freeze_status = Signal::new(String::new());
    let render_ctx = perseus::get_render_ctx!();

    view! {
        p(id = "global_state") { (test.get()) }

        // When the user visits this and then comes back, they'll still be able to see their username (the previous state will be retrieved from the global state automatically)
        a(href = "", id = "index-link") { "Index" }
        br()

        // We'll let the user freeze from here to demonstrate that the frozen state also navigates back to the last route
        button(id = "freeze_button", on:click = cloned!(freeze_status, render_ctx => move |_|
            // The IndexedDB API is asynchronous, so we'll spawn a future
            wasm_bindgen_futures::spawn_local(cloned!(render_ctx, freeze_status => async move {
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
            }))
        )) { "Freeze to IndexedDB" }
        p { (freeze_status.get()) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page)
}
