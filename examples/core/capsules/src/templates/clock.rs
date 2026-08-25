use crate::capsules::links::LINKS;
use crate::capsules::time::TIME;
use perseus::prelude::*;
use sycamore::prelude::*;

fn clock_page() -> View {
    // Nothing's wrong with preparing a widget in advance, especially if you want to
    // use the same one in a few places (this will avoid unnecessary fetches in
    // some cases, see the book for details)
    let time = TIME.widget("", ());

    view! {
        p {
            "The most recent update to the time puts it at "
            (time)
        }
        (LINKS.widget("", ()))
    }
}

pub fn get_template() -> Template {
    Template::build("clock")
        .view(clock_page)
        // See `about.rs` for an explanation of this
        .allow_rescheduling()
        .build()
}
