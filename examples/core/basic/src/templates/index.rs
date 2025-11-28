use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "IndexPageStateRx")]
struct IndexPageState {
    greeting: String,
}

#[auto_scope]
fn index_page(state: IndexPageStateRx) -> View {
    view! {
        p { (state.greeting.get_clone()) }
        a(href = "about", id = "about-link") { "About!" }
    }
}

#[engine_only_fn]
fn head(_props: IndexPageState) -> View {
    view! {
        title { "Index Page | Perseus Example – Basic" }
    }
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> IndexPageState {
    IndexPageState {
        greeting: "Hello World!".to_string(),
    }
}

pub fn get_template() -> Template {
    Template::build("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .head_with_state(head)
        .build()
}
