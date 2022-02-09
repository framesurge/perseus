use perseus::{Html, Template};
use sycamore::prelude::{view, SsrNode, View};

use crate::global_state::AppStateRx;

// This template needs global state, but doesn't have any state of its own, so the first argument is the unit type `()` (which the macro will detect)
#[perseus::template_rx(AboutPage)]
pub fn index_page(_: (), global_state: AppStateRx) -> View<G> {
    let test = global_state.test;
    let test_2 = test.clone();
    view! {
        // The user can change the global state through an input, and the changes they make will be reflected throughout the app
        p { (test.get()) }
        input(bind:value = test_2)

        a(href = "about", id = "about-link") { "About" }
    }
}

#[perseus::head]
pub fn head() -> View<SsrNode> {
    view! {
        title { "Index Page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_page).head(head)
}
