use perseus::prelude::*;
use sycamore::prelude::*;

fn about_page() -> View {
    view! {
        p { (t!("about")) }
        button(id = "switch-button", on:click = move |_| {
            #[cfg(client)]
            Reactor::from_cx().switch_locale("fr-FR");
        }) { "Switch to French" }
    }
}

pub fn get_template() -> Template {
    Template::build("about").view(about_page).build()
}
