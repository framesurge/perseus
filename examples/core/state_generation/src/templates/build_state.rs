use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {
    greeting: String,
}

fn build_state_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a PageStateRx) -> View<G> {
    view! { cx,
        p { (state.greeting.get()) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("build_state")
        .build_state_fn(get_build_state)
        .view_with_state(build_state_page)
        .build()
}

// We're told the path we're generating for (useless unless we're using build
// paths as well) and the locale (which will be `xx-XX` if we're not using i18n)
// Note that this function is asynchronous, so we can do work like fetching from
// a server or the like here (see the `demo/fetching` example), along with any
// helper state we generated with build paths (which we aren't using, hence the
// `()`)
//
// This returns a `Result` with a `BlamedError`, because, if we were using
// *incremental generation*, then build state might be executed again in future
// (see `incremental_generation.rs` for an example of that).
#[engine_only_fn]
async fn get_build_state(
    _info: StateGeneratorInfo<()>,
) -> Result<PageState, BlamedError<std::io::Error>> {
    Ok(PageState {
        greeting: "Hello World!".to_string(),
    })
}
