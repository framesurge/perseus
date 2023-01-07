use perseus::prelude::*;
use sycamore::prelude::*;
use sycamore::view::View;

fn about_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { (t!(cx, "about-msg")) }

        a(id = "index", href = link!(cx, "")) { (t!(cx, "about-index-link")) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("about").view(about_page).build()
}
