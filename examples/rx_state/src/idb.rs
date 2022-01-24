use perseus::state::IdbFrozenStateStore;
use perseus::{Html, RenderFnResultWithCause, Template};
use sycamore::prelude::*;

#[perseus::make_rx(TestPropsRx)]
pub struct TestProps {
    pub username: String,
}

// This special macro (normally we'd use `template(IndexProps)`) converts the state we generate elsewhere to a reactive version
#[perseus::template_rx(IdbPage)]
pub fn idb_page(TestPropsRx { username }: TestPropsRx) -> View<G> {
    let username_2 = username.clone(); // This is necessary until Sycamore's new reactive primitives are released
                                       // TODO Futures etc.
    let _idb_store = IdbFrozenStateStore::new();

    view! {
        p { (format!("Greetings, {}!", username.get())) }
        input(bind:value = username_2, placeholder = "Username")

        // When the user visits this and then comes back, they'll still be able to see their username (the previous state will be retrieved from the global state automatically)
        a(href = "about") { "About" }
        a(href = "") { "Index" }

        // button(on:click = cloned!(frozen_app, render_ctx => move |_| {
        //     frozen_app.set(render_ctx.freeze());
        // })) { "Freeze!" }
        // p { (frozen_app.get()) }

        // button(on:click = cloned!(frozen_app_3, render_ctx => move |_| {
        //     render_ctx.thaw(&frozen_app_3.get(), perseus::state::ThawPrefs {
        //         page: perseus::state::PageThawPrefs::IncludeAll,
        //         global_prefer_frozen: true
        //     }).unwrap();
        // })) { "Thaw..." }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("idb")
        .build_state_fn(get_build_state)
        .template(idb_page)
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(_path: String, _locale: String) -> RenderFnResultWithCause<TestProps> {
    Ok(TestProps {
        username: "".to_string(),
    })
}
