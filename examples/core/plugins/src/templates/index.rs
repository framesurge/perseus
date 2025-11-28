use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page() -> View {
    view! {
        p { "Hello World!" }
    }
}

#[engine_only_fn]
fn head() -> View {
    view! {
        title { "Index Page | Perseus Example – Plugins" }
    }
}

pub fn get_template() -> Template {
    Template::build("index").view(index_page).head(head).build()
}
