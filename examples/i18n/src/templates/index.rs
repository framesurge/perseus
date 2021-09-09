use perseus::{Template, Translator};
use std::rc::Rc;
use std::sync::Arc;
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};

#[component(IndexPage<G>)]
pub fn index_page(translator: Rc<Translator>) -> SycamoreTemplate<G> {
    // TODO fix multiple translators with some kind of macro
    let translator_1 = Rc::clone(&translator);
    let translator_2 = Rc::clone(&translator);
    template! {
        // TODO switch to `t!` macro
        p { (translator_1.translate("hello", {
            let mut args = fluent_bundle::FluentArgs::new();
            args.set("user", "User");
            Some(args)
        })) }
        a(href = translator_2.url("/about")) { "About" }
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
