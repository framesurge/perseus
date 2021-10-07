use crate::components::container::{Container, ContainerProps};
use crate::components::features_list::get_features_list;
use crate::components::github_svg::GITHUB_SVG;
use perseus::{link, t, GenericNode, Template};
use std::rc::Rc;
use sycamore::prelude::Template as SycamoreTemplate;
use sycamore::prelude::*;

#[component(IndexPage<G>)]
pub fn index_page() -> SycamoreTemplate<G> {
    template! {
        Container(ContainerProps {
            title: t!("perseus"),
            children: template! {
                // Splash page
                div(
                    class = "bg-cover h-full flex justify-center items-center text-center dark:text-white",
                    style = "background-image: url(\".perseus/static/svg/splash_page_bg.svg\"), url(\".perseus/static/svg/polygon_scatter.svg\");"
                ) {
                    div {
                        p(class = "text-7xl xs:text-8xl sm:text-9xl p-2 font-extrabold") { (t!("perseus")) }
                        p(class = "text-lg") { (t!("index-caption")) }
                        br()
                        a(
                            class = "py-3 px-4 m-2 font-semibold rounded-lg shadow-2xl text-white bg-indigo-500 hover:bg-indigo-400 transition-colors duration-200",
                            href = link!("/docs")
                        ) { (t!("index-get-started")) }
                        a(
                            // The difference in y-axis padding is deliberate, it looks better with the ring
                            class = "py-2 px-4 m-2 font-semibold rounded-lg shadow-2xl dark:text-white ring-4 ring-indigo-500 hover:ring-indigo-400 transition-colors duration-200",
                            href = "https://github.com/arctic-hen7/perseus",
                            // I genuinely have no clue why this works, but it does
                            style = "display: ruby;"
                        ) {
                            span(
                               dangerously_set_inner_html = GITHUB_SVG
                            )
                            span { (format!(" {}", t!("index-github"))) }
                        }
                    }
                }
                div(
                    class = "bg-cover py-4 text-white",
                    style = "background-image: url(\".perseus/static/svg/stacked_waves.svg\");"
                ) {
                    // Brief description
                    div(class = "flex justify-center text-center text-lg w-full") {
                        p(class = "p-1 xs:p-4 max-w-7xl") {
                            (t!("index-desc"))
                        }
                    }
                    br()
                    // Feature list (static, so we don't need Sycamore iteration)
                    div(class = "flex justify-center") {
                        ul(class = "text-center max-w-7xl mx-2") {
                            (get_features_list())
                        }
                    }
                    br()
                    br()
                    // Workflow explanation
                    div(class = "flex justify-center text-center text-lg w-full p-1 xs:p-4") {
                        div(class = "p-4 shadow-2xl rounded-xl max-w-lg") {
                            p(class = "underline text-2xl mb-2") { (t!("index-workflow.heading")) }
                            ol(class = "list-decimal list-inside") {
                                li {
                                    span {
                                        (t!("index-workflow.step-1"))
                                        " "
                                        code { "cargo install perseus-cli" }
                                    }
                                }
                                li {
                                    span(
                                        // We set the HTML because we need a link inside the Fluent translation
                                        dangerously_set_inner_html = &t!("index-workflow.step-2")
                                    )
                                }
                                li {
                                    span {
                                        (t!("index-workflow.step-3"))
                                        " "
                                        code { "perseus serve" }
                                    }
                                }
                                li {
                                    span {
                                        (t!("index-workflow.step-4"))
                                        " "
                                        code { "perseus deploy" }
                                    }
                                }
                                li {
                                    span {
                                        (t!("index-workflow.step-5"))
                                    }
                                }
                            }
                        }
                    }
                }
                // Second CTA
                div(
                    class = "pb-24 flex flex-col justify-center text-white",
                    style = "background: url(\".perseus/static/svg/cta_bg.svg\");background-size:cover;"
                ) {
                    div(class = "text-3xl 2xs:text-4xl xs:text-5xl sm:text-6xl p-2 font-extrabold text-center") {
                        p { (t!("index-cta.first")) }
                        p { (t!("index-cta.second")) }
                    }
                    br()
                    div(class = "flex justify-center") {
                        a(
                            class = "py-3 px-4 m-2 text-xl font-semibold rounded-lg shadow-2xl text-white bg-indigo-600 hover:bg-indigo-500 transition-colors duration-200",
                            href = link!("/docs")
                        ) { (t!("index-get-started")) }
                    }
                    br()
                    div(class = "flex justify-center") {
                        a(
                            class = "py-2 px-4 m-2 font-semibold rounded-lg shadow-2xl text-white bg-indigo-500 hover:bg-indigo-400 transition-colors duration-200",
                            href = link!("/comparisons"),
                        ) { (t!("index-comparisons")) }
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
                title { (t!("perseus")) }
            }
        }))
}
