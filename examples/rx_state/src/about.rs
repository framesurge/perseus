use crate::index::IndexPropsRx;
use perseus::{get_render_ctx, Html, Template};
use sycamore::prelude::{component, view, Signal};
use sycamore::view::View;

// This template doesn't have any properties, so there's no point in using the special `template_with_rx_state` macro (but we could)
#[perseus::template(AboutPage)]
#[component(AboutPage<G>)]
pub fn about_page() -> View<G> {
    // Get the global state manually
    let global_state = get_render_ctx!().global_state;
    // Get the state from the index page
    // If the user hasn't visited there yet, this won't exist
    let username = match global_state.get::<IndexPropsRx>() {
        Some(IndexPropsRx { username }) => username,
        None => Signal::new("".to_string()),
    };

    view! {
        p { (format!("Greetings, {}!", username.get())) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page)
}
