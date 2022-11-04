use perseus::{Html, RenderFnResultWithCause, SsrNode, Template};
use sycamore::prelude::{view, Scope, View};

#[perseus::make_rx(IndexPageStateRx)]
pub struct IndexPageState {
    pub greeting: String,
}

#[perseus::template_rx]
pub fn index_page<'a, G: Html>(cx: Scope<'a>, state: IndexPageStateRx<'a>) -> View<G> {
    view! { cx,
        p { (state.greeting.get()) }
        a(href = "about", id = "about-link") { "About!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_build_state)
        .template(index_page)
        .head(head)
        .build_paths_fn(build_paths)
}

#[perseus::head]
pub fn head(cx: Scope, _props: IndexPageState) -> View<SsrNode> {
    view! { cx,
        title { "Index Page | Perseus Example – Basic" }
    }
}

#[perseus::build_state]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<IndexPageState> {
    Ok(IndexPageState {
        greeting: "Hello World!".to_string(),
    })
}

#[perseus::build_paths]
async fn build_paths() -> perseus::prelude::RenderFnResult<Vec<String>> {
    Ok(vec!["".to_string(), "a test".to_string()])
}
