use lazy_static::lazy_static;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

// Putting our capsule in a static means it can easily be included in templates!
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

// This needs to be public, because it will be passed to us by templates
#[derive(Clone)]
pub struct GreetingProps {
    pub color: String,
}

pub fn get_capsule<G: Html>() -> Capsule<G, GreetingProps> {
    // Template properties, to do with state generation, are set on a template
    // that's passed to the capsule. Note that we don't call `.build()` on the
    // template, because we want a capsule, not a template (we're using the
    // `TemplateInner`).
    Capsule::build(Template::build("greeting").build_state_fn(get_build_state))
        .empty_fallback()
        // Very importantly, we declare our views on the capsule, **not** the template!
        // This lets us use properties.
        .view_with_state(greeting_capsule)
        .build()
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> GreetingState {
    GreetingState {
        greeting: "Hello world! (I'm a widget!)".to_string(),
    }
}
