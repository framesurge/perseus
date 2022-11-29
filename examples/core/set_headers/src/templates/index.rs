use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {
    greeting: String,
}

#[perseus::template]
fn index_page<'a, G: Html>(cx: Scope<'a>, state: PageStateRx<'a>) -> View<G> {
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
        .set_headers_fn(set_headers)
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> RenderFnResultWithCause<PageState> {
    Ok(PageState {
        greeting: "Hello World!".to_string(),
    })
}

// Unfortunately, this return type does have
// to be fully qualified, or you have to import it with a server-only
// target-gate
#[engine_only_fn]
fn set_headers(state: PageState) -> perseus::http::header::HeaderMap {
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
