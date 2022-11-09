use perseus::{Html, Template};
use sycamore::prelude::{view, Scope, View};

#[perseus::template]
pub fn index_page<'a, G: Html>(cx: Scope<'a>) -> View<G> {
    view! { cx,
        p { "Hello World!" }
        a(href = "about", id = "about-link") { "About!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_page)
}
