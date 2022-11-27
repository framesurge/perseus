use perseus::prelude::*;
use sycamore::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "IndexPageStateRx")]
struct IndexPageState {
    #[rx(suspense = "greeting_handler")]
    greeting: Result<String, String>, // TODO Infallible
}

#[template]
fn index_page<'a, G: Html>(cx: Scope<'a>, state: IndexPageStateRx<'a>) -> View<G> {
    let greeting = create_memo(cx, || match &*state.greeting.get() {
        Ok(state) => state.to_string(),
        Err(_) => unreachable!(),
    });

    // perseus::state::compute_suspense(cx, state.greeting.clone(), greeting_handler(cx, create_ref(cx, state.greeting.clone())));

    view! { cx,
        p { (greeting.get()) }
    }
}

// This takes the same reactive scope as `index_page`, along with the individual reactive version
// of the `greeting` field (notice the parallels to `index_page`'s signature).
// This doesn't return any value, it uses Sycamore's reactivity system to mutate the greeting
// directly.
#[browser_only_fn]
async fn greeting_handler<'a>(_cx: Scope<'a>, greeting: &'a RcSignal<Result<String, String>>) -> Result<(), String> {
    // This is very simple, but we could easily perform network requests etc. here
    greeting.set(Ok("Hello from the handler!".to_string()));
    Ok(())
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> RenderFnResultWithCause<IndexPageState> {
    Ok(IndexPageState {
        greeting: Ok("Hello from the server!".to_string()),
    })
}

pub fn get_template<G: Html>() -> Template<G> {
    // Note that suspense handlers are registered through the state, not here
    Template::new("index")
        .template_with_state(index_page)
        .build_state_fn(get_build_state)
}
