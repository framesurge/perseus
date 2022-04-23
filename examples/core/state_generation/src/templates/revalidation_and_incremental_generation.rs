// This page exists mostly for testing revalidation together with incremental generation (because the two work in complex
// ways together)

use perseus::{RenderFnResult, RenderFnResultWithCause, Template};
use sycamore::prelude::{view, Html, Scope, View};

#[perseus::make_rx(PageStateRx)]
pub struct PageState {
    pub time: String,
}

#[perseus::template_rx]
pub fn revalidation_and_incremental_generation_page<G: Html>(
    cx: Scope,
    state: PageStateRx,
) -> View<G> {
    view! { cx,
        p { (format!("The time when this page was last rendered was '{}'.", state.time.get())) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("revalidation_and_incremental_generation")
        .template(revalidation_and_incremental_generation_page)
        // This page will revalidate every five seconds (and so the time displayed will be updated)
        .revalidate_after("5s".to_string())
        // This is an alternative method of revalidation that uses logic, which will be executed every itme a user tries to
        // load this page. For that reason, this should NOT do long-running work, as requests will be delayed. If both this
        // and `revaldiate_after()` are provided, this logic will only run when `revalidate_after()` tells Perseus
        // that it should revalidate.
        .should_revalidate_fn(|| async { Ok(true) })
        .build_state_fn(get_build_state)
        .build_paths_fn(get_build_paths)
        .incremental_generation()
}

// This will get the system time when the app was built
#[perseus::autoserde(build_state)]
pub async fn get_build_state(_path: String, _locale: String) -> RenderFnResultWithCause<PageState> {
    Ok(PageState {
        time: format!("{:?}", std::time::SystemTime::now()),
    })
}

pub async fn get_build_paths() -> RenderFnResult<Vec<String>> {
    Ok(vec!["test".to_string(), "blah/test/blah".to_string()])
}
