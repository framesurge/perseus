use crate::components::layout::Layout;
use perseus::prelude::*;
use sycamore::prelude::*;

fn long_page() -> View {
    view! {
        Layout(title = "Long".to_string()) {
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
fn head() -> View {
    view! {
        title { "Long Page" }
    }
}

pub fn get_template() -> Template {
    Template::build("long").view(long_page).head(head).build()
}
