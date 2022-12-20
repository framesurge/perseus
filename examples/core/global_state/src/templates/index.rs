use crate::global_state::AppStateRx;
use perseus::prelude::*;
use sycamore::prelude::*;

// Note that this template takes no state of its own in this example, but it
// certainly could
fn index_page<G: Html>(cx: Scope) -> View<G> {
    // We access the global state through the render context, extracted from
    // Sycamore's context system
    let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);

    view! { cx,
        // The user can change the global state through an input, and the changes they make will be reflected throughout the app
        p { (global_state.test.get()) }
        input(bind:value = global_state.test)

        a(href = "about", id = "about-link") { "About" }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Index Page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").view(index_page).head(head).build()
}
