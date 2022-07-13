use perseus::{Html, RenderFnResultWithCause, Template};
use sycamore::prelude::{view, Scope, SsrNode, View};

#[perseus::make_rx(PageStateRx)]
struct PageState {
    greeting: String,
}

#[perseus::template_rx]
pub fn index_page<'a, G: Html>(cx: Scope<'a>, state: PageStateRx<'a>) -> View<G> {
    view! { cx,
        p { (state.greeting.get()) }
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Index Page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .template(index_page)
        .head(head)
        .build_state_fn(get_build_state)
        .set_headers_fn(set_headers)
}

#[perseus::build_state]
pub async fn get_build_state(_path: String, _locale: String) -> RenderFnResultWithCause<PageState> {
    Ok(PageState {
        greeting: "Hello World!".to_string(),
    })
}

// For legacy reasons, this takes an `Option<T>`, but, if you're generating
// state, it will always be here In v0.4.0, this will be updated to take just
// your page's state (if it has any) Unfortunately, this return type does have
// to be fully qualified, or you have to import it with a server-only
// target-gate
#[perseus::set_headers]
pub fn set_headers(state: PageState) -> perseus::http::header::HeaderMap {
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
