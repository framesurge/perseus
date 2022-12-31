use perseus::prelude::*;
use sycamore::prelude::*;
use sycamore::view::View;

fn about_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { (t!("about-msg", cx)) }

        a(id = "index", href = link!("", cx)) { (t!("about-index-link", cx)) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("about").view(about_page).build()
}
