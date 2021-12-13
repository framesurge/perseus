use perseus::{
    http::header::{HeaderMap, HeaderName},
    Html, RenderFnResultWithCause, SsrNode, Template,
};
use serde::{Deserialize, Serialize};
use sycamore::prelude::{component, view, View};

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexPageProps {
    pub greeting: String,
}

#[perseus::template(IndexPage)]
#[component(IndexPage<G>)]
pub fn index_page(props: IndexPageProps) -> View<G> {
    view! {
        p {(props.greeting)}
        a(href = "about", id = "about-link") { "About!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_build_props)
        .template(index_page)
        .head(head)
        .set_headers_fn(set_headers)
}

#[perseus::head]
pub fn head(_props: IndexPageProps) -> View<SsrNode> {
    view! {
        title { "Index Page | Perseus Example – Basic" }
    }
}

#[perseus::autoserde(build_state)]
pub async fn get_build_props(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<IndexPageProps> {
    Ok(IndexPageProps {
        greeting: "Hello World!".to_string(),
    })
}

#[perseus::autoserde(set_headers)]
pub fn set_headers(props: Option<IndexPageProps>) -> HeaderMap {
    let mut map = HeaderMap::new();
    map.insert(
        HeaderName::from_lowercase(b"x-greeting").unwrap(),
        props.unwrap().greeting.parse().unwrap(),
    );
    map
}
