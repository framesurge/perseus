use perseus::Template;
use sycamore::prelude::{view, Html, Scope, SsrNode, View};

fn about_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "About." }
    }
}

fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "About Page | Perseus Example – Basic" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page).head(head)
}
