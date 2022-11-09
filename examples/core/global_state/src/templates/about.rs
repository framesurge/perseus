use perseus::prelude::*;
use sycamore::prelude::*;

use crate::global_state::AppStateRx;

#[perseus::template]
pub fn about_page<'a, G: Html>(cx: Scope<'a>) -> View<G> {
    let global_state = RenderCtx::from_ctx(cx).get_global_state::<AppStateRx>(cx);

    view! { cx,
        // The user can change the global state through an input, and the changes they make will be reflected throughout the app
        p { (global_state.test.get()) }
        input(bind:value = global_state.test)

        a(href = "") { "Index" }
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "About Page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page).head(head)
}
