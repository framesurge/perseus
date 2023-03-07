use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    let counter = create_signal(cx, 0);
    view! { cx,
        (counter.get())
        br () {}
        button (on:click=move |_| {counter.set(*counter.get() - 1)}) { "remove 1"}
        button (on:click=move |_| {counter.set(*counter.get() + 1)}) { "Add 1"}
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
