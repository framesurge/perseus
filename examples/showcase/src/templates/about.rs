use perseus::Template;
use sycamore::prelude::{component, view, Html, View};

#[perseus::template(AboutPage)]
#[component(AboutPage<G>)]
pub fn about_page() -> View<G> {
    view! {
        p { "About." }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page)
}
