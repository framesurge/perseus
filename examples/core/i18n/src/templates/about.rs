use perseus::{t, Template};
use sycamore::prelude::{view, Html, View};

#[perseus::template_rx]
pub fn about_page() -> View<G> {
    view! {
        p { (t!("about")) }
        p {
            (
                if !G::IS_BROWSER {
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
