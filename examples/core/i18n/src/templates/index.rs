use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page() -> View {
    let username = "User";

    view! {
        p { (t!("hello", {
            "user" = username
        })) }
        Link(to = link!("/about")) { "About" }
    }
}

pub fn get_template() -> Template {
    Template::build("index").view(index_page).build()
}
