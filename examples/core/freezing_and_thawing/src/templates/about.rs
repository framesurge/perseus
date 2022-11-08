use perseus::state::Freeze;
use perseus::prelude::*;
use sycamore::prelude::*;

use crate::global_state::AppStateRx;

#[perseus::template_rx]
pub fn about_page<'a, G: Html>(cx: Scope<'a>) -> View<G> {
    // This is not part of our data model, we do NOT want the frozen app
    // synchronized as part of our page's state, it should be separate
    let frozen_app = create_signal(cx, String::new());
    let render_ctx = RenderCtx::from_ctx(cx);

    let global_state = render_ctx.get_global_state::<AppStateRx>(cx);

    view! { cx,
        p(id = "global_state") { (global_state.test.get()) }

        // When the user visits this and then comes back, they'll still be able to see their username (the previous state will be retrieved from the global state automatically)
        a(href = "", id = "index-link") { "Index" }
        br()

        // We'll let the user freeze from here to demonstrate that the frozen state also navigates back to the last route
        button(id = "freeze_button", on:click = |_| {
            frozen_app.set(render_ctx.freeze());
        }) { "Freeze!" }
        p(id = "frozen_app") { (frozen_app.get()) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page)
}
