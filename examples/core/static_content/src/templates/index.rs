use perseus::{Html, Template};
use sycamore::prelude::{view, Scope, SsrNode, View};

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "Hello World!" }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Index Page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_page).head(head)
}
