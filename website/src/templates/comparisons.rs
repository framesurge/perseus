use crate::components::comparisons::{render_lighthouse_score, Comparison};
use crate::components::container::{Container, ContainerProps};
use crate::components::info_svg::INFO_SVG;
use perseus::{t, ErrorCause, GenericErrorWithCause, Html, RenderFnResultWithCause, Template};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use sycamore::prelude::*;
use walkdir::WalkDir;
use wasm_bindgen::JsCast;

struct ComparisonRowProps {
    perseus_val: String,
    comparison_val: ReadSignal<String>,
    name: String,
}
#[component(ComparisonRow<G>)]
fn comparison_row(props: ComparisonRowProps) -> View<G> {
    let ComparisonRowProps {
        perseus_val,
        comparison_val,
        name,
    } = props;
    let name_2 = name.clone();
    let show_details = Signal::new(false);

    view! {
        tr {
            th(class = "text-left p-1 py-2 text-xs xs:text-base") {
                div(class = "flex items-center") {
                    (t!(&format!("comparisons-table-headings.{}", name)))
                    span(
                        class = "ml-1",
                        on:click = cloned!((show_details) => move |_| {
                            show_details.set(!*show_details.get())
                        }),
                        dangerously_set_inner_html = INFO_SVG
                    )
                }
                p(
                    class = format!(
                        "italic font-normal {}",
                        if *show_details.get() {
                            "visible"
                        } else {
                            "hidden"
                        }
                    )
                ) {
                    (t!(&format!("comparisons-table-details.{}", name_2)))
                }
            }
            td(class = "p-1 py-2 text-xs xs:text-base") {
                (perseus_val)
            }
            // The only thing that could overflow is the comparison language (everything else is tested)
            // Anything longer than 15 characters will overflow (by testing on smallest supported screen -- iPhone 5)
            td(class = "p-1 py-2 text-xs xs:text-base break-words xs:break-normal") {
                (comparison_val.get())
            }
        }
    }
}

