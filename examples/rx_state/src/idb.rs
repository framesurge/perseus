use perseus::state::{Freeze, IdbFrozenStateStore, PageThawPrefs, ThawPrefs};
use perseus::{Html, RenderFnResultWithCause, Template};
use sycamore::prelude::*;

#[perseus::make_rx(TestPropsRx)]
pub struct TestProps {
    pub username: String,
}

// This special macro (normally we'd use `template(IndexProps)`) converts the state we generate elsewhere to a reactive version
#[perseus::template_rx(IdbPage)]
pub fn idb_page(TestPropsRx { username }: TestPropsRx) -> View<G> {
    let username_2 = username.clone(); // This is necessary until Sycamore's new reactive primitives are released
    let render_ctx = perseus::get_render_ctx!(); // We get the render context out here, it's not accessible in the future
    let freeze_status = Signal::new(String::new()); // This isn't part of the template's data model, it's just here for demonstration
    let thaw_status = Signal::new(String::new());

    view! {
        p { (format!("Greetings, {}!", username.get())) }
        input(bind:value = username_2, placeholder = "Username")

        // When the user visits this and then comes back, they'll still be able to see their username (the previous state will be retrieved from the global state automatically)
        a(href = "about") { "About" }
        a(href = "") { "Index" }

        button(on:click = cloned!(freeze_status, render_ctx => move |_|
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

        button(on:click = cloned!(thaw_status, render_ctx => move |_|
            // The IndexedDB API is asynchronous, so we'll spawn a future
            wasm_bindgen_futures::spawn_local(cloned!(render_ctx, thaw_status => async move {
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
            }))
        )) { "Thaw from IndexedDB" }
        p { (thaw_status.get()) }

        // button(on:click = cloned!(frozen_app, render_ctx => move |_| {
        //     frozen_app.set(render_ctx.freeze());
        // })) { "Freeze!" }
        // p { (frozen_app.get()) }

        // button(on:click = cloned!(frozen_app_3, render_ctx => move |_| {
        //     render_ctx.thaw(&frozen_app_3.get(), perseus::state::ThawPrefs {
        //         page: perseus::state::PageThawPrefs::IncludeAll,
        //         global_prefer_frozen: true
        //     }).unwrap();
        // })) { "Thaw..." }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("idb")
        .build_state_fn(get_build_state)
        .template(idb_page)
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(_path: String, _locale: String) -> RenderFnResultWithCause<TestProps> {
    Ok(TestProps {
        username: "".to_string(),
    })
}
