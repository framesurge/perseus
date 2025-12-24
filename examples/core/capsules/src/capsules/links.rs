use lazy_static::lazy_static;
use perseus::prelude::*;
use sycamore::prelude::*;

// There are a fair few pages in this example, so this serves as a little
// navigation bar. (You could easily do this with a normal Sycamore component,
// and that would probably make more sense, but this is a capsules example!)

lazy_static! {
    pub static ref LINKS: Capsule<()> = get_capsule();
}

fn links_capsule(_: ()) -> View {
    view! {
        div(id = "links", style = "margin-top: 1rem;") {
            Link(to = "/", id = "index-link") { "Index" }
            br {}
            Link(to = "/about", id = "about-link") { "About" }
            br {}
            Link(to = "/clock", id = "clock-link") { "Clock" }
            br {}
            Link(to = "/four", id = "four-link") { "4" }
            br {}
            Link(to = "/calc", id = "calc-link") { "Calc" }
        }
    }
}

pub fn get_capsule() -> Capsule<()> {
    Capsule::build(Template::build("links"))
        .empty_fallback()
        .view(links_capsule)
        .build()
}
