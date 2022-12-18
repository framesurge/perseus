use perseus::prelude::*;
use sycamore::prelude::*;

fn about_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { (t!("about", cx)) }
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
    Template::new("about").template(about_page).build()
}
