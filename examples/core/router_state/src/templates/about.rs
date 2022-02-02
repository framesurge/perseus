use perseus::{Html, Template};
use sycamore::prelude::{view, SsrNode, View};

#[perseus::template_rx(AboutPage)]
pub fn about_page() -> View<G> {
    view! {
        p { "Hello World!" }
    }
}

#[perseus::head]
pub fn head() -> View<SsrNode> {
    view! {
        title { "About Page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(index_page).head(head)
}
