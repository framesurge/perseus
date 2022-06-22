use perseus::state::Freeze;
use perseus::{Html, RenderFnResultWithCause, Template};
use sycamore::prelude::*;

use crate::global_state::*;

#[perseus::make_rx(IndexPropsRx)]
pub struct IndexProps {
    username: String,
}

#[perseus::template_rx]
pub fn index_page<'a, G: Html>(
    cx: Scope<'a>,
    state: IndexPropsRx<'a>,
    global_state: AppStateRx<'a>,
) -> View<G> {
    // This is not part of our data model, we do NOT want the frozen app synchronized as part of our page's state, it should be separate
    let frozen_app = create_signal(cx, String::new());
    let render_ctx = perseus::get_render_ctx!(cx);

    view! { cx,
        // For demonstration, we'll let the user modify the page's state and the global state arbitrarily
        p(id = "page_state") { (format!("Greetings, {}!", state.username.get())) }
        input(id = "set_page_state", bind:value = state.username, placeholder = "Username")
        p(id = "global_state") { (global_state.test.get()) }
        input(id = "set_global_state", bind:value = global_state.test, placeholder = "Global state")

        // When the user visits this and then comes back, they'll still be able to see their username (the previous state will be retrieved from the global state automatically)
        a(href = "about", id = "about-link") { "About" }
        br()

        button(id = "freeze_button", on:click = |_| {
            frozen_app.set(render_ctx.freeze());
        }) { "Freeze!" }
        p(id = "frozen_app") { (frozen_app.get()) }

        input(id = "thaw_input", bind:value = frozen_app, placeholder = "Frozen state")
        button(id = "thaw_button", on:click = |_| {
            render_ctx.thaw(&frozen_app.get(), perseus::state::ThawPrefs {
                page: perseus::state::PageThawPrefs::IncludeAll,
                global_prefer_frozen: true
            }).unwrap();
        }) { "Thaw..." }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_build_state)
        .template(index_page)
}

#[perseus::build_state]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<IndexProps> {
    Ok(IndexProps {
        username: "".to_string(),
    })
}
