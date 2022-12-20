use perseus::prelude::*;
use sycamore::prelude::*;
use sycamore::view::View;

fn about_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "Check out your browser's network DevTools, no new requests were needed to get to this page!" }

        a(id = "index-link", href = "") { "Index" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").view(about_page).build()
}
