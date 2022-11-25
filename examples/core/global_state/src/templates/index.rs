use perseus::prelude::*;
use sycamore::prelude::*;

use crate::global_state::AppStateRx;

// Note that this template takes no state of its own in this example, but it
// certainly could
#[perseus::template]
pub fn index_page<'a, G: Html>(cx: Scope<'a>) -> View<G> {
    // We access the global state through the render context, extracted from
    // Sycamore's context system
    let global_state = RenderCtx::from_ctx(cx).get_global_state::<AppStateRx>(cx);

    view! { cx,
        // The user can change the global state through an input, and the changes they make will be reflected throughout the app
        p { (global_state.test.get()) }
        input(bind:value = global_state.test)

        a(href = "about", id = "about-link") { "About" }
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Index Page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_page).head(head)
}
