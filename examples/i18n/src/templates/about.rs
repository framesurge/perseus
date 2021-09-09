use perseus::{Template, Translator};
use std::rc::Rc;
use std::sync::Arc;
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};

#[component(AboutPage<G>)]
pub fn about_page(translator: Rc<Translator>) -> SycamoreTemplate<G> {
    template! {
        // TODO switch to `t!` macro
        p { (translator.translate("about", None)) }
    }
}

pub fn template_fn<G: GenericNode>() -> perseus::template::TemplateFn<G> {
    Arc::new(|_, translator: Rc<Translator>| {
        template! {
            AboutPage(translator)
        }
    })
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("about").template(template_fn())
}
