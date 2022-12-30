use lazy_static::lazy_static;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

lazy_static! {
    pub static ref TIME: Capsule<PerseusNodeType, ()> = get_capsule();
}

// Note the use of props as `()`, indicating that this capsule doesn't take any
// properties
#[auto_scope]
fn time_capsule<G: Html>(cx: Scope, state: &TimeStateRx, _props: ()) -> View<G> {
    view! { cx,
        // We'll put this inside a `p`, so we'll use a `span`
        span(id = "time") { (state.time.get()) }
    }
}

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "TimeStateRx")]
struct TimeState {
    time: String,
}

pub fn get_capsule<G: Html>() -> Capsule<G, ()> {
    Capsule::build(
        Template::build("time")
            .build_state_fn(get_build_state)
            // This setup means, ever five seconds, `should_revalidate` will be executed to check if
            // the capsule should really revalidate. If it should (which, since that function always
            // returns `true`, it always should), `get_build_state` will be re-executed.
            .should_revalidate_fn(should_revalidate)
            .revalidate_after("5s"),
    )
    .empty_fallback()
    // Very importantly, we declare our views on the capsule, **not** the template!
    // This lets us use properties.
    .view_with_state(time_capsule)
    .build()
}

// This will get the system time when the app was built
#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> TimeState {
    TimeState {
        time: format!("{:?}", std::time::SystemTime::now()),
    }
}

#[engine_only_fn]
async fn should_revalidate(
    // This takes the same arguments as request state
    _info: StateGeneratorInfo<()>,
    _req: perseus::Request,
) -> bool {
    // For simplicity's sake, this will always say we should revalidate, but you
    // could make this check any condition
    true
}
