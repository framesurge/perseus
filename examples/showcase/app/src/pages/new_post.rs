use perseus::template::Template;
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};

#[component(NewPostPage<G>)]
pub fn new_post_page() -> SycamoreTemplate<G> {
    template! {
        p { "New post creator." }
    }
}

pub fn get_page<G: GenericNode>() -> Template<G> {
    Template::new("post/new").template(template_fn())
}

pub fn template_fn<G: GenericNode>() -> perseus::template::TemplateFn<G> {
    Box::new(|_| {
        template! {
            NewPostPage()
        }
    })
}
