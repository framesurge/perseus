use lazy_static::lazy_static;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

lazy_static! {
    // This `PerseusNodeType` alias will resolve to `SsrNode`/`DomNode`/`HydrateNode` automatically
    // as needed. This is needed because `lazy_static!` doesn't support generics, like `G: Html`.
    // Perseus can bridge the gap internally with type coercions, so this "just works"!
    pub static ref GREETING: Capsule<PerseusNodeType, GreetingProps> = get_capsule();
}

#[auto_scope]
fn greeting_capsule<G: Html>(cx: Scope, state: &GreetingStateRx, props: GreetingProps) -> View<G> {
    view! { cx,
        p(style = format!("color: {};", props.color)) { (state.greeting.get()) }
    }
}

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "GreetingStateRx")]
struct GreetingState {
    greeting: String,
}

#[derive(Clone)]
pub struct GreetingProps {
    pub color: String,
}

pub fn get_capsule<G: Html>() -> Capsule<G, GreetingProps> {
    Capsule::build(Template::build("greeting").build_state_fn(get_build_state))
        // This method is on `CapsuleInner`, and must be called before the others...
        .empty_fallback()
        .view_with_state(greeting_capsule)
        .build()
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> GreetingState {
    GreetingState {
        greeting: "Hello world! (I'm a widget!)".to_string(),
    }
}
