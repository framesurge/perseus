use crate::capsules::greeting::{GreetingProps, GREETING};
use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "Hello World!" }
        (GREETING.widget(cx, "", GreetingProps { color: "red".to_string() }))
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Index Page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").view(index_page).head(head).build()
}
