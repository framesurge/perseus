use perseus::Template;
use sycamore::prelude::{view, Html, View};

#[perseus::template_rx]
pub fn about_page() -> View<G> {
    view! {
        p { "About." }
        a(href = "") { "Index" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page)
}
