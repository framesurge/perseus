use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "IndexPageStateRx")]
struct IndexPageState {
    pub username: String,
}

// This macro will make our state reactive *and* store it in the page state
// store, which means it'll be the same even if we go to the about page and come
// back (as long as we're in the same session)
fn index_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a IndexPageStateRx) -> View<G> {
    view! { cx,
        p { (format!("Greetings, {}!", state.username.get())) }
        input(bind:value = state.username, placeholder = "Username")

        a(href = "about", id = "about-link") { "About" }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Index Page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .view_with_state(index_page)
        .head(head)
        .build_state_fn(get_build_state)
        .build()
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> IndexPageState {
    IndexPageState {
        username: "".to_string(),
    }
}
