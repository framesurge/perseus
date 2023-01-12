use perseus::prelude::*;
use sycamore::prelude::*;

fn about_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { (t!(cx, "about")) }
        button(on:click = move |_| {
            #[cfg(client)]
            Reactor::<G>::from_cx(cx).switch_locale("fr-FR");
        }) { "Switch to French" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("about").view(about_page).build()
}
