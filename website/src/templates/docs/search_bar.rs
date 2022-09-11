use perseus::t;
use sycamore::prelude::*;

#[component]
pub fn SearchBar<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        input(
            class = "p-2 border rounded-md mb-2 focus:outline-indigo-500 search-bar-bg max-w-full",
            placeholder = t!("search", cx)
        )
    }
}
