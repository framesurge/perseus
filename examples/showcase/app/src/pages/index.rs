use serde::{Serialize, Deserialize};
use sycamore::prelude::{template, component, GenericNode, Template as SycamoreTemplate};
use perseus::template::Template;

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexPageProps {
    pub greeting: String
}

#[component(IndexPage<G>)]
pub fn index_page(props: IndexPageProps) -> SycamoreTemplate<G> {
	template! {
		p {(props.greeting)}
        a(href = "/about") { "About!" }
	}
}

pub fn get_page<G: GenericNode>() -> Template<IndexPageProps, G> {
    Template::new("index")
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
