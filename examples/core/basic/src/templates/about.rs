use perseus::prelude::*;
use sycamore::prelude::*;

fn about_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "About." }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "About Page | Perseus Example – Basic" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page).head(head)
}
