use perseus::prelude::*;
use sycamore::prelude::*;

fn about_page() -> View {
    view! {
        p { "Try going back to the index page, and the state should still be the same!" }

        // Note: Using on:click with navigate() because sycamore-router doesn't attach click handlers to dynamic views.
        // The href is kept for SEO (crawlers will see the link).
        a(href = "/", id = "index-link", on:click = |ev: web_sys::MouseEvent| {
            ev.prevent_default();
            navigate("/");
        }) { "Index" }
    }
}

pub fn get_template() -> Template {
    Template::build("about").view(about_page).build()
}
