use perseus::{
    http::header::{HeaderMap, HeaderName},
    Html, RenderFnResultWithCause, SsrNode, Template,
};
use sycamore::prelude::{view, View};

#[perseus::make_rx(IndexPageStateRx)]
pub struct IndexPageState {
    pub greeting: String,
}

#[perseus::template_rx(IndexPage)]
pub fn index_page(state: IndexPageStateRx) -> View<G> {
    view! {
        p { (state.greeting.get()) }
        a(href = "about", id = "about-link") { "About!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_build_state)
        .template(index_page)
        .head(head)
        .set_headers_fn(set_headers)
}

#[perseus::head]
pub fn head(_props: IndexPageState) -> View<SsrNode> {
    view! {
        title { "Index Page | Perseus Example – Basic" }
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

#[perseus::autoserde(set_headers)]
pub fn set_headers(props: Option<IndexPageState>) -> HeaderMap {
    let mut map = HeaderMap::new();
    map.insert(
        HeaderName::from_lowercase(b"x-greeting").unwrap(),
        props.unwrap().greeting.parse().unwrap(),
    );
    map
}
