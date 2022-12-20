use crate::components::layout::Layout;
use perseus::prelude::*;
use sycamore::prelude::*;

fn long_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Layout(title = "Long") {
            // Anything we put in here will be rendered inside the `<main>` block of the layout
            a(href = "") { "Index" }
            br {}
            p {
                ("This is a test. ".repeat(5000))
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Long Page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("long").view(long_page).head(head).build()
}
