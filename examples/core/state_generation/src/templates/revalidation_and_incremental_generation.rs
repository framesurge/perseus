// This page exists mostly for testing revalidation together with incremental
// generation (because the two work in complex ways together)

use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {
    time: String,
}

fn revalidation_and_incremental_generation_page<'a, G: Html>(
    cx: BoundedScope<'_, 'a>,
    state: &'a PageStateRx,
) -> View<G> {
    view! { cx,
        p { (format!("The time when this page was last rendered was '{}'.", state.time.get())) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("revalidation_and_incremental_generation")
        .view_with_state(revalidation_and_incremental_generation_page)
        // This page will revalidate every five seconds (and so the time displayed will be updated)
        .revalidate_after(Duration::new(5, 0))
        // This is an alternative method of revalidation that uses logic, which will be executed
        // every time a user tries to load this page. For that reason, this should NOT do
        // long-running work, as requests will be delayed. If both this
        // and `revalidate_after()` are provided, this logic will only run when `revalidate_after()`
        // tells Perseus that it should revalidate.
        .should_revalidate_fn(should_revalidate)
        .build_state_fn(get_build_state)
        .build_paths_fn(get_build_paths)
        // WARNING: this will revalidate on every reload in development, because incremental
        // generation is recalculated on every request in development
        .incremental_generation()
        .build()
}

// This will get the system time when the app was built
#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> PageState {
    PageState {
        time: format!("{:?}", std::time::SystemTime::now()),
    }
}

#[engine_only_fn]
async fn get_build_paths() -> BuildPaths {
    BuildPaths {
        paths: vec!["test".to_string(), "blah/test/blah".to_string()],
        extra: ().into(),
    }
}

// This will run every time `.revalidate_after()` permits the page to be
// revalidated This acts as a secondary check, and can perform arbitrary logic
// to check if we should actually revalidate a page
#[engine_only_fn]
async fn should_revalidate(_info: StateGeneratorInfo<()>, _req: perseus::Request) -> bool {
    // For simplicity's sake, this will always say we should revalidate, but you
    // could make this check any condition
    true
}
