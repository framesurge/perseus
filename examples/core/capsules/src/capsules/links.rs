use lazy_static::lazy_static;
use perseus::prelude::*;
use sycamore::prelude::*;

// There are a fair few pages in this example, so this serves as a little
// navigation bar. (You could easily do this with a normal Sycamore component,
// and that would probably make more sense, but this is a capsules example!)

lazy_static! {
    pub static ref LINKS: Capsule<PerseusNodeType, ()> = get_capsule();
}

fn links_capsule<G: Html>(cx: Scope, _: ()) -> View<G> {
    view! { cx,
        div(id = "links", style = "margin-top: 1rem;") {
            a(id = "index-link", href = "") { "Index" }
            br {}
            a(id = "about-link", href = "about") { "About" }
            br {}
            a(id = "clock-link", href = "clock") { "Clock" }
            br {}
            a(id = "four-link", href = "four") { "4" }
            br {}
            a(id = "calc-link", href = "calc") { "Calc" }
        }
    }
}

pub fn get_capsule<G: Html>() -> Capsule<G, ()> {
    Capsule::build(Template::build("links"))
        .empty_fallback()
        .view(links_capsule)
        .build()
}
