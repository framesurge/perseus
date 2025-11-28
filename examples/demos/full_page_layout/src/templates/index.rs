use crate::components::layout::Layout;
use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page() -> View {
    view! {
        Layout(title = "Index".to_string()) {
            // Anything we put in here will be rendered inside the `<main>` block of the layout
            p { "Hello World!" }
            br {}
            a(href = "long") { "Long page" }
        }
    }
}

#[engine_only_fn]
fn head() -> View {
    view! {
        title { "Index Page" }
    }
}

pub fn get_template() -> Template {
    Template::build("index").view(index_page).head(head).build()
}
