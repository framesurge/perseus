use crate::components::comparisons::{render_lighthouse_score, Comparison, RawComparison};
use crate::components::container::Container;
use crate::components::header::HeaderProps;
use crate::components::info_svg::INFO_SVG;
use perseus::{t, ErrorCause, GenericErrorWithCause, Html, RenderFnResultWithCause, Template};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use sycamore::prelude::*;

struct ComparisonRowProps<'a> {
    perseus_val: String,
    comparison_val: &'a ReadSignal<String>,
    name: String,
}
#[component(ComparisonRow<G>)]
fn ComparisonRow<'a, G: Html>(cx: Scope<'a>, props: ComparisonRowProps<'a>) -> View<G> {
    let show_details = create_signal(cx, false);
    let name = create_ref(cx, props.name);

    view! { cx,
        tr {
            th(class = "text-left p-1 py-2 text-xs xs:text-base") {
                div(class = "flex items-center") {
                    (t!(&format!("comparisons-table-headings.{}", name), cx))
                    span(
                        class = "ml-1",
                        on:click = |_| {
                            show_details.set(!*show_details.get())
                        },
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
                    (t!(&format!("comparisons-table-details.{}", name), cx))
                }
            }
            td(class = "p-1 py-2 text-xs xs:text-base") {
                (props.perseus_val)
            }
            // The only thing that could overflow is the comparison language (everything else is tested)
            // Anything longer than 15 characters will overflow (by testing on smallest supported screen -- iPhone 5)
            td(class = "p-1 py-2 text-xs xs:text-base break-words xs:break-normal") {
                (props.comparison_val.get())
            }
        }
    }
}

