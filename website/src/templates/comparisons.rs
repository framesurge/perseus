use crate::components::comparisons::{get_comparisons, Comparison, FeatureSupport};
use crate::components::container::{Container, ContainerProps};
use perseus::{t, GenericNode, Template};
use std::rc::Rc;
use sycamore::prelude::Template as SycamoreTemplate;
use sycamore::prelude::*;

#[component(ComparisonsPage<G>)]
pub fn comparisons_page() -> SycamoreTemplate<G> {
    let comparisons = get_comparisons();
    let comparisons = SycamoreTemplate::new_fragment(
        comparisons.iter().map(move |comparison| {
            let Comparison {
                name,
                supports_ssg,
                supports_ssr,
                supports_ssr_ssg_same_page,
                supports_i18n,
                supports_incremental,
                supports_revalidation,
                inbuilt_cli,
                inbuilt_routing,
                supports_shell,
                supports_deployment,
                supports_exporting,
                language
            } = comparison.clone();
            template! {
                tr {
                    th(class = "text-center border border-black p-2 sticky left-0 z-10 bg-white dark:bg-navy") { (name) }
                    td(class = "text-center border border-black p-2") { (supports_ssg.render()) }
                    td(class = "text-center border border-black p-2") { (supports_ssr.render()) }
                    td(class = "text-center border border-black p-2") { (supports_ssr_ssg_same_page.render()) }
                    td(class = "text-center border border-black p-2") { (supports_i18n.render()) }
                    td(class = "text-center border border-black p-2") { (supports_incremental.render()) }
                    td(class = "text-center border border-black p-2") { (supports_revalidation.render()) } // TODO revalidation type
                    td(class = "text-center border border-black p-2") { (inbuilt_cli.render()) }
                    td(class = "text-center border border-black p-2") { (inbuilt_routing.render()) }
                    td(class = "text-center border border-black p-2") { (supports_shell.render()) }
                    td(class = "text-center border border-black p-2") { (supports_deployment.render()) }
                    td(class = "text-center border border-black p-2") { (supports_exporting.render()) }
                    td(class = "text-center border border-black p-2") { (language) }
                }
            }
        }).collect()
    );
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
                    div(class = "flex justify-center p-2") {
                        // TODO fix up table height
                        table(class = "block overflow-scroll max-h-96 max-w-7xl") {
                            thead(class = "sticky top-0 z-20") {
                                tr {
                                    // The name heading should stay in the same place for any scroll
                                    th(class = "bg-indigo-500 text-white p-1 sticky left-0") {(t!("comparisons-table-headings.name"))}
                                    th(class = "bg-indigo-500 text-white p-1") {(t!("comparisons-table-headings.supports_ssg"))}
                                    th(class = "bg-indigo-500 text-white p-1") {(t!("comparisons-table-headings.supports_ssr"))}
                                    th(class = "bg-indigo-500 text-white p-1") {(t!("comparisons-table-headings.supports_ssr_ssg_same_page"))}
                                    th(class = "bg-indigo-500 text-white p-1") {(t!("comparisons-table-headings.supports_i18n"))}
                                    th(class = "bg-indigo-500 text-white p-1") {(t!("comparisons-table-headings.supports_incremental"))}
                                    th(class = "bg-indigo-500 text-white p-1") {(t!("comparisons-table-headings.revalidation_type"))}
                                    th(class = "bg-indigo-500 text-white p-1") {(t!("comparisons-table-headings.inbuilt_cli"))}
                                    th(class = "bg-indigo-500 text-white p-1") {(t!("comparisons-table-headings.routing_type"))}
                                    th(class = "bg-indigo-500 text-white p-1") {(t!("comparisons-table-headings.supports_shell"))}
                                    th(class = "bg-indigo-500 text-white p-1") {(t!("comparisons-table-headings.supports_deployment"))}
                                    th(class = "bg-indigo-500 text-white p-1") {(t!("comparisons-table-headings.supports_exporting"))}
                                    th(class = "bg-indigo-500 text-white p-1") {(t!("comparisons-table-headings.language"))}
                                }
                                tr {
                                    ({
                                        let Comparison {
                                            name,
                                            supports_ssg,
                                            supports_ssr,
                                            supports_ssr_ssg_same_page,
                                            supports_i18n,
                                            supports_incremental,
                                            supports_revalidation,
                                            inbuilt_cli,
                                            inbuilt_routing,
                                            supports_shell,
                                            supports_deployment,
                                            supports_exporting,
                                            language
                                        } = Comparison {
                                            name: "Perseus".to_string(),
                                            supports_ssg: FeatureSupport::Full,
                                            supports_ssr: FeatureSupport::Full,
                                            supports_ssr_ssg_same_page: FeatureSupport::Full,
                                            supports_i18n: FeatureSupport::Full,
                                            supports_incremental: FeatureSupport::Full,
                                            supports_revalidation: FeatureSupport::Full,
                                            inbuilt_cli: FeatureSupport::Full,
                                            inbuilt_routing: FeatureSupport::Full,
                                            supports_shell: FeatureSupport::Full,
                                            supports_deployment: FeatureSupport::Full,
                                            supports_exporting: FeatureSupport::Full,
                                            language: "Rust + Wasm".to_string()
                                        };
                                        template! {
                                            th(class = "text-center border border-black p-2 sticky left-0 z-10 bg-white dark:bg-navy") { (name) }
                                            td(class = "text-center border border-black p-2 bg-white dark:bg-navy") { (supports_ssg.render()) }
                                            td(class = "text-center border border-black p-2 bg-white dark:bg-navy") { (supports_ssr.render()) }
                                            td(class = "text-center border border-black p-2 bg-white dark:bg-navy") { (supports_ssr_ssg_same_page.render()) }
                                            td(class = "text-center border border-black p-2 bg-white dark:bg-navy") { (supports_i18n.render()) }
                                            td(class = "text-center border border-black p-2 bg-white dark:bg-navy") { (supports_incremental.render()) }
                                            td(class = "text-center border border-black p-2 bg-white dark:bg-navy") { (supports_revalidation.render()) } // TODO revalidation type
                                            td(class = "text-center border border-black p-2 bg-white dark:bg-navy") { (inbuilt_cli.render()) }
                                            td(class = "text-center border border-black p-2 bg-white dark:bg-navy") { (inbuilt_routing.render()) }
                                            td(class = "text-center border border-black p-2 bg-white dark:bg-navy") { (supports_shell.render()) }
                                            td(class = "text-center border border-black p-2 bg-white dark:bg-navy") { (supports_deployment.render()) }
                                            td(class = "text-center border border-black p-2 bg-white dark:bg-navy") { (supports_exporting.render()) }
                                            td(class = "text-center border border-black p-2 bg-white dark:bg-navy") { (language) }
                                        }
                                    })
                                }
                            }
                            (comparisons)
                        }
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
