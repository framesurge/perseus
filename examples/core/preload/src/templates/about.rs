use perseus::prelude::*;
use sycamore::prelude::*;

fn about_page() -> View {
    view! {
        p { (t!("about-msg")) }

        Link(to = link!("/"), id = "index") { (t!("about-index-link")) }
    }
}

pub fn get_template() -> Template {
    Template::build("about").view(about_page).build()
}
