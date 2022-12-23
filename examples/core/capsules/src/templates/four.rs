use crate::capsules::links::LINKS;
use crate::capsules::number::NUMBER;
use perseus::prelude::*;
use sycamore::prelude::*;

fn four_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p {
            "The number four: "
            // We're using the second argument to provide a *widget path* within the capsule
            (NUMBER.widget(cx, "/4", ()))
            "."
        }
        (LINKS.widget(cx, "", ()))
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    // Notice that this doesn't need to have rescheduling, because the widget it
    // uses was built at build-time as part of `number`'s `get_build_paths`
    // function.
    Template::build("four").view(four_page).build()
}
