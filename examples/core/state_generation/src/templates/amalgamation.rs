use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {
    message: String,
}

#[perseus::template]
fn amalgamation_page<'a, G: Html>(cx: Scope<'a>, state: PageStateRx<'a>) -> View<G> {
    view! { cx,
        p { (format!("The message is: '{}'", state.message.get())) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("amalgamation")
        // We'll generate some state at build time and some more at request time
        .build_state_fn(get_build_state)
        .request_state_fn(get_request_state)
        // But Perseus doesn't know which one to use, so we provide a function to unify them
        .amalgamate_states_fn(amalgamate_states)
        .template_with_state(amalgamation_page)
}

async fn amalgamate_states(
    // This takes the same information as build state, request state, etc.
    _info: StateGeneratorInfo<()>,
    build_state: PageState,
    req_state: PageState,
) -> RenderFnResultWithCause<PageState> {
    Ok(PageState {
        message: format!(
            "Hello from the amalgamation! (Build says: '{}', server says: '{}'.)",
            build_state.message, req_state.message
        ),
    })
}

async fn get_build_state(_info: StateGeneratorInfo<()>) -> RenderFnResultWithCause<PageState> {
    Ok(PageState {
        message: "Hello from the build process!".to_string(),
    })
}

async fn get_request_state(
    _info: StateGeneratorInfo<()>,
    _req: Request,
) -> RenderFnResultWithCause<PageState> {
    Ok(PageState {
        message: "Hello from the server!".to_string(),
    })
}
