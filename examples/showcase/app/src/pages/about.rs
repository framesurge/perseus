use perseus::template::Template;
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};
use std::sync::Arc;

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
    Arc::new(|_| {
        template! {
            AboutPage()
        }
    })
}
