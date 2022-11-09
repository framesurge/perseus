use perseus::{Html, Template};
use sycamore::prelude::{view, Scope, SsrNode, View};

#[perseus::template]
pub fn about_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "Hello World!" }
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "About Page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page).head(head)
}
