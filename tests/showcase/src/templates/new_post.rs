use perseus::Template;
use sycamore::prelude::{component, view, Html, View};

#[perseus::template(NewPostPage)]
#[component(NewPostPage<G>)]
pub fn new_post_page() -> View<G> {
    view! {
        p { "New post creator." }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("post/new").template(new_post_page)
}
