use perseus::prelude::*;
use sycamore::prelude::*;

fn about_page(cx: Scope) -> View {
    view! {
        p { "About." }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! {
        title { "About Page | Perseus Example – Basic" }
    }
}

pub fn get_template() -> Template<G> {
    Template::build("about").view(about_page).head(head).build()
}
