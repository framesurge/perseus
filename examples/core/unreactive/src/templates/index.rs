use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

// Without `#[make_rx(...)]`, we have to manually derive `Serialize` and
// `Deserialize`
// We derive `UnreactiveState` too, which actually creates a pseudo-reactive
// wrapper for this unreactive type, allowing it to work with Perseus;
// rather strict state platform (this is just a marker trait though)
#[derive(Serialize, Deserialize, Clone, UnreactiveState)]
struct IndexPageState {
    pub greeting: String,
}

// By adding `unreactive` in brackets, we tell Perseus to expect something with
// `UnreactiveState` derived.
// Otherwise, you can do everything in this macro that you can do with a
// reactive template! Caching, preloading, reactive global state, etc. are all
// supported.
fn index_page<G: Html>(cx: Scope, state: IndexPageState) -> View<G> {
    view! { cx,
        p { (state.greeting) }
        a(href = "about") { "About" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_build_state)
        .template_with_unreactive_state(index_page)
        .head_with_state(head)
}

#[engine_only_fn]
fn head(cx: Scope, _props: IndexPageState) -> View<SsrNode> {
    view! { cx,
        title { "Index Page" }
    }
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> RenderFnResultWithCause<IndexPageState> {
    Ok(IndexPageState {
        greeting: "Hello World!".to_string(),
    })
}
