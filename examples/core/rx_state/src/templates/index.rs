use perseus::{prelude::*, state::rx_collections::RxVec};
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "IndexPageStateRx")]
struct IndexPageState {
    username: String,
    #[rx(nested)]
    test: RxVec<String>,
}

// This macro will make our state reactive *and* store it in the page state
// store, which means it'll be the same even if we go to the about page and come
// back (as long as we're in the same session)
fn index_page(state: IndexPageStateRx) -> View {
    // IMPORTANT: Remember, Perseus caches all reactive state, so, if you come here,
    // go to another page, and then come back, *two* elements will have been
    // added in total. The state is preserved across routes! To avoid this, use
    // unreactive state.
    //
    // ADVANCED: Note also that, even if we ignored this page's state type from HSR,
    // this would still be performed for HSR, because the state restoration
    // process will double-execute this logic. That's why things like this
    // should generally be done with suspended state.
    // In Sycamore 0.9.2, use .update() instead of .modify()
    state
        .test
        .update(|vec| vec.push(create_signal("bar".to_string())));

    view! {
        p { (format!("Greetings, {}!", state.username.get_clone())) }
        input(bind:value = state.username, placeholder = "Username")
        p { (
            state
                .test
                // Get the underlying `Vec`
                .get_clone()
                // Now, in that `Vec`, get the third element
                .get(2)
                // Because that will be `None` initially, display `None` otherwise
                .map(|x| x.get_clone())
                .unwrap_or("None".to_string().into())
        ) }

        // Note: Using on:click with navigate() because sycamore-router doesn't attach click handlers to dynamic views.
        // The href is kept for SEO (crawlers will see the link).
        a(href = "/about", id = "about-link", on:click = |ev: web_sys::MouseEvent| {
            ev.prevent_default();
            navigate("/about");
        }) { "About" }
    }
}

#[engine_only_fn]
fn head() -> View {
    view! {
        title { "Index Page" }
    }
}

pub fn get_template() -> Template {
    Template::build("index")
        .view_with_state(index_page)
        .head(head)
        .build_state_fn(get_build_state)
        .build()
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> IndexPageState {
    IndexPageState {
        username: "".to_string(),
        test: vec!["foo".to_string()].into(),
    }
}
