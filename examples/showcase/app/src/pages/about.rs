use sycamore::prelude::*;
use crate::page::Page;

#[component(AboutPage<G>)]
pub fn about_page() -> Template<G> {
	template! {
		p { "About." }
	}
}

pub fn get_page() -> Page<()> {
    Page::new("about")
        .template(Box::new(|_| template! {
                AboutPage()
            }
        ))
}
