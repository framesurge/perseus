use crate::templates::index::IndexPageStateRx;
use perseus::{get_render_ctx, Html, Template};
use sycamore::prelude::{view, Signal};
use sycamore::view::View;

// WARNING: Accessing the page state store manually as this template does is NOT recommended, and is done for demonstration purposes only! In reality, you should use global state for anything that
// you need to share between pages.

#[perseus::template_rx(AboutPage)]
pub fn about_page() -> View<G> {
    // Get the page state store manually
    // The index page is just an empty string
    let index_props_rx = get_render_ctx!()
        .page_state_store
        .get::<IndexPageStateRx>("");
    // Get the state from the index page
    // If the user hasn't visited there yet, this won't exist
    let username = match index_props_rx {
        Some(IndexPageStateRx { username }) => username,
        None => Signal::new("".to_string()),
    };

    view! {
        p { (format!("Greetings, {}!", username.get())) }

        a(href = "") { "Index" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page)
}
