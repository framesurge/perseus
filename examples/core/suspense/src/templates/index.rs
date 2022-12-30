use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[cfg(target_arch = "wasm32")]
use gloo_timers::future::sleep;
#[cfg(target_arch = "wasm32")]
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "IndexPageStateRx")]
struct IndexPageState {
    #[rx(suspense = "greeting_handler")]
    // For non-nested suspense, we have to wrap our type in a `Result` (anything
    // else won't compile), where the error type can be serialized/deserialized by Serde.
    // In this case, we have something that can't fail, so we use `SerdeInfallible`, an
    // analogue of `std::convert::Infallible` that works with Serde.
    greeting: Result<String, SerdeInfallible>, // This can't fail
    #[rx(nested)]
    #[rx(suspense = "test_handler")]
    // For suspense on nested fields, we have to use `RxResult`, which adds an extra
    // `.get()` layer to enable tracking changes from `Ok -> Err` and vice versa. Using
    // any other wrapper type for nested suspense will not compile.
    test: RxResult<Test, String>, // This can fail
    #[rx(nested)]
    // This is nested, and has lower-level suspended fields, but it itself isn't suspended,
    // which is fine.
    other_test: OtherTest,
}
#[derive(Serialize, Deserialize, ReactiveState, Clone)]
struct Test {
    // In absolutely no case can you ever use suspense here! If the `test` field
    // in `IndexPageState` weren't suspended, it would be fine, but having a suspense
    // inside a suspense will not work! It will compile, but the lower-level handlers
    // *will not execute*!
    second_greeting: String,
}
#[derive(Serialize, Deserialize, Clone, ReactiveState)]
struct OtherTest {
    #[rx(suspense = "other_test_handler")]
    third_greeting: Result<String, SerdeInfallible>,
}

fn index_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a IndexPageStateRx) -> View<G> {
    let greeting = create_memo(cx, || match &*state.greeting.get() {
        Ok(state) => state.to_string(),
        Err(_) => unreachable!(),
    });
    let second_greeting = create_memo(cx, move || match &*state.test.get() {
        // We don't particularly want `Rc<Rc<T>>`, hence this clone (but either will work)
        Ok(test) => (*test.second_greeting.get()).clone(),
        Err(_) => "Error!".to_string(),
    });
    let third_greeting = create_memo(cx, move || match &*state.other_test.third_greeting.get() {
        // We don't particularly want `Rc<Rc<T>>`, hence this clone (but either will work)
        Ok(state) => state.to_string(),
        Err(_) => unreachable!(),
    });

    view! { cx,
        p(id = "first") { (greeting.get()) }
        p(id = "second") { (second_greeting.get()) }
        p(id = "third") { (third_greeting.get()) }
    }
}

// Unfortunately, you can't just return `T` from suspense handlers as you can
// with state generation functions like `get_build_state`, due to constraints
// within the Rust language. Hopefully, this will be one day possible!

// This takes the same reactive scope as `index_page`, along with the individual
// reactive version of the `greeting` field (notice the parallels to
// `index_page`'s signature). This doesn't return any value, it uses Sycamore's
// reactivity system to mutate the greeting directly.
//
// We can do things with the scope here as necessary, but we don't use it in
// this example.
#[browser_only_fn]
async fn greeting_handler<'a>(
    _cx: Scope<'a>,
    greeting: &'a RcSignal<Result<String, SerdeInfallible>>,
) -> Result<(), SerdeInfallible> {
    // Here, we're just waiting for a second before continuing, just to show a delay
    // (and so that Perseus isn't too fast for the tests of this example...)
    sleep(Duration::from_secs(1)).await;
    // This is very simple, but we could easily perform network requests etc. here
    greeting.set(Ok("Hello from the handler!".to_string()));
    Ok(())
}

// This is the handler for nested suspense, so it takes the final reactive
// version of `RxResult`. As `IndexPageStateRx` is to `IndexPageState`,
// `RxResultRef` is to `RxResult`!
#[browser_only_fn]
async fn test_handler<'a>(
    _cx: Scope<'a>,
    test: &'a RxResultRx<Test, String>,
) -> Result<(), String> {
    sleep(Duration::from_secs(1)).await;
    // Unfortunately, this verbosity is necessary until `Try` is stabilized so we
    // can have custom implementations of the `?` operator.
    let test = match &*test.get() {
        Ok(test) => test.clone(),
        Err(err) => return Err(err.clone()),
    };
    test.second_greeting
        .set("Hello again from the handler!".to_string());
    Ok(())
}

#[browser_only_fn]
async fn other_test_handler<'a>(
    _cx: Scope<'a>,
    greeting: &'a RcSignal<Result<String, SerdeInfallible>>,
) -> Result<(), SerdeInfallible> {
    sleep(Duration::from_secs(1)).await;
    // This is very simple, but we could easily perform network requests etc. here
    greeting.set(Ok("Hello again again from the handler!".to_string()));
    Ok(())
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> IndexPageState {
    IndexPageState {
        greeting: Ok("Hello from the server!".to_string()),
        // `RxResult` can be created from a standard `Result` with a simple `.into()`
        test: Ok(Test {
            second_greeting: "Hello again from the server!".to_string(),
        })
        .into(),
        other_test: OtherTest {
            third_greeting: Ok("Hello again again from the server!".to_string()),
        },
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    // Note that suspense handlers are registered through the state, not here
    Template::build("index")
        .view_with_state(index_page)
        .build_state_fn(get_build_state)
        .build()
}
