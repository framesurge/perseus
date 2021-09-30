use crate::components::container::{Container, ContainerProps};
use crate::components::github_svg::GITHUB_SVG;
use perseus::{link, t, GenericNode, Template};
use std::rc::Rc;
use sycamore::prelude::Template as SycamoreTemplate;
use sycamore::prelude::*;

#[component(IndexPage<G>)]
pub fn index_page() -> SycamoreTemplate<G> {
    template! {
        Container(ContainerProps {
            title: "Perseus".to_string(),
            children: template! {
                div(
                    class = "bg-waves dark:bg-waves-dark h-full flex justify-center items-center text-center dark:text-white"
                ) {
                    div {
                        p(class = "text-7xl xs:text-8xl sm:text-9xl p-2 font-extrabold") { "Perseus" }
                        p(class = "text-lg") { "The Rust framework for the modern web." }
                        br()
                        a(
                            class = "py-3 px-4 m-2 font-semibold rounded-lg shadow-2xl text-white bg-fuchsia-500 hover:bg-fuchsia-400 transition-colors duration-200",
                            href = link!("/docs")
                        ) { "Get Started" }
                        a(
                            // TODO try block display here
                            // The difference in y-axis padding is deliberate, it looks better with the ring
                            class = "py-2 px-4 m-2 font-semibold rounded-lg shadow-2xl dark:text-white ring-4 ring-fuchsia-500 hover:ring-fuchsia-400 transition-colors duration-200",
                            href = "https://github.com/arctic-hen7/perseus",
                            // I genuinely have no clue why this works, but it does
                            style = "display: ruby;"
                        ) {
                            span(
                               dangerously_set_inner_html = GITHUB_SVG
                            )
                            span { " GitHub" }
                        }
                    }
                }
            }
        })
    }
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("index")
        .template(Rc::new(|_| {
            template! {
                IndexPage()
            }
        }))
        .head(Rc::new(|_| {
            template! {
                title { "Perseus" }
                link(
                    rel = "preconnect",
                    href = "https://fonts.googleapis.com"
                )
                link(
                    rel = "preconnect",
                    href = "https://fonts.gstatic.com"
                )
                link(
                    rel = "stylesheet",
                    href = "https://fonts.googleapis.com/css2?family=Comfortaa:wght@600&display=swap"
                )
            }
        }))
}
