use crate::components::container::{Container, ContainerProps};
use crate::components::features_list::get_features_list;
use crate::components::github_svg::GITHUB_SVG;
use perseus::{link, t, Html, Template};
use sycamore::prelude::*;

#[perseus::template(IndexPage)]
#[component(IndexPage<G>)]
pub fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Container(ContainerProps {
            title: t!("perseus", cx),
            children: view! { cx,
                // Splash page
                div(
                    class = "bg-cover h-full flex justify-center items-center text-center dark:text-white",
                    style = "background-image: url(\".perseus/static/svg/splash_page_bg.svg\"), url(\".perseus/static/svg/polygon_scatter.svg\");"
                ) {
                    div {
                        p(class = "text-7xl xs:text-8xl sm:text-9xl p-2 font-extrabold") { (t!("perseus", cx)) }
                        p(class = "text-lg") { (t!("index-caption", cx)) }
                        br()
                        div(class = "flex items-center justify-center") {
                            a(
                                class = "py-3 px-4 m-2 font-semibold rounded-lg shadow-2xl text-white bg-indigo-500 hover:bg-indigo-400 transition-colors duration-200",
                                href = link!("/docs", cx)
                            ) { (t!("index-get-started", cx)) }
                            a(
                                // The difference in y-axis padding is deliberate, it looks better with the ring
                                class = "inline-flex items-center py-2 px-4 m-2 font-semibold rounded-lg shadow-2xl dark:text-white ring-4 ring-indigo-500 hover:ring-indigo-400 transition-all duration-200",
                                href = "https://github.com/arctic-hen7/perseus"
                            ) {
                                span(
                                    class = "mr-1",
                                    dangerously_set_inner_html = GITHUB_SVG
                                )
                                span { (format!(" {}", t!("index-github", cx))) }
                            }
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
                            (t!("index-desc", cx))
                        }
                    }
                    br()
                    // Feature list (static, so we don't need Sycamore iteration)
                    div(class = "flex justify-center") {
                        ul(class = "text-center max-w-7xl mx-2") {
                            (get_features_list(cx))
                        }
                    }
                    br()
                    br()
                    // Workflow explanation
                    div(class = "flex justify-center text-center text-lg w-full p-1 xs:p-4") {
                        div(class = "p-4 shadow-2xl rounded-xl max-w-lg") {
                            p(class = "underline text-2xl mb-2") { (t!("index-workflow.heading", cx)) }
                            ol(class = "list-decimal list-inside") {
                                li {
                                    span {
                                        (t!("index-workflow.step-1", cx))
                                        " "
                                        code { "cargo install perseus-cli" }
                                    }
                                }
                                li {
                                    span(
                                        // We set the HTML because we need a link inside the Fluent translation
                                        dangerously_set_inner_html = &t!("index-workflow.step-2", cx)
                                    )
                                }
                                li {
                                    span {
                                        (t!("index-workflow.step-3", cx))
                                        " "
                                        code { "perseus serve" }
                                    }
                                }
                                li {
                                    span {
                                        (t!("index-workflow.step-4", cx))
                                        " "
                                        code { "perseus deploy" }
                                    }
                                }
                                li {
                                    span {
                                        (t!("index-workflow.step-5", cx))
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
                        p { (t!("index-cta.first", cx)) }
                        p { (t!("index-cta.second", cx)) }
                    }
                    br()
                    div(class = "flex justify-center") {
                        a(
                            class = "py-3 px-4 m-2 text-xl font-semibold rounded-lg shadow-2xl text-white bg-indigo-600 hover:bg-indigo-500 transition-colors duration-200",
                            href = link!("/docs", cx)
                        ) { (t!("index-get-started", cx)) }
                    }
                    br()
                    div(class = "flex justify-center") {
                        a(
                            class = "py-2 px-4 m-2 font-semibold rounded-lg shadow-2xl text-white bg-indigo-500 hover:bg-indigo-400 transition-colors duration-200",
                            href = link!("/comparisons", cx),
                        ) { (t!("index-comparisons", cx)) }
                    }
                }
            }
        })
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { (t!("perseus", cx)) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_page).head(head)
}
