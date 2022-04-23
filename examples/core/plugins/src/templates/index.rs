use perseus::{Html, Template};
use sycamore::prelude::{view, Scope, SsrNode, View};

#[perseus::template_rx]
pub fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "Hello World!" }
        a(href = "about", id = "about-link") { "About!" }
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Index Page | Perseus Example – Plugins" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_page).head(head)
}
