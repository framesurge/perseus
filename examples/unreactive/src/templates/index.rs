use perseus::{Html, RenderFnResultWithCause, SsrNode, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::{view, View};

// Without `#[make_rx(...)]`, we have to manually derive `Serialize` and `Deserialize`
#[derive(Serialize, Deserialize)]
pub struct IndexPageState {
    pub greeting: String,
}

// With the old template macro, we have to add the Sycamore `#[component(...)]` annotation manually and we get unreactive state passed in
// Additionally, global state is not supported at all
// So there's no way of persisting state between templates
#[perseus::template(IndexPage)]
#[sycamore::component(IndexPage<G>)]
pub fn index_page(state: IndexPageState) -> View<G> {
    view! {
        p { (state.greeting) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_build_state)
        .template(index_page)
        .head(head)
}

#[perseus::head]
pub fn head(_props: IndexPageState) -> View<SsrNode> {
    view! {
        title { "Index Page" }
    }
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<IndexPageState> {
    Ok(IndexPageState {
        greeting: "Hello World!".to_string(),
    })
}
