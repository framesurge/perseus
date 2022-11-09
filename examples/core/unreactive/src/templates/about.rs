use perseus::prelude::*;
use sycamore::prelude::*;

#[perseus::template(unreactive)]
pub fn about_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "About." }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page).head(head)
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "About Page" }
    }
}
