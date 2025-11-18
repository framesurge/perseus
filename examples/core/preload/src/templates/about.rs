use perseus::prelude::*;
use sycamore::prelude::*;
use sycamore::view::View;

fn about_page() -> View {
    view! {
        p { (t!("about-msg")) }

        a(id = "index", href = link!("")) { (t!("about-index-link")) }
    }
}

pub fn get_template() -> Template {
    Template::build("about").view(about_page).build()
}
