use perseus::{GenericNode, StringResultWithCause, Template};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use sycamore::prelude::{component, template, Template as SycamoreTemplate};

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexPageProps {
    pub greeting: String,
}

#[component(IndexPage<G>)]
pub fn index_page(props: IndexPageProps) -> SycamoreTemplate<G> {
    template! {
        p {(props.greeting)}
        a(href = "/about") { "About!" }
    }
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("index")
        .build_state_fn(Rc::new(get_build_props))
        .template(template_fn())
        .head(head_fn())
}

pub async fn get_build_props(_path: String) -> StringResultWithCause<String> {
    Ok(serde_json::to_string(&IndexPageProps {
        greeting: "Hello World!".to_string(),
    })
    .unwrap())
}

pub fn template_fn<G: GenericNode>() -> perseus::template::TemplateFn<G> {
    Rc::new(|props: Option<String>| {
        template! {
            IndexPage(
                serde_json::from_str::<IndexPageProps>(&props.unwrap()).unwrap()
            )
        }
    })
}

pub fn head_fn() -> perseus::template::HeadFn {
    Rc::new(|_| {
        template! {
            title { "Index Page | Perseus Example – Basic" }
        }
    })
}
