use perseus::Template;
use sycamore::prelude::{view, Html, SsrNode, View};

// This page will actually be replaced entirely by a plugin!
#[perseus::template_rx(AboutPage)]
pub fn about_page() -> View<G> {
    view! {
        p { "About." }
    }
}

#[perseus::head]
pub fn head() -> View<SsrNode> {
    view! {
        title { "About Page | Perseus Example – Plugins" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page).head(head)
}
