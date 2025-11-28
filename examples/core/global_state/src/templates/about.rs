use perseus::prelude::*;
use sycamore::prelude::*;

use crate::global_state::AppStateRx;

fn about_page() -> View {
    let global_state = Reactor::from_cx().get_global_state::<AppStateRx>();

    view! {
        // The user can change the global state through an input, and the changes they make will be reflected throughout the app
        p { (global_state.test.get_clone()) }
        input(bind:value = global_state.test)

        a(href = "") { "Index" }
    }
}

#[engine_only_fn]
fn head() -> View {
    view! {
        title { "About Page" }
    }
}

pub fn get_template() -> Template {
    Template::build("about").view(about_page).head(head).build()
}
