use perseus::Template;
use sycamore::prelude::{view, Html, Scope, SsrNode, View};

fn index_page<G: Html>(cx: Scope) -> View<G> {
    // We can't preload pages on the engine-side
    #[cfg(target_arch = "wasm32")]
    {
        // Get the render context first, which is the one-stop-shop for everything
        // internal to Perseus in the browser
        let render_ctx = perseus::get_render_ctx!(cx);
        // This spawns a future in the background, and will panic if the page you give
        // doesn't exist (to handle those errors and manage the future, use
        // `.try_preload` instead)
        render_ctx.preload(cx, "about");
    }

    view! { cx,
        p { "Open up your browser's DevTools, go to the network tab, and then click the link below..." }

        a(href = "about") { "About" }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Index Page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_page).head(head)
}
