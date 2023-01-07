use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    // We can't preload pages on the engine-side
    #[cfg(client)]
    {
        // Get the reactor first, which is the one-stop-shop for everything
        // internal to Perseus in the browser
        let reactor = Reactor::<G>::from_cx(cx);
        // This spawns a future in the background, and will panic if the page you give
        // doesn't exist (to handle those errors and manage the future, use
        // `.try_preload` instead).
        //
        // Note that there is no `link!` macro here, and preloading is expressly
        // disallowed across locales (i.e. you can only preload things in the
        // current locale). This is to prevent unnecessary translations
        // requests, which can be quite heavy.
        reactor.preload(cx, "about");
    }

    view! { cx,
        p { (t!(cx, "index-msg")) }

        a(id = "about", href = link!(cx, "about")) { (t!(cx, "index-about-link")) }
        a(id = "fr-about", href = "fr-FR/about") { "About (French)" }
        a(id = "en-about", href = "en-US/about") { "About (English)" }
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
