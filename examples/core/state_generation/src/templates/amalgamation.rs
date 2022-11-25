#[cfg(not(target_arch = "wasm32"))]
use perseus::Request;
use perseus::{RenderFnResultWithCause, Template};
use sycamore::prelude::{view, Html, Scope, View};

#[perseus::make_rx(PageStateRx)]
pub struct PageState {
    pub message: String,
}

#[perseus::template]
pub fn amalgamation_page<'a, G: Html>(cx: Scope<'a>, state: PageStateRx<'a>) -> View<G> {
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
        .template(amalgamation_page)
}

#[perseus::amalgamate_states]
pub async fn amalgamate_states(
    _path: String,
    _locale: String,
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

#[perseus::build_state]
pub async fn get_build_state(_path: String, _locale: String) -> RenderFnResultWithCause<PageState> {
    Ok(PageState {
        message: "Hello from the build process!".to_string(),
    })
}

#[perseus::request_state]
pub async fn get_request_state(
    _path: String,
    _locale: String,
    _req: Request,
) -> RenderFnResultWithCause<PageState> {
    Ok(PageState {
        message: "Hello from the server!".to_string(),
    })
}
