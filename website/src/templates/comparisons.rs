// use crate::components::comparisons::{get_comparisons, Comparison, FeatureSupport};
use crate::components::container::{Container, ContainerProps};
use perseus::{t, GenericNode, Template};
use std::rc::Rc;
use sycamore::prelude::Template as SycamoreTemplate;
use sycamore::prelude::*;

#[component(ComparisonsPage<G>)]
pub fn comparisons_page() -> SycamoreTemplate<G> {
    template! {
        Container(ContainerProps {
            title: t!("perseus"),
            children: template! {
                div(class = "flex flex-col justify-center text-center dark:text-white mt-14 xs:mt-16 sm:mt-20 lg:mt-25") {
                    div {
                        h1(class = "text-5xl xs:text-7xl sm:text-8xl md:text-9xl p-2 font-extrabold") {
                            "Comparisons"
                        }
                        br()
                        p(class = "text-lg") {
                            "See how Perseus compares to other web development frameworks."
                        }
                        p(class = "italic") {
                            "Is there anything we're missing here? Please "
                            a(href = "https://github.com/arctic-hen7/perseus/issues/new/choose") { "open an issue" }
                            " and let us know!"
                        }
                    }
                    br(class = "mb-24")

                    p(class = "text-xl") {
                        (t!("comparisons-todo"))
                    }
                }
            }
        })
    }
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("comparisons")
        .template(Rc::new(|_| {
            template! {
                ComparisonsPage()
            }
        }))
        .head(Rc::new(|_| {
            template! {
                title { (format!("{} | {}", t!("comparisons-title"), t!("perseus"))) }
            }
        }))
}
