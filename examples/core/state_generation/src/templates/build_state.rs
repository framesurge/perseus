use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {
    greeting: String,
}

#[perseus::template]
fn build_state_page<'a, G: Html>(cx: Scope<'a>, state: PageStateRx<'a>) -> View<G> {
    view! { cx,
        p { (state.greeting.get()) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("build_state")
        .build_state_fn(get_build_state)
        .template_with_state(build_state_page)
}

// We're told the path we're generating for (useless unless we're using build
// paths as well) and the locale (which will be `xx-XX` if we're not using i18n)
// Note that this function is asynchronous, so we can do work like fetching from
// a server or the like here (see the `demo/fetching` example), along with any
// helper state we generated with build paths (which we aren't using, hence the
// `()`)
#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> RenderFnResultWithCause<PageState> {
    Ok(PageState {
        greeting: "Hello World!".to_string(),
    })
}
