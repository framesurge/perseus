use perseus::Template;
use std::rc::Rc;
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};

#[component(AboutPage<G>)]
pub fn about_page() -> SycamoreTemplate<G> {
    template! {
        p { "About." }
    }
}

pub fn get_page<G: GenericNode>() -> Template<G> {
    Template::new("about").template(template_fn())
}

pub fn template_fn<G: GenericNode>() -> perseus::template::TemplateFn<G> {
    Rc::new(|_| {
        template! {
            AboutPage()
        }
    })
}
