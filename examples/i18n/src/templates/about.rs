use perseus::{is_server, t, Template};
use sycamore::prelude::{component, view, Html, View};

#[perseus::template(AboutPage)]
#[component(AboutPage<G>)]
pub fn about_page() -> View<G> {
    view! {
        p { (t!("about")) }
        p {
            (
                if is_server!() {
                    "This is running on the server."
                } else {
                    "This is running on the client."
                }
            )
        }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page)
}
