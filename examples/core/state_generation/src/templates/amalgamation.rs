use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {
    message: String,
}

fn amalgamation_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a PageStateRx) -> View<G> {
    view! { cx,
        p { (format!("The message is: '{}'", state.message.get())) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("amalgamation")
        // We'll generate some state at build time and some more at request time
        .build_state_fn(get_build_state)
        .request_state_fn(get_request_state)
        // But Perseus would usually just override the build state with request
        // state, so we provide a custom function to unify them
        .amalgamate_states_fn(amalgamate_states)
        .view_with_state(amalgamation_page)
        .build()
}

// Could be fallible with a `BlamedError`
#[engine_only_fn]
async fn amalgamate_states(
    // This takes the same information as build state, request state, etc.
    _info: StateGeneratorInfo<()>,
    build_state: PageState,
    req_state: PageState,
) -> PageState {
    PageState {
        message: format!(
            "Hello from the amalgamation! (Build says: '{}', server says: '{}'.)",
            build_state.message, req_state.message
        ),
    }
}

// Could be fallible with a `BlamedError`
#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> PageState {
    PageState {
        message: "Hello from the build process!".to_string(),
    }
}

// Could be fallible
#[engine_only_fn]
async fn get_request_state(_info: StateGeneratorInfo<()>, _req: Request) -> PageState {
    PageState {
        message: "Hello from the server!".to_string(),
    }
}
