use perseus::state::Freeze;
use perseus::{Html, RenderFnResultWithCause, Template};
use sycamore::prelude::*;

use crate::global_state::AppStateRx;

#[perseus::make_rx(IndexPropsRx)]
pub struct IndexProps {
    username: String,
}

#[perseus::template_rx]
pub fn index_page(state: IndexPropsRx, global_state: AppStateRx) -> View<G> {
    let username = state.username;
    let username_2 = username.clone(); // This is necessary until Sycamore's new reactive primitives are released
    let test = global_state.test;
    let test_2 = test.clone();

    // This is not part of our data model, we do NOT want the frozen app synchronized as part of our page's state, it should be separate
    let frozen_app = Signal::new(String::new());
    let frozen_app_2 = frozen_app.clone();
    let frozen_app_3 = frozen_app.clone();
    let render_ctx = perseus::get_render_ctx!();

    view! {
        // For demonstration, we'll let the user modify the page's state and the global state arbitrarily
        p(id = "page_state") { (format!("Greetings, {}!", username.get())) }
        input(id = "set_page_state", bind:value = username_2, placeholder = "Username")
        p(id = "global_state") { (test.get()) }
        input(id = "set_global_state", bind:value = test_2, placeholder = "Global state")

        // When the user visits this and then comes back, they'll still be able to see their username (the previous state will be retrieved from the global state automatically)
        a(href = "about", id = "about-link") { "About" }
        br()

        button(id = "freeze_button", on:click = cloned!(frozen_app, render_ctx => move |_| {
            frozen_app.set(render_ctx.freeze());
        })) { "Freeze!" }
        p(id = "frozen_app") { (frozen_app.get()) }

        input(id = "thaw_input", bind:value = frozen_app_2, placeholder = "Frozen state")
        button(id = "thaw_button", on:click = cloned!(frozen_app_3, render_ctx => move |_| {
            render_ctx.thaw(&frozen_app_3.get(), perseus::state::ThawPrefs {
                page: perseus::state::PageThawPrefs::IncludeAll,
                global_prefer_frozen: true
            }).unwrap();
        })) { "Thaw..." }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_build_state)
        .template(index_page)
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<IndexProps> {
    Ok(IndexProps {
        username: "".to_string(),
    })
}
