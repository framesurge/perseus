use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page() -> View {
    view! {
        p { "Hello World!" }
        Link(to = "/about", id = "about-link") { "About!" }
    }
}

pub fn get_template() -> Template {
    Template::build("index").view(index_page).build()
}
