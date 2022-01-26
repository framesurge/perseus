use perseus::{RenderFnResultWithCause, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::{component, view, Html, View};

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexPageProps {
    pub greeting: String,
}

#[perseus::template(IndexPage)]
#[component(IndexPage<G>)]
pub fn index_page(props: IndexPageProps) -> View<G> {
    view! {
        p {(props.greeting)}
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_static_props)
        .template(index_page)
}

#[perseus::autoserde(build_state)]
pub async fn get_static_props(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<IndexPageProps> {
    Ok(IndexPageProps {
        greeting: "Hello World!".to_string(),
    })
}
