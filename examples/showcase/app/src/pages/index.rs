use sycamore::prelude::*;
use serde::{Serialize, Deserialize};
use perseus::page::Page;

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexPageProps {
    pub greeting: String
}

#[component(IndexPage<G>)]
pub fn index_page(props: IndexPageProps) -> Template<G> {
	template! {
		p {(props.greeting)}
        a(href = "/about") { "About!" }
	}
}

pub fn get_page<G: GenericNode>() -> Page<IndexPageProps, G> {
    Page::new("index")
        .build_state_fn(Box::new(get_static_props))
        .template(Box::new(|props: Option<IndexPageProps>| template! {
            IndexPage(props.unwrap())
        }))
}

pub fn get_static_props(_path: String) -> IndexPageProps {
    IndexPageProps {
        greeting: "Hello World!".to_string()
    }
}
