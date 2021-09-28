use perseus::{is_server, t, Template};
use std::rc::Rc;
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};

#[component(AboutPage<G>)]
pub fn about_page() -> SycamoreTemplate<G> {
    template! {
        p { (t!("about")) }
        p {
            (
                if is_server!() {
                    "This is running on the server."
                } else {
                    "This is running on the client."
                }
            )
        }
    }
}

pub fn template_fn<G: GenericNode>() -> perseus::template::TemplateFn<G> {
    Rc::new(|_| {
        template! {
            AboutPage()
        }
    })
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("about").template(template_fn())
}
