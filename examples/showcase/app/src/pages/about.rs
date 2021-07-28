use sycamore::prelude::*;
use perseus::page::Page;

#[component(AboutPage<G>)]
pub fn about_page() -> Template<G> {
	template! {
		p { "About." }
	}
}

pub fn get_page<G: GenericNode>() -> Page<(), G> {
    Page::new("about")
        .template(Box::new(|_| template! {
                AboutPage()
            }
        ))
}