struct ComparisonTableProps<'a> {
    comparison: &'a ReadSignal<Comparison>,
    perseus_comparison: Comparison,
}
#[component(ComparisonTable<G>)]
fn ComparisonTable<'a, G: Html>(cx: Scope<'a>, props: ComparisonTableProps<'a>) -> View<G> {
    let comparison = props.comparison;
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
        text: _, // The Perseus comparison has no text
    } = props.perseus_comparison;

    let show_details_homepage_lighthouse_desktop = create_signal(cx, false);
    let show_details_homepage_lighthouse_mobile = create_signal(cx, false);

    // We now need to deconstruct the comparison with memos (actual pain)
    // Otherwise, the props passed through to the row component aren't considered
    // reactive
    let comparison_language = create_memo(cx, || comparison.get().language.to_string());
    let comparison_supports_ssg = create_memo(cx, || comparison.get().supports_ssg.render());
    let comparison_supports_ssr = create_memo(cx, || comparison.get().supports_ssr.render());
    let comparison_supports_ssr_ssg_same_page =
        create_memo(cx, || comparison.get().supports_ssr_ssg_same_page.render());
    let comparison_supports_i18n = create_memo(cx, || comparison.get().supports_i18n.render());
    let comparison_supports_incremental =
        create_memo(cx, || comparison.get().supports_incremental.render());
    let comparison_supports_revalidation =
        create_memo(cx, || comparison.get().supports_revalidation.render());
    let comparison_inbuilt_cli = create_memo(cx, || comparison.get().inbuilt_cli.render());
    let comparison_inbuilt_routing = create_memo(cx, || comparison.get().inbuilt_routing.render());
    let comparison_supports_shell = create_memo(cx, || comparison.get().supports_shell.render());
    let comparison_supports_deployment =
        create_memo(cx, || comparison.get().supports_deployment.render());
    let comparison_supports_exporting =
        create_memo(cx, || comparison.get().supports_exporting.render());
    let comparison_text = create_memo(cx, || comparison.get().text.to_string());

    view! { cx,
        table(class = "w-full overflow-x-scroll table-fixed border-collapse") {
            thead(class = "mt-4 text-white bg-indigo-500 rounded-xl") {
                th(class = "p-1 py-2 text-xs xs:text-base") {
                    (t!("comparisons-table-header", cx))
                }
                th(class = "p-1 py-2 text-xs xs:text-base") {
                    (t!("perseus", cx))
                }
                th(class = "p-1 py-2 text-xs xs:text-base") {
                    (comparison.get().name)
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
                            (t!("comparisons-table-headings.homepage_lighthouse_desktop", cx))
                            span(
                                class = "ml-1",
                                on:click = |_| {
                                    show_details_homepage_lighthouse_desktop.set(!*show_details_homepage_lighthouse_desktop.get())
                                },
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
                            (t!("comparisons-table-details.homepage_lighthouse_desktop", cx))
                        }
                    }
                    td(class = "p-1 py-2 text-xs xs:text-base") {
                        (render_lighthouse_score(cx, perseus_homepage_lighthouse_desktop))
                    }
                    td(class = "p-1 py-2 text-xs xs:text-base") {
                        (render_lighthouse_score(cx, comparison.get().homepage_lighthouse_desktop))
                    }
                }
                tr {
                    th(class = "text-left p-1 py-2 text-xs xs:text-base") {
                        div(class = "flex items-center") {
                            (t!("comparisons-table-headings.homepage_lighthouse_mobile", cx))
                            span(
                                class = "ml-1",
                                on:click = |_| {
                                    show_details_homepage_lighthouse_mobile.set(!*show_details_homepage_lighthouse_mobile.get())
                                },
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
                            (t!("comparisons-table-details.homepage_lighthouse_mobile", cx))
                        }
                    }
                    td(class = "p-1 py-2 text-xs xs:text-base") {
                        (render_lighthouse_score(cx, perseus_homepage_lighthouse_mobile))
                    }
                    td(class = "p-1 py-2 text-xs xs:text-base") {
                        (render_lighthouse_score(cx, comparison.get().homepage_lighthouse_mobile))
                    }
                }
            }
        }
        h3(class = "text-2xl underline") { (t!(
            "comparisons-unknown-heading",
            {
                "name" = &comparison.get().name
            },
            cx
        )) }
        div(class = "w-full flex justify-center") {
            p(class = "max-w-prose") { (comparison_text.get()) }
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
pub fn comparisons_page<G: Html>(cx: Scope, props: ComparisonsPageProps) -> View<G> {
    let comparisons = props.comparisons.clone();
    let perseus_comparison = props.perseus_comparison;
    let mut comparison_names: Vec<String> = comparisons.keys().cloned().collect();
    comparison_names.sort();
    // The current comparison should be the first element in the list alphabetically
    let curr_comparison_name = create_signal(cx, comparison_names[0].clone());

    let select_options = View::new_fragment(
        comparison_names
            .iter()
            .map(|name| {
                let name = name.clone();
                let name_2 = name.clone();
                view! { cx,
                    option(value = name) {
                        (name_2)
                    }
                }
            })
            .collect(),
    );

    let curr_comparison = create_memo(cx, move || {
        comparisons
            .get(&*curr_comparison_name.get())
            .unwrap()
            .clone()
    });

    view! { cx,
        Container {
            header: HeaderProps {
                title: t!("perseus", cx),
                text_color: "text-black".to_string(),
                menu_color: "bg-black".to_string(),
                mobile_nav_extension: View::empty(),
                menu_open: None,
            },
            footer: true,
            children: view! { cx,
                div(class = "flex flex-col justify-center text-center dark:text-white mt-14 xs:mt-16 sm:mt-20 lg:mt-25") {
                    div {
                        h1(class = "text-5xl xs:text-7xl sm:text-8xl font-extrabold") {
                            (t!("comparisons-heading", cx))
                        }
                        br()
                        p(class = "text-lg") {
                            (t!("comparisons-subtitle", cx))
                        }
                        p(
                            class = "italic px-1",
                            dangerously_set_inner_html = &t!("comparisons-extra", cx)
                        )
                    }
                    br(class = "mb-2 sm:mb-16 md:mb-24")
                    div(class = "p-1") {
                        select(
                            class = "p-1 rounded-sm dark:bg-navy mb-4",
                            on:input = |event: web_sys::Event| {
                                use wasm_bindgen::JsCast;
                                let target: web_sys::HtmlInputElement = event.target().unwrap().unchecked_into();
                                let new_comparison_name = target.value();
                                curr_comparison_name.set(new_comparison_name);
                            }
                        ) {
                            (select_options)
                        }
                        br()
                        div(class = "px-3 w-full sm:mr-auto sm:ml-auto sm:max-w-prose lg:max-w-3xl xl:max-w-4xl 2xl:max-w-5xl") {
                            div(class = "flex justify-center flex-col") {
                                ComparisonTable(ComparisonTableProps {
                                    comparison: curr_comparison,
                                    perseus_comparison
                                })
                            }
                        }
                        br(class = "mb-1 sm:mb-8 md:mb-12")
                        h3(class = "text-xl underline") { (t!("comparisons-sycamore-heading", cx)) }
                        div(class = "w-full flex justify-center text-sm") {
                            p(class = "max-w-prose") { (t!("comparisons-sycamore-text", cx)) }
                        }
                    }
                }
            }
        }
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { (format!("{} | {}", t!("comparisons-title", cx), t!("perseus", cx))) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("comparisons")
        .template(comparisons_page)
        .head(head)
        .build_state_fn(get_build_state)
}

#[perseus::build_state]
pub async fn get_build_state(
    _path: String,
    locale: String,
) -> RenderFnResultWithCause<ComparisonsPageProps> {
    use walkdir::WalkDir;

    // Get all the comparisons from JSON
    // This includes the special properties for Perseus itself
    let mut perseus_comparison: Option<Comparison> = None;
    let mut comparisons: HashMap<String, Comparison> = HashMap::new();

    // Get the `comparisons/` directory in `website`
    // This can have any file structure we want for organization, we just want the
    // files
    let comparisons_dir = PathBuf::from("comparisons");
    // Loop through it
    for entry in WalkDir::new(comparisons_dir) {
        let entry = entry?;
        let path = entry.path();
        // Ignore any empty directories or the like
        if path.is_file() {
            // There shouldn't be any non-Unicode comparison files
            let path_str = path.to_str().unwrap();
            let contents = fs::read_to_string(&path)?;
            // If the file is `perseus.json`, we'll add this to a special variable,
            // otherwise it gets added to the generic map
            if path_str.ends_with("perseus.json") {
                // The Perseus comparison has no localized text
                let comparison = serde_json::from_str::<Comparison>(&contents)?;
                perseus_comparison = Some(comparison);
            } else {
                // Other comparisons have multiple comparison paragraphs, one
                // for each locale (we have to choose the right one)
                let raw_comparison = serde_json::from_str::<RawComparison>(&contents)?;
                let comparison_text = match raw_comparison.text.get(&locale) {
                    Some(text) => text.to_string(),
                    None => {
                        return Err(GenericErrorWithCause {
                            error: format!(
                            "comparison {} does not have localized comparison text for locale {}",
                            raw_comparison.name, locale
                        )
                            .into(),
                            cause: ErrorCause::Server(None),
                        })
                    }
                };
                let comparison = Comparison {
                    name: raw_comparison.name,
                    supports_ssg: raw_comparison.supports_ssg,
                    supports_ssr: raw_comparison.supports_ssr,
                    supports_ssr_ssg_same_page: raw_comparison.supports_ssr_ssg_same_page,
                    supports_i18n: raw_comparison.supports_i18n,
                    supports_incremental: raw_comparison.supports_incremental,
                    supports_revalidation: raw_comparison.supports_revalidation,
                    inbuilt_cli: raw_comparison.inbuilt_cli,
                    inbuilt_routing: raw_comparison.inbuilt_routing,
                    supports_shell: raw_comparison.supports_shell,
                    supports_deployment: raw_comparison.supports_deployment,
                    supports_exporting: raw_comparison.supports_exporting,
                    language: raw_comparison.language,
                    // Ours are 100 and 95, respectively
                    homepage_lighthouse_desktop: raw_comparison.homepage_lighthouse_desktop,
                    homepage_lighthouse_mobile: raw_comparison.homepage_lighthouse_mobile,
                    text: comparison_text,
                };
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
