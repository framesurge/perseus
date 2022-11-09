use perseus::{Html, Template};
use sycamore::prelude::{view, Scope};
use sycamore::view::View;

#[perseus::template]
pub fn about_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "Check out your browser's network DevTools, no new requests were needed to get to this page!" }

        a(id = "index-link", href = "") { "Index" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page)
}
