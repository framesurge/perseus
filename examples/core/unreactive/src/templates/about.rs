use perseus::{Html, SsrNode, Template};
use sycamore::prelude::{view, Scope, View};

// With the old template macro, we have to add the Sycamore `#[component(...)]`
// annotation manually and we get unreactive state passed in Additionally,
// global state is not supported at all So there's no way of persisting state
// between templates
#[perseus::template]
#[sycamore::component]
pub fn about_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "About." }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page).head(head)
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "About Page" }
    }
}
