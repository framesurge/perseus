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
fn index_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a IndexPageStateRx) -> View<G> {
    // IMPORTANT: Remember, Perseus caches all reactive state, so, if you come here,
    // go to another page, and then come back, *two* elements will have been
    // added in total. The state is preserved across routes! To avoid this, use
    // unreactive state.
    state
        .test
        .modify()
        .push(create_rc_signal("bar".to_string()));

    view! { cx,
        p { (format!("Greetings, {}!", state.username.get())) }
        input(bind:value = state.username, placeholder = "Username")
        p { (
            state
                .test
                // Get the underlying `Vec`
                .get()
                // Now, in that `Vec`, get the third element
                .get(2)
                // Because that will be `None` initially, display `None` otherwise
                .map(|x| x.get())
                .unwrap_or("None".to_string().into())
        ) }

        a(href = "about", id = "about-link") { "About" }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Index Page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
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
