use crate::global_state::AppStateRx;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "IndexPageStateRx")]
struct IndexPageState {
    username: String,
}

fn index_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a IndexPageStateRx) -> View<G> {
    // This is not part of our data model, we do NOT want the frozen app
    // synchronized as part of our page's state, it should be separate
    let frozen_app = create_signal(cx, String::new());
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

        button(id = "freeze_button", on:click = |_| {
            #[cfg(target_arch = "wasm32")]
            {
                use perseus::state::Freeze;
                frozen_app.set(reactor.freeze());
            }
        }) { "Freeze!" }
        p(id = "frozen_app") { (frozen_app.get()) }

        input(id = "thaw_input", bind:value = frozen_app, placeholder = "Frozen state")
        button(id = "thaw_button", on:click = |_| {
            #[cfg(target_arch = "wasm32")]
            reactor.thaw(&frozen_app.get(), perseus::state::ThawPrefs {
                page: perseus::state::PageThawPrefs::IncludeAll,
                global_prefer_frozen: true
            }).unwrap();
        }) { "Thaw..." }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .build()
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> IndexPageState {
    IndexPageState {
        username: "".to_string(),
    }
}
