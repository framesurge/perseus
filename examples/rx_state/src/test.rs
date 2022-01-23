use perseus::{Html, RenderFnResultWithCause, Template};
use sycamore::prelude::*;

// We define a normal `struct` and then use `make_rx` (which derives `Serialize`, `Deserialize`, and `Clone` automatically)
// This will generate a new `struct` called `IndexPropsRx` (as we asked it to), in which every field is made reactive with a `Signal`
#[perseus::make_rx(TestPropsRx)]
pub struct TestProps {
    pub username: String,
}

// This special macro (normally we'd use `template(IndexProps)`) converts the state we generate elsewhere to a reactive version
#[perseus::template_rx(IndexPage)]
pub fn test_page(TestPropsRx { username }: TestPropsRx) -> View<G> {
    let username_2 = username.clone(); // This is necessary until Sycamore's new reactive primitives are released

    view! {
        p { (format!("Greetings, {}!", username.get())) }
        input(bind:value = username_2, placeholder = "Username")

        // When the user visits this and then comes back, they'll still be able to see their username (the previous state will be retrieved from the global state automatically)
        a(href = "about") { "About" }
        a(href = "") { "Index" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("test")
        .build_state_fn(get_build_state)
        .template(test_page)
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(_path: String, _locale: String) -> RenderFnResultWithCause<TestProps> {
    Ok(TestProps {
        username: "".to_string(),
    })
}
