use perseus::{GenericNode, Template};
use sycamore::prelude::{component, template, SsrNode, Template as SycamoreTemplate};

#[perseus::template(IndexPage)]
#[component(IndexPage<G>)]
pub fn index_page() -> SycamoreTemplate<G> {
    template! {
        p { "Hello World!" }
        a(href = "about", id = "about-link") { "About!" }
    }
}

#[perseus::head]
pub fn head() -> SycamoreTemplate<SsrNode> {
    template! {
        title { "Index Page | Perseus Example – Plugins" }
    }
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("index").template(index_page).head(head)
}
