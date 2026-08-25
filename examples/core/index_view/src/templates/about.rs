use perseus::prelude::*;
use sycamore::prelude::*;

fn about_page() -> View {
    view! {
        p { "About." }
        Link(to = "/") { "Index." }
    }
}

pub fn get_template() -> Template {
    Template::build("about").view(about_page).build()
}
