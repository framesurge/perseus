use crate::templates::index::IndexPageStateRx;
use perseus::{get_render_ctx, Html, Template};
use sycamore::prelude::{Scope, create_rc_signal, view};
use sycamore::view::View;

// WARNING: Accessing the page state store manually as this template does is NOT recommended, and is done for demonstration purposes only! In reality, you should use global state for anything that
// you need to share between pages.

#[perseus::template_rx]
pub fn about_page<G: Html>(cx: Scope) -> View<G> {
    // Get the page state store manually
    // The index page is just an empty string
    let index_props_rx = get_render_ctx!(cx)
        .page_state_store
        .get::<IndexPageStateRx>("");
    // Get the state from the index page
    // If the user hasn't visited there yet, this won't exist
    let username = match index_props_rx {
        Some(IndexPageStateRx { username }) => username,
        None => create_rc_signal("".to_string()),
    };

    view! { cx,
        p { (format!("Greetings, {}!", username.get())) }

        a(href = "") { "Index" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page)
}
