use crate::components::layout::Layout;
use perseus::Template;
use sycamore::prelude::{view, Html, Scope, SsrNode, View};

#[perseus::template_rx]
pub fn long_page<G: Html>(cx: Scope) -> View<G> {
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

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Long Page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("long").template(long_page).head(head)
}
