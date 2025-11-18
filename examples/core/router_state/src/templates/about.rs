use perseus::prelude::*;
use sycamore::prelude::*;

fn about_page() -> View {
    view! {
        p { "Hello World!" }
    }
}

#[engine_only_fn]
fn head() -> View {
    view! {
        title { "About Page" }
    }
}

pub fn get_template() -> Template {
    Template::build("about").view(about_page).head(head).build()
}
