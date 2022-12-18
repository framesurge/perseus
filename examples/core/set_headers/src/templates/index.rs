use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {
    greeting: String,
}

fn index_page<'a, 'b, G: Html>(cx: BoundedScope<'a, 'b>, state: &'b PageStateRx) -> View<G> {
    view! { cx,
        p { (state.greeting.get()) }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Index Page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .template_with_state(index_page)
        .head(head)
        .build_state_fn(get_build_state)
        // There is also `.set_headers()`, which takes a function that does not use the page state
        .set_headers_with_state(set_headers)
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> PageState {
    PageState {
        greeting: "Hello World!".to_string(),
    }
}

// Unfortunately, this return type does have
// to be fully qualified, or you have to import it with a server-only
// target-gate.
//
// This function takes a a scope so it can get at a `Reactor`, which will have
// the global state and, potentially, a translator. This can allow you to create
// localized headers.
#[engine_only_fn]
fn set_headers(_cx: Scope, state: PageState) -> perseus::http::header::HeaderMap {
    // These imports are only available on the server-side, which this function is
    // automatically gated to
    use perseus::http::header::{HeaderMap, HeaderName};

    let mut map = HeaderMap::new();
    map.insert(
        HeaderName::from_lowercase(b"x-greeting").unwrap(),
        state.greeting.parse().unwrap(),
    );
    map
}
