use perseus::prelude::*;
use sycamore::prelude::*;

#[template]
fn greeting_capsule<'a, G: Html>(cx: Scope<'a>, state: GreetingCapsuleStateRx<'a>) -> View<G> {
    view! { cx,
        p { (state.greeting.get()) }
    }
}

#[derive(Serialize, Deserialize, ReactiveState)]
struct GreetingCapsuleState {
    greeting: String,
}

pub fn get_capsule<G: Html>() -> Capsule<G> {
    Capsule::new("greeting")
        .template_with_state(greeting_capsule)
        .build_state_fn(get_build_state)
}

fn get_build_state(_info: StateGeneratorInfo<()>) -> RenderFnResultWithCause<GreetingCapsuleState> {
    Ok(GreetingCapsuleState {
        greeting: "Hello world! (I'm a widget!)".to_string(),
    })
}
