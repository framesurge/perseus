use perseus::{t, GenericNode, Template};
use std::rc::Rc;
use sycamore::prelude::{component, template, Template as SycamoreTemplate};

#[component(IndexPage<G>)]
pub fn index_page() -> SycamoreTemplate<G> {
    template! {
        p { (t!("hello")) }
    }
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("index")
        .template(Rc::new(|_| {
            template! {
                IndexPage()
            }
        }))
        .head(Rc::new(|_| {
            template! {
                title { "Perseus" }
            }
        }))
}
