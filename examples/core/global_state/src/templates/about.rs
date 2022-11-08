use perseus::{Html, Template};
use sycamore::prelude::{view, Scope, SsrNode, View};

use crate::global_state::AppStateRx;

// This template needs global state, but doesn't have any state of its own, so
// the first argument is the unit type `()` (which the macro will detect)
#[perseus::template_rx]
pub fn about_page<'a, G: Html>(cx: Scope<'a>) -> View<G> {
    let global_state = perseus::get_render_ctx!(cx).get_global_state::<AppStateRx>(cx);

    view! { cx,
        // The user can change the global state through an input, and the changes they make will be reflected throughout the app
        p { (global_state.test.get()) }
        input(bind:value = global_state.test)

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
