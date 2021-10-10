use perseus::Template;
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};

#[component(AboutPage<G>)]
pub fn about_page() -> SycamoreTemplate<G> {
    template! {
        p { "About." }
    }
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("about")
        .template(|_| {
            template! {
                AboutPage()
            }
        })
        .head(|_| {
            template! {
                title { "About Page | Perseus Example – Basic" }
            }
        })
}
