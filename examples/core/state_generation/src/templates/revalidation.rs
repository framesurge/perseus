use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {
    time: String,
}

fn revalidation_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a PageStateRx) -> View<G> {
    view! { cx,
        p { (format!("The time when this page was last rendered was '{}'.", state.time.get())) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("revalidation")
        .view_with_state(revalidation_page)
        // This page will revalidate every five seconds (and so the time displayed will be updated)
        .revalidate_after("5s")
        // This is an alternative method of revalidation that uses logic, which will be executed
        // every time a user tries to load this page. For that reason, this should NOT do
        // long-running work, as requests will be delayed. If both this
        // and `revalidate_after()` are provided, this logic will only run when `revalidate_after()`
        // tells Perseus that it should revalidate.
        .should_revalidate_fn(should_revalidate)
        .build_state_fn(get_build_state)
        .build()
}

// This will get the system time when the app was built
#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> PageState {
    PageState {
        time: format!("{:?}", std::time::SystemTime::now()),
    }
}

// This will run every time `.revalidate_after()` permits the page to be
// revalidated This acts as a secondary check, and can perform arbitrary logic
// to check if we should actually revalidate a page.
//
// Since this takes the request, this uses a `BlamedError` if it's fallible.
#[engine_only_fn]
async fn should_revalidate(
    // This takes the same arguments as request state
    _info: StateGeneratorInfo<()>,
    _req: perseus::Request,
) -> Result<bool, BlamedError<std::convert::Infallible>> {
    // For simplicity's sake, this will always say we should revalidate, but you
    // could make this check any condition
    Ok(true)
}
