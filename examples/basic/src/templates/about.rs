use perseus::Template;
use std::rc::Rc;
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};

#[component(AboutPage<G>)]
pub fn about_page() -> SycamoreTemplate<G> {
    template! {
        p { "About." }
    }
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("about")
        .template(Rc::new(|_| {
            template! {
                AboutPage()
            }
        }))
        .head(Rc::new(|_| {
            template! {
                title { "About Page | Perseus Example – Basic" }
            }
        }))
}
