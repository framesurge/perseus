use perseus::{Html, Template};
use sycamore::prelude::{view, SsrNode, View};

#[perseus::template_rx(IndexPage)]
pub fn index_page() -> View<G> {
    view! {
        p { "Hello World!" }
        a(href = "about", id = "about-link") { "About!" }
    }
}

#[perseus::head]
pub fn head() -> View<SsrNode> {
    view! {
        title { "Index Page | Perseus Example – Plugins" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_page).head(head)
}
