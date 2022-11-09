use perseus::Template;
use sycamore::prelude::{view, Html, Scope, View};

#[perseus::template]
pub fn about_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "About." }
        a(href = "") { "Index" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page)
}
