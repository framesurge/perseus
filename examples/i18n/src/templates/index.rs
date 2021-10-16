use perseus::{link, t, Template};
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};

#[component(IndexPage<G>)]
pub fn index_page() -> SycamoreTemplate<G> {
    let username = "User";
    template! {
        p { (t!("hello", {
            "user": username
        })) }
        a(href = link!("/about")) { "About" }
    }
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("index").template(|_| {
        template! {
            IndexPage()
        }
    })
}
