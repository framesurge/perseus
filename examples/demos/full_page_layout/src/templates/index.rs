use crate::components::layout::Layout;
use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Layout(title = "Index") {
            // Anything we put in here will be rendered inside the `<main>` block of the layout
            p { "Hello World!" }
            br {}
            a(href = "long") { "Long page" }
        }
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
