use crate::global_state::AppStateRx;
use perseus::prelude::*;
use sycamore::prelude::*;

// Note that this template takes no state of its own in this example, but it
// certainly could
fn index_page() -> View {
    // We access the global state through the render context, extracted from
    // Sycamore's context system
    let global_state = Reactor::from_cx().get_global_state::<AppStateRx>();

    view! {
        // The user can change the global state through an input, and the changes they make will be reflected throughout the app
        p { (global_state.test.get_clone()) }
        input(bind:value = global_state.test)

        a(href = "about", id = "about-link") { "About" }
    }
}

#[engine_only_fn]
fn head() -> View {
    view! {
        title { "Index Page" }
    }
}

pub fn get_template() -> Template {
    Template::build("index").view(index_page).head(head).build()
}
