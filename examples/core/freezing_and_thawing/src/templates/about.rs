use perseus::prelude::*;
use sycamore::prelude::*;

use crate::global_state::AppStateRx;

fn about_page() -> View {
    // This is not part of our data model, we do NOT want the frozen app
    // synchronized as part of our page's state, it should be separate
    let frozen_app = create_signal(String::new());
    let render_ctx = Reactor::from_cx();

    let global_state = render_ctx.get_global_state::<AppStateRx>();

    // Clone for the closure (required for 'static lifetime in Sycamore 0.9.2)
    let frozen_app_clone = frozen_app.clone();
    let render_ctx_clone = render_ctx.clone();

    view! {
        p(id = "global_state") { (global_state.test.get_clone()) }

        // When the user visits this and then comes back, they'll still be able to see their username (the previous state will be retrieved from the global state automatically)
        a(href = "", id = "index-link") { "Index" }
        br()

        // We'll let the user freeze from here to demonstrate that the frozen state also navigates back to the last route
        button(id = "freeze_button", on:click = move |_| {
            #[cfg(client)]
            {
                use perseus::state::Freeze;
                frozen_app_clone.set(render_ctx_clone.freeze());
            }
        }) { "Freeze!" }
        p(id = "frozen_app") { (frozen_app.get_clone()) }
    }
}

pub fn get_template() -> Template {
    Template::build("about").view(about_page).build()
}
