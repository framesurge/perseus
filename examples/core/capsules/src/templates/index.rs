use crate::capsules::greeting::GreetingProps;
use crate::capsules::links::LINKS;
use crate::capsules::wrapper::WRAPPER;
use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page() -> View {
    view! {
        p { "Hello World!" }
        // This capsule wraps another capsule
        (WRAPPER.widget( "", GreetingProps { color: "red".to_string() }))

        // This is not the prettiest function call, deliberately, to encourage you
        // to make this sort of thing part of the template it's used in, or to use
        // a Sycamore component instead (which, for a navbar, we should, this is
        // just an example)
        (LINKS.widget( "", ()))
    }
}

#[engine_only_fn]
fn head() -> View {
    view! {
        title { "Index Page" }
    }
}

pub fn get_template() -> Template {
    Template::build("index").view(index_page).head(head).build()
}
