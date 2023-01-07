use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    let username = "User";

    view! { cx,
        p { (t!(cx, "hello", {
            "user" = username
        })) }
        a(href = link!(cx, "/about")) { "About" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).build()
}
