use perseus::{link, t, Template};
use sycamore::prelude::{view, Html, Scope, View};

fn index_page<G: Html>(cx: Scope) -> View<G> {
    let username = "User";

    view! { cx,
        p { (t!("hello", {
            "user" = username
        }, cx)) }
        a(href = link!("/about", cx)) { "About" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_page)
}
