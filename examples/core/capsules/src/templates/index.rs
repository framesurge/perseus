use crate::capsules::greeting::GreetingProps;
use crate::capsules::links::LINKS;
use crate::capsules::wrapper::WRAPPER;
use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "Hello World!" }
        // This capsule wraps another capsule
        (WRAPPER.widget(cx, "", GreetingProps { color: "red".to_string() }))

        // This is not the prettiest function call, deliberately, to encourage you
        // to make this sort of thing part of the template it's used in, or to use
        // a Sycamore component instead (which, for a navbar, we should, this is
        // just an example)
        (LINKS.widget(cx, "", ()))
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
