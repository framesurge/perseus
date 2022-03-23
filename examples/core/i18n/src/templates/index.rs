use perseus::{link, t, Template};
use sycamore::prelude::{view, Html, View};

#[perseus::template_rx]
pub fn index_page() -> View<G> {
    let username = "User";
    view! {
        p { (t!("hello", {
            "user": username
        })) }
        a(href = link!("/about")) { "About" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_page)
}
