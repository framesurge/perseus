use perseus::prelude::*;
use sycamore::prelude::*;

use crate::global_state::AppStateRx;

fn about_page<G: Html>(cx: Scope) -> View<G> {
    let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);

    view! { cx,
        // The user can change the global state through an input, and the changes they make will be reflected throughout the app
        p { (global_state.test.get()) }
        input(bind:value = global_state.test)

        a(href = "") { "Index" }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "About Page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("about").view(about_page).head(head).build()
}
