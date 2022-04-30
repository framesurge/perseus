use perseus::{Html, Template};
use sycamore::prelude::{Scope, SsrNode, View, view};

use crate::global_state::AppStateRx;

// This template needs global state, but doesn't have any state of its own, so the first argument is the unit type `()` (which the macro will detect)
#[perseus::template_rx]
pub fn about_page<G: Html>(cx: Scope, _: (), global_state: AppStateRx) -> View<G> {
    let test = global_state.test;
    let test_2 = test.clone();
    view! { cx,
        // The user can change the global state through an input, and the changes they make will be reflected throughout the app
        p { (test.get()) }
        input(bind:value = test_2)

        a(href = "") { "Index" }
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "About Page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page).head(head)
}
