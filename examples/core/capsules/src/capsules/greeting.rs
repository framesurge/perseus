use lazy_static::lazy_static;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[cfg(target_arch = "wasm32")]
lazy_static! {
    pub static ref GREETING: Capsule<BrowserNodeType, GreetingProps> = get_capsule();
}
#[cfg(not(target_arch = "wasm32"))]
lazy_static! {
    pub static ref GREETING: Capsule<SsrNode, GreetingProps> = get_capsule();
}

fn greeting_capsule<'a, 'b, G: Html>(
    cx: BoundedScope<'a, 'b>,
    state: &'b GreetingStateRx,
    props: GreetingProps,
) -> View<G> {
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
    Capsule::new(Template::new("greeting").build_state_fn(get_build_state))
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