struct ComparisonTableProps {
    comparison: ReadSignal<Comparison>,
    perseus_comparison: Comparison,
}
#[component(ComparisonTable<G>)]
fn comparison_table(props: ComparisonTableProps) -> View<G> {
    let comparison = props.comparison.clone();
    let Comparison {
        name: _perseus_name, // We'll use the translation ID
        supports_ssg: perseus_supports_ssg,
        supports_ssr: perseus_supports_ssr,
        supports_ssr_ssg_same_page: perseus_supports_ssr_ssg_same_page,
        supports_i18n: perseus_supports_i18n,
        supports_incremental: perseus_supports_incremental,
        supports_revalidation: perseus_supports_revalidation,
        inbuilt_cli: perseus_inbuilt_cli,
        inbuilt_routing: perseus_inbuilt_routing,
        supports_shell: perseus_supports_shell,
        supports_deployment: perseus_supports_deployment,
        supports_exporting: perseus_supports_exporting,
        language: perseus_language,
        homepage_lighthouse_desktop: perseus_homepage_lighthouse_desktop,
        homepage_lighthouse_mobile: perseus_homepage_lighthouse_mobile,
    } = props.perseus_comparison;
    // We now need to deconstruct the comparison with memos (actual pain)
    let comparison_name =
        create_memo(cloned!((comparison) => move || comparison.get().name.clone()));
    let comparison_supports_ssg = create_memo(
        cloned!((comparison) => move || comparison.get().supports_ssg.clone().render()),
    );
    let comparison_supports_ssr = create_memo(
        cloned!((comparison) => move || comparison.get().supports_ssr.clone().render()),
    );
    let comparison_supports_ssr_ssg_same_page = create_memo(
        cloned!((comparison) => move || comparison.get().supports_ssr_ssg_same_page.clone().render()),
    );
    let comparison_supports_i18n = create_memo(
        cloned!((comparison) => move || comparison.get().supports_i18n.clone().render()),
    );
    let comparison_supports_incremental = create_memo(
        cloned!((comparison) => move || comparison.get().supports_incremental.clone().render()),
    );
    let comparison_supports_revalidation = create_memo(
        cloned!((comparison) => move || comparison.get().supports_revalidation.clone().render()),
    );
    let comparison_inbuilt_cli =
        create_memo(cloned!((comparison) => move || comparison.get().inbuilt_cli.clone().render()));
    let comparison_inbuilt_routing = create_memo(
        cloned!((comparison) => move || comparison.get().inbuilt_routing.clone().render()),
    );
    let comparison_supports_shell = create_memo(
        cloned!((comparison) => move || comparison.get().supports_shell.clone().render()),
    );
    let comparison_supports_deployment = create_memo(
        cloned!((comparison) => move || comparison.get().supports_deployment.clone().render()),
    );
    let comparison_supports_exporting = create_memo(
        cloned!((comparison) => move || comparison.get().supports_exporting.clone().render()),
    );
    let comparison_language =
        create_memo(cloned!((comparison) => move || comparison.get().language.clone()));
    let comparison_homepage_lighthouse_desktop =
        create_memo(cloned!((comparison) => move || comparison.get().homepage_lighthouse_desktop));
    let comparison_homepage_lighthouse_mobile =
        create_memo(cloned!((comparison) => move || comparison.get().homepage_lighthouse_mobile));

    let show_details_homepage_lighthouse_desktop = Signal::new(false);
    let show_details_homepage_lighthouse_mobile = Signal::new(false);

    view! {
        table(class = "w-full overflow-x-scroll table-fixed border-collapse") {
            thead(class = "mt-4 text-white bg-indigo-500 rounded-xl") {
                th(class = "p-1 py-2 text-xs xs:text-base") {
                    (t!("comparisons-table-header"))
                }
                th(class = "p-1 py-2 text-xs xs:text-base") {
                    (t!("perseus"))
                }
                th(class = "p-1 py-2 text-xs xs:text-base") {
                    (comparison_name.get())
                }
            }
            tbody {
                // One row for each comparison point
                // One heading explaining it
                // Then two cells, one Perseus, and the for the comparison
                ComparisonRow(ComparisonRowProps {
                    perseus_val: perseus_language,
                    comparison_val: comparison_language,
                    name: "language".to_string()
                })
                ComparisonRow(ComparisonRowProps {
                    perseus_val: perseus_supports_ssg.render(),
                    comparison_val: comparison_supports_ssg,
                    name: "supports_ssg".to_string()
                })
                ComparisonRow(ComparisonRowProps {
                    perseus_val: perseus_supports_ssr.render(),
                    comparison_val: comparison_supports_ssr,
                    name: "supports_ssr".to_string()
                })
                ComparisonRow(ComparisonRowProps {
                    perseus_val: perseus_supports_ssr_ssg_same_page.render(),
                    comparison_val: comparison_supports_ssr_ssg_same_page,
                    name: "supports_ssr_ssg_same_page".to_string()
                })
                ComparisonRow(ComparisonRowProps {
                    perseus_val: perseus_supports_i18n.render(),
                    comparison_val: comparison_supports_i18n,
                    name: "supports_i18n".to_string()
                })
                ComparisonRow(ComparisonRowProps {
                    perseus_val: perseus_supports_incremental.render(),
                    comparison_val: comparison_supports_incremental,
                    name: "supports_incremental".to_string()
                })
                ComparisonRow(ComparisonRowProps {
                    perseus_val: perseus_supports_revalidation.render(),
                    comparison_val: comparison_supports_revalidation,
                    name: "supports_revalidation".to_string()
                })
                ComparisonRow(ComparisonRowProps {
                    perseus_val: perseus_inbuilt_cli.render(),
                    comparison_val: comparison_inbuilt_cli,
                    name: "inbuilt_cli".to_string()
                })
                ComparisonRow(ComparisonRowProps {
                    perseus_val: perseus_inbuilt_routing.render(),
                    comparison_val: comparison_inbuilt_routing,
                    name: "inbuilt_routing".to_string()
                })
                ComparisonRow(ComparisonRowProps {
                    perseus_val: perseus_supports_shell.render(),
                    comparison_val: comparison_supports_shell,
                    name: "supports_shell".to_string()
                })
                ComparisonRow(ComparisonRowProps {
                    perseus_val: perseus_supports_deployment.render(),
                    comparison_val: comparison_supports_deployment,
                    name: "supports_deployment".to_string()
                })
                ComparisonRow(ComparisonRowProps {
                    perseus_val: perseus_supports_exporting.render(),
                    comparison_val: comparison_supports_exporting,
                    name: "supports_exporting".to_string()
                })
                // These last two get special rendering for text colors and possible emoji
                tr {
                    th(class = "text-left p-1 py-2 text-xs xs:text-base") {
                        div(class = "flex items-center") {
                            (t!("comparisons-table-headings.homepage_lighthouse_desktop"))
                            span(
                                class = "ml-1",
                                on:click = cloned!((show_details_homepage_lighthouse_desktop) => move |_| {
                                    show_details_homepage_lighthouse_desktop.set(!*show_details_homepage_lighthouse_desktop.get())
                                }),
                                dangerously_set_inner_html = INFO_SVG
                            )
                        }
                        p(
                            class = format!(
                                "italic font-normal {}",
                                if *show_details_homepage_lighthouse_desktop.get() {
                                    "visible"
                                } else {
                                    "hidden"
                                }
                            )
                        ) {
                            (t!("comparisons-table-details.homepage_lighthouse_desktop"))
                        }
                    }
                    td(class = "p-1 py-2 text-xs xs:text-base") {
                        (render_lighthouse_score(perseus_homepage_lighthouse_desktop))
                    }
                    td(class = "p-1 py-2 text-xs xs:text-base") {
                        (render_lighthouse_score(*comparison_homepage_lighthouse_desktop.get()))
                    }
                }
                tr {
                    th(class = "text-left p-1 py-2 text-xs xs:text-base") {
                        div(class = "flex items-center") {
                            (t!("comparisons-table-headings.homepage_lighthouse_mobile"))
                            span(
                                class = "ml-1",
                                on:click = cloned!((show_details_homepage_lighthouse_mobile) => move |_| {
                                    show_details_homepage_lighthouse_mobile.set(!*show_details_homepage_lighthouse_mobile.get())
                                }),
                                dangerously_set_inner_html = INFO_SVG
                            )
                        }
                        p(
                            class = format!(
                                "italic font-normal {}",
                                if *show_details_homepage_lighthouse_mobile.get() {
                                    "visible"
                                } else {
                                    "hidden"
                                }
                            )
                        ) {
                            (t!("comparisons-table-details.homepage_lighthouse_mobile"))
                        }
                    }
                    td(class = "p-1 py-2 text-xs xs:text-base") {
                        (render_lighthouse_score(perseus_homepage_lighthouse_mobile))
                    }
                    td(class = "p-1 py-2 text-xs xs:text-base") {
                        (render_lighthouse_score(*comparison_homepage_lighthouse_mobile.get()))
                    }
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ComparisonsPageProps {
    pub comparisons: HashMap<String, Comparison>,
    /// The comparison data for Perseus itself.
    pub perseus_comparison: Comparison,
}

#[perseus::template(ComparisonsPage)]
#[component(ComparisonsPage<G>)]
pub fn comparisons_page(props: ComparisonsPageProps) -> View<G> {
    let comparisons = props.comparisons.clone();
    let perseus_comparison = props.perseus_comparison;
    let mut comparison_names: Vec<String> = comparisons.keys().cloned().collect();
    comparison_names.sort();
    // The current comparison should be the first element in the list alphabetically
    let curr_comparison_name = Signal::new(comparison_names[0].clone());

    let select_options = View::new_fragment(
        comparison_names
            .iter()
            .map(|name| {
                let name = name.clone();
                let name_2 = name.clone();
                view! {
                    option(value = name) {
                        (name_2)
                    }
                }
            })
            .collect(),
    );

    let curr_comparison = create_memo(cloned!((curr_comparison_name, comparisons) => move || {
        comparisons.get(&*curr_comparison_name.get()).unwrap().clone()
    }));

    view! {
        Container(ContainerProps {
            title: t!("perseus"),
            children: view! {
                div(class = "flex flex-col justify-center text-center dark:text-white mt-14 xs:mt-16 sm:mt-20 lg:mt-25") {
                    div {
                        h1(class = "text-5xl xs:text-7xl sm:text-8xl font-extrabold") {
                            (t!("comparisons-heading"))
                        }
                        br()
                        p(class = "text-lg") {
                            (t!("comparisons-subtitle"))
                        }
                        p(
                            class = "italic px-1",
                            dangerously_set_inner_html = &t!("comparisons-extra")
                        )
                    }
                    br(class = "mb-2 sm:mb-16 md:mb-24")
                    div(class = "p-1") {
                        select(
                            class = "p-1 rounded-sm dark:bg-navy mb-4",
                            on:input = cloned!((curr_comparison_name) => move |event: web_sys::Event| {
                                let target: web_sys::HtmlInputElement = event.target().unwrap().unchecked_into();
                                let new_comparison_name = target.value();
                                curr_comparison_name.set(new_comparison_name);
                            })
                        ) {
                            (select_options)
                        }
                        br()
                        div(class = "px-3 w-full sm:mr-auto sm:ml-auto sm:max-w-prose lg:max-w-3xl xl:max-w-4xl 2xl:max-w-5xl") {
                            div(class = "flex justify-center") {
                                ComparisonTable(ComparisonTableProps {
                                    comparison: curr_comparison,
                                    perseus_comparison
                                })
                            }
                        }
                        br(class = "mb-1 sm:mb-8 md:mb-12")
                        h3(class = "text-2xl underline") { (t!("comparisons-sycamore-heading")) }
                        div(class = "w-full flex justify-center") {
                            p(class = "max-w-prose") { (t!("comparisons-sycamore-text")) }
                        }
                    }
                }
            }
        })
    }
}

#[perseus::head]
pub fn head() -> View<SsrNode> {
    view! {
        title { (format!("{} | {}", t!("comparisons-title"), t!("perseus"))) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("comparisons")
        .template(comparisons_page)
        .head(head)
        .build_state_fn(get_build_state)
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<ComparisonsPageProps> {
    // Get all the comparisons from JSON
    // This includes the special properties for Perseus itself
    let mut perseus_comparison: Option<Comparison> = None;
    let mut comparisons: HashMap<String, Comparison> = HashMap::new();

    // Get the `comparisons/` directory in `website` (relative to `.perseus/`)
    // This can have any file structure we want for organization, we just want the files
    let comparisons_dir = PathBuf::from("../comparisons");
    // Loop through it
    for entry in WalkDir::new(comparisons_dir) {
        let entry = entry?;
        let path = entry.path();
        // Ignore any empty directories or the like
        if path.is_file() {
            // There shouldn't be any non-Unicode comparison files
            let path_str = path.to_str().unwrap();
            // Get the JSON contents and parse them as a comparison
            let contents = fs::read_to_string(&path)?;
            let comparison = serde_json::from_str::<Comparison>(&contents)?;
            // If the file is `perseus.json`, we'll add this to a special variable, otherwise it gets added to the generic map
            if path_str.ends_with("perseus.json") {
                perseus_comparison = Some(comparison);
            } else {
                comparisons.insert(comparison.name.clone(), comparison);
            }
        }
    }

    let props = ComparisonsPageProps {
        comparisons,
        perseus_comparison: match perseus_comparison {
            Some(perseus_comparison) => perseus_comparison,
            None => return Err(GenericErrorWithCause {
                error: "perseus comparison data not recorded, please ensure `comparisons/perseus.json` exists".into(),
                cause: ErrorCause::Server(None)
            })
        }
    };
    Ok(props)
}
