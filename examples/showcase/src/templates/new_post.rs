use perseus::Template;
use std::rc::Rc;
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};

#[component(NewPostPage<G>)]
pub fn new_post_page() -> SycamoreTemplate<G> {
    template! {
        p { "New post creator." }
    }
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("post/new").template(Rc::new(|_| {
        template! {
            NewPostPage()
        }
    }))
}
