use perseus::{Html, Template};
use sycamore::prelude::{view, Scope};
use sycamore::view::View;

#[perseus::template]
pub fn about_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "Try going back to the index page, and the state should still be the same!" }

        a(id = "index-link", href = "") { "Index" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page)
}
