use perseus::state::Freeze;
use perseus::{Html, RenderFnResultWithCause, Template};
use sycamore::prelude::*;

use crate::global_state::AppStateRx;

// We define a normal `struct` and then use `make_rx` (which derives `Serialize`, `Deserialize`, and `Clone` automatically)
// This will generate a new `struct` called `IndexPropsRx` (as we asked it to), in which every field is made reactive with a `Signal`
#[perseus::make_rx(IndexPropsRx)]
pub struct IndexProps {
    pub username: String,
}

// This special macro (normally we'd use `template(IndexProps)`) converts the state we generate elsewhere to a reactive version
// We need to tell it the name of the unreactive properties we created to start with (unfortunately the compiler isn't smart enough to figure that out yet)
// This will also add our reactive properties to the global state store, and, if they're already there, it'll use the existing one
#[perseus::template_rx(IndexPage)]
pub fn index_page(IndexPropsRx { username }: IndexPropsRx, global_state: AppStateRx) -> View<G> {
    let username_2 = username.clone(); // This is necessary until Sycamore's new reactive primitives are released
    let frozen_app = Signal::new(String::new()); // This is not part of our data model, so it's not part of the state properties (everything else should be though)
    let frozen_app_2 = frozen_app.clone();
    let frozen_app_3 = frozen_app.clone();
    let render_ctx = perseus::get_render_ctx!();

    view! {
        p { (format!("Greetings, {}!", username.get())) }
        input(bind:value = username_2, placeholder = "Username")
        p { (global_state.test.get()) }

        // When the user visits this and then comes back, they'll still be able to see their username (the previous state will be retrieved from the global state automatically)
        a(href = "about") { "About" }
        br()

        button(on:click = cloned!(frozen_app, render_ctx => move |_| {
            frozen_app.set(render_ctx.freeze());
        })) { "Freeze!" }
        p { (frozen_app.get()) }

        input(bind:value = frozen_app_2, placeholder = "Frozen state")
        button(on:click = cloned!(frozen_app_3, render_ctx => move |_| {
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
