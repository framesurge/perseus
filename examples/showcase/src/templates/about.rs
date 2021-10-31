use perseus::Template;
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};

#[perseus::template(AboutPage)]
#[component(AboutPage<G>)]
pub fn about_page() -> SycamoreTemplate<G> {
    template! {
        p { "About." }
    }
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("about").template(about_page)
}
