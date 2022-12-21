use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    // Deliberate panic to show how panic handling works (in an `on_mount` so we still reach the right checkpoints for testing)
    #[cfg(target_arch = "wasm32")]
    on_mount(cx, || {
        panic!();
    });

    view! { cx,
        p { "Hello World!" }
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
