use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page(cx: Scope) -> View {
    view! {
        p { "Hello World!" }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! {
        title { "Index Page | Perseus Example – Plugins" }
    }
}

pub fn get_template() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}
