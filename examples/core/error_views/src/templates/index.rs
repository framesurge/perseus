use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page() -> View {
    // Deliberate panic to show how panic handling works (in an `on_mount` so we
    // still reach the right checkpoints for testing)
    #[cfg(client)]
    on_mount(cx, || {
        panic!();
    });

    view! {
        p { "Hello World!" }
    }
}

#[engine_only_fn]
fn head() -> View {
    view! {
        title { "Index Page" }
    }
}

pub fn get_template() -> Template {
    Template::build("index").view(index_page).head(head).build()
}
