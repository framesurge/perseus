use perseus::{Html, Template};
use sycamore::prelude::{view, View};

#[perseus::template_rx]
pub fn index_page() -> View<G> {
    view! {
        p { "Hello World!" }
        a(href = "about", id = "about-link") { "About!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_page)
}
