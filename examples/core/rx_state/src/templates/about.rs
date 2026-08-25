use perseus::prelude::*;
use sycamore::prelude::*;

fn about_page() -> View {
    view! {
        p { "Try going back to the index page, and the state should still be the same!" }

        Link(to = "/", id = "index-link") { "Index" }
    }
}

pub fn get_template() -> Template {
    Template::build("about").view(about_page).build()
}
