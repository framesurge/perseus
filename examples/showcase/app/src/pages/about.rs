use sycamore::prelude::{template, component, GenericNode, Template as SycamoreTemplate};
use perseus::template::Template;

#[component(AboutPage<G>)]
pub fn about_page() -> SycamoreTemplate<G> {
	template! {
		p { "About." }
	}
}

pub fn get_page<G: GenericNode>() -> Template<G> {
    Template::new("about")
        .template(template_fn())
}

pub fn template_fn<G: GenericNode>() -> perseus::template::TemplateFn<G> {
    Box::new(|_| template! {
            AboutPage()
        }
    )
}