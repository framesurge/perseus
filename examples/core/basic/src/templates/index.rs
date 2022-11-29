use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "IndexPageStateRx")]
struct IndexPageState {
    pub greeting: String,
}

#[perseus::template]
fn index_page<'a, G: Html>(cx: Scope<'a>, state: IndexPageStateRx<'a>) -> View<G> {
    view! { cx,
        p { (state.greeting.get()) }
        a(href = "about", id = "about-link") { "About!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_build_state)
        .template_with_state(index_page)
        .head_with_state(head)
}

#[engine_only_fn]
fn head(cx: Scope, _props: IndexPageState) -> View<SsrNode> {
    view! { cx,
        title { "Index Page | Perseus Example – Basic" }
    }
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> RenderFnResultWithCause<IndexPageState> {
    Ok(IndexPageState {
        greeting: "Hello World!".to_string(),
    })
}
