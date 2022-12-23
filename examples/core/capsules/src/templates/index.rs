use crate::capsules::greeting::GreetingProps;
use crate::capsules::wrapper::WRAPPER;
use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "Hello World!" }
        a(href = "about") { "About" }
        // This capsule wraps another capsule
        (WRAPPER.widget(cx, "", GreetingProps { color: "red".to_string() }))
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Index Page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}
