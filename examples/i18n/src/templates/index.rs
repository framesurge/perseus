use perseus::{t, Template, Translator};
use std::rc::Rc;
use std::sync::Arc;
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};

#[component(IndexPage<G>)]
pub fn index_page(translator: Rc<Translator>) -> SycamoreTemplate<G> {
    template! {
        // TODO switch to `t!` macro
        p { (translator.translate("hello")) }
        a(href = "/en-US/about") { "About" }
    }
}

pub fn template_fn<G: GenericNode>() -> perseus::template::TemplateFn<G> {
    Arc::new(|_, translator: Rc<Translator>| {
        template! {
            IndexPage(translator)
        }
    })
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("index").template(template_fn())
}
