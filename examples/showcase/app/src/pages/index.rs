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

pub fn get_page<G: GenericNode>() -> Template<G> {
    Template::new("index")
        .build_state_fn(Box::new(get_static_props))
        .template(template_fn())
}

pub fn get_static_props(_path: String) -> String {
    serde_json::to_string(
        &IndexPageProps {
            greeting: "Hello World!".to_string()
        }
    ).unwrap()
}

pub fn template_fn<G: GenericNode>() -> perseus::template::TemplateFn<G> {
    Box::new(|props: Option<String>| template! {
        IndexPage(
            serde_json::from_str::<IndexPageProps>(&props.unwrap()).unwrap()
        )
    })
}