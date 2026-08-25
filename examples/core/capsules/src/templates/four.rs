use crate::capsules::links::LINKS;
use crate::capsules::number::NUMBER;
use perseus::prelude::*;
use sycamore::prelude::*;

fn four_page() -> View {
    view! {
        p(id = "four") {
            "The number four: "
            // We're using the second argument to provide a *widget path* within the capsule
            (NUMBER.widget( "/4", ()))
            "."
        }
        (LINKS.widget("", ()))
    }
}

pub fn get_template() -> Template {
    // Notice that this doesn't need to have rescheduling, because the widget it
    // uses was built at build-time as part of `number`'s `get_build_paths`
    // function.
    Template::build("four").view(four_page).build()
}
