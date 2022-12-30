use crate::capsules::links::LINKS;
use crate::capsules::time::TIME;
use perseus::prelude::*;
use sycamore::prelude::*;

fn clock_page<G: Html>(cx: Scope) -> View<G> {
    // Nothing's wrong with preparing a widget in advance, especially if you want to
    // use the same one in a few places (this will avoid unnecessary fetches in
    // some cases, see the book for details)
    let time = TIME.widget(cx, "", ());

    view! { cx,
        p {
            "The most recent update to the time puts it at "
            (time)
        }
        (LINKS.widget(cx, "", ()))
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("clock")
        .view(clock_page)
        // See `about.rs` for an explanation of this
        .allow_rescheduling()
        .build()
}
