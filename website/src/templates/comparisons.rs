#[cfg(engine)]
use crate::components::comparisons::RawComparison;
use crate::components::comparisons::{render_lighthouse_score, Comparison};
use crate::components::container::Container;
use crate::components::header::HeaderProps;
use crate::components::info_svg::INFO_SVG;
#[cfg(engine)]
use crate::Error;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(engine)]
use std::fs;
#[cfg(engine)]
use std::path::PathBuf;
use sycamore::prelude::*;

#[derive(Props)]
struct ComparisonRowProps {
    perseus_val: String,
    comparison_val: ReadSignal<String>,
    name: String,
}
#[component]
fn ComparisonRow(props: ComparisonRowProps) -> View {
    let show_details = create_signal(false);
    // In Sycamore 0.9.2, we can use the string directly
    let name = props.name.clone();
    let heading_key = format!("comparisons-table-headings.{}", name);
    let details_key = format!("comparisons-table-details.{}", name);

    view! {
        tr {
            th(class = "text-left p-1 py-2 text-xs xs:text-base") {
                div(class = "flex items-center") {
                    (t!( &heading_key))
                    span(
                        class = "ml-1",
                        on:click = move |_| {
                            show_details.set(!show_details.get())
                        },
                        dangerously_set_inner_html = INFO_SVG
                    )
                }
                p(
                    class = format!(
                        "italic font-normal {}",
                        if show_details.get() {
                            "visible"
                        } else {
                            "hidden"
                        }
                    )
                ) {
                    (t!( &details_key))
                }
            }
            td(class = "p-1 py-2 text-xs xs:text-base") {
                (props.perseus_val)
            }
            // The only thing that could overflow is the comparison language (everything else is tested)
            // Anything longer than 15 characters will overflow (by testing on smallest supported screen -- iPhone 5)
            td(class = "p-1 py-2 text-xs xs:text-base break-words xs:break-normal") {
                (props.comparison_val.get_clone())
            }
        }
    }
}

#[derive(Props)]
struct ComparisonTableProps {
    comparison: ReadSignal<Comparison>,
    perseus_comparison: Comparison,
}
#[component]
fn ComparisonTable(props: ComparisonTableProps) -> View {
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

    let show_details_homepage_lighthouse_desktop = create_signal(false);
    let show_details_homepage_lighthouse_mobile = create_signal(false);

    // We now need to deconstruct the comparison with memos (actual pain)
    // Otherwise, the props passed through to the row component aren't considered
    // reactive
    let comparison_language = create_memo(move || comparison.get_clone().language.to_string());
    let comparison_supports_ssg = create_memo(move || comparison.get_clone().supports_ssg.render());
    let comparison_supports_ssr = create_memo(move || comparison.get_clone().supports_ssr.render());
    let comparison_supports_ssr_ssg_same_page =
        create_memo(move || comparison.get_clone().supports_ssr_ssg_same_page.render());
    let comparison_supports_i18n = create_memo(move || comparison.get_clone().supports_i18n.render());
    let comparison_supports_incremental =
        create_memo(move || comparison.get_clone().supports_incremental.render());
    let comparison_supports_revalidation =
        create_memo(move || comparison.get_clone().supports_revalidation.render());
    let comparison_inbuilt_cli = create_memo(move || comparison.get_clone().inbuilt_cli.render());
    let comparison_inbuilt_routing = create_memo(move || comparison.get_clone().inbuilt_routing.render());
    let comparison_supports_shell = create_memo(move || comparison.get_clone().supports_shell.render());
    let comparison_supports_deployment =
        create_memo(move || comparison.get_clone().supports_deployment.render());
    let comparison_supports_exporting =
        create_memo(move || comparison.get_clone().supports_exporting.render());
    let comparison_text = create_memo(move || comparison.get_clone().text.to_string());
    let comparison_name = create_memo(move || comparison.get_clone().name.to_string());
    let comparison_name_str = comparison_name.get_clone();
    let comparison_name_str2 = comparison_name.get_clone();

    view! {
        table(class = "w-full overflow-x-scroll table-fixed border-collapse") {
            thead(class = "mt-4 text-white bg-indigo-500 dark:bg-indigo-700 rounded-xl") {
                th(class = "p-1 py-2 text-xs xs:text-base") {
                    (t!( "comparisons-table-header"))
                }
                th(class = "p-1 py-2 text-xs xs:text-base") {
                    (t!( "perseus"))
                }
                th(class = "p-1 py-2 text-xs xs:text-base") {
                    (comparison_name_str.clone())
                }
            }
            tbody {
                // One row for each comparison point
                // One heading explaining it
                // Then two cells, one Perseus, and the for the comparison
                ComparisonRow(
                    perseus_val = perseus_language,
                    comparison_val = comparison_language,
                    name = "language".to_string()
                )
                ComparisonRow(
                    perseus_val = perseus_supports_ssg.render(),
                    comparison_val = comparison_supports_ssg,
                    name = "supports_ssg".to_string()
                )
                ComparisonRow(
                    perseus_val = perseus_supports_ssr.render(),
                    comparison_val = comparison_supports_ssr,
                    name = "supports_ssr".to_string()
                )
                ComparisonRow(
                    perseus_val = perseus_supports_ssr_ssg_same_page.render(),
                    comparison_val = comparison_supports_ssr_ssg_same_page,
                    name = "supports_ssr_ssg_same_page".to_string()
                )
                ComparisonRow(
                    perseus_val = perseus_supports_i18n.render(),
                    comparison_val = comparison_supports_i18n,
                    name = "supports_i18n".to_string()
                )
                ComparisonRow(
                    perseus_val = perseus_supports_incremental.render(),
                    comparison_val = comparison_supports_incremental,
                    name = "supports_incremental".to_string()
                )
                ComparisonRow(
                    perseus_val = perseus_supports_revalidation.render(),
                    comparison_val = comparison_supports_revalidation,
                    name = "supports_revalidation".to_string()
                )
                ComparisonRow(
                    perseus_val = perseus_inbuilt_cli.render(),
                    comparison_val = comparison_inbuilt_cli,
                    name = "inbuilt_cli".to_string()
                )
                ComparisonRow(
                    perseus_val = perseus_inbuilt_routing.render(),
                    comparison_val = comparison_inbuilt_routing,
                    name = "inbuilt_routing".to_string()
                )
                ComparisonRow(
                    perseus_val = perseus_supports_shell.render(),
                    comparison_val = comparison_supports_shell,
                    name = "supports_shell".to_string()
                )
                ComparisonRow(
                    perseus_val = perseus_supports_deployment.render(),
                    comparison_val = comparison_supports_deployment,
                    name = "supports_deployment".to_string()
                )
                ComparisonRow(
                    perseus_val = perseus_supports_exporting.render(),
                    comparison_val = comparison_supports_exporting,
                    name = "supports_exporting".to_string()
                )
                // These last two get special rendering for text colors and possible emoji
                tr {
                    th(class = "text-left p-1 py-2 text-xs xs:text-base") {
                        div(class = "flex items-center") {
                            (t!( "comparisons-table-headings.homepage_lighthouse_desktop"))
                            span(
                                class = "ml-1",
                                on:click = move |_| {
                                    show_details_homepage_lighthouse_desktop.set(!show_details_homepage_lighthouse_desktop.get())
                                },
                                dangerously_set_inner_html = INFO_SVG
                            )
                        }
                        p(
                            class = format!(
                                "italic font-normal {}",
                                if show_details_homepage_lighthouse_desktop.get() {
                                    "visible"
                                } else {
                                    "hidden"
                                }
                            )
                        ) {
                            (t!( "comparisons-table-details.homepage_lighthouse_desktop"))
                        }
                    }
                    td(class = "p-1 py-2 text-xs xs:text-base") {
                        (render_lighthouse_score( perseus_homepage_lighthouse_desktop))
                    }
                    td(class = "p-1 py-2 text-xs xs:text-base") {
                        (render_lighthouse_score( comparison.get_clone().homepage_lighthouse_desktop))
                    }
                }
                tr {
                    th(class = "text-left p-1 py-2 text-xs xs:text-base") {
                        div(class = "flex items-center") {
                            (t!( "comparisons-table-headings.homepage_lighthouse_mobile"))
                            span(
                                class = "ml-1",
                                on:click = move |_| {
                                    show_details_homepage_lighthouse_mobile.set(!show_details_homepage_lighthouse_mobile.get())
                                },
                                dangerously_set_inner_html = INFO_SVG
                            )
                        }
                        p(
                            class = format!(
                                "italic font-normal {}",
                                if show_details_homepage_lighthouse_mobile.get() {
                                    "visible"
                                } else {
                                    "hidden"
                                }
                            )
                        ) {
                            (t!( "comparisons-table-details.homepage_lighthouse_mobile"))
                        }
                    }
                    td(class = "p-1 py-2 text-xs xs:text-base") {
                        (render_lighthouse_score( perseus_homepage_lighthouse_mobile))
                    }
                    td(class = "p-1 py-2 text-xs xs:text-base") {
                        (render_lighthouse_score( comparison.get_clone().homepage_lighthouse_mobile))
                    }
                }
            }
        }
        h3(class = "text-2xl underline") { (t!(

            "comparisons-unknown-heading",
            {
                "name" = &comparison_name_str2
            }
        )) }
        div(class = "w-full flex justify-center") {
            p(class = "max-w-prose") { (comparison_text.get_clone()) }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, UnreactiveState)]
pub struct ComparisonsPageProps {
    pub comparisons: HashMap<String, Comparison>,
    /// The comparison data for Perseus itself.
    pub perseus_comparison: Comparison,
}

pub fn comparisons_page(props: ComparisonsPageProps) -> View {
    let comparisons = props.comparisons.clone();
    let perseus_comparison = props.perseus_comparison;
    let mut comparison_names: Vec<String> = comparisons.keys().cloned().collect();
    comparison_names.sort();
    // The current comparison should be the first element in the list alphabetically
    let curr_comparison_name = create_signal(comparison_names[0].clone());

    let select_options = comparison_names
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
            .collect::<Vec<_>>();

    let curr_comparison = create_memo(move || {
        comparisons
            .get(&curr_comparison_name.get_clone())
            .unwrap()
            .clone()
    });

    let comparisons_extra = t!( "comparisons-extra");
    let menu_signal = create_signal(false);
    let perseus_title = t!( "perseus");
    let text_color = "text-black dark:text-white".to_string();
    let menu_color = "bg-black dark:bg-white".to_string();
    let mobile_nav = View::default();

    let header_props = HeaderProps {
        title: perseus_title,
        text_color: text_color,
        menu_color: menu_color,
        mobile_nav_extension: mobile_nav,
        menu_open: Some(menu_signal),
    };

    let comparisons_heading = t!( "comparisons-heading");
    let comparisons_subtitle = t!( "comparisons-subtitle");
    let sycamore_heading = t!( "comparisons-sycamore-heading");
    let sycamore_text = t!( "comparisons-sycamore-text");

    view! {
        Container(
            header = header_props,
            footer = true,
        ) {
            div(class = "flex flex-col justify-center text-center dark:text-white mt-14 xs:mt-16 sm:mt-20 lg:mt-25") {
                div {
                    h1(class = "text-5xl xs:text-7xl sm:text-8xl font-bold") {
                        (comparisons_heading)
                    }
                    br()
                        p(class = "text-lg") {
                            (comparisons_subtitle)
                        }
                    p(
                        class = "italic px-1",
                        dangerously_set_inner_html = comparisons_extra
                    )
                }
                br(class = "mb-2 sm:mb-16 md:mb-24")
                    div(class = "p-1") {
                        select(
                            class = "p-2 rounded-sm dark:bg-neutral-800 mb-4",
                            on:input = move |event: web_sys::Event| {
                                use wasm_bindgen::JsCast;
                                let target: web_sys::HtmlSelectElement = event.target().unwrap().unchecked_into();
                                let new_comparison_name = target.value();
                                curr_comparison_name.set(new_comparison_name);
                            }
                        ) {
                            (select_options)
                        }
                        br()
                            div(class = "px-3 w-full sm:mr-auto sm:ml-auto sm:max-w-prose lg:max-w-3xl xl:max-w-4xl 2xl:max-w-5xl") {
                                div(class = "flex justify-center flex-col") {
                                    ComparisonTable(
                                        comparison = curr_comparison,
                                        perseus_comparison = perseus_comparison,
                                    )
                                }
                            }
                        br(class = "mb-1 sm:mb-8 md:mb-12")
                            h3(class = "text-xl underline") { (sycamore_heading) }
                        div(class = "w-full flex justify-center text-sm") {
                            p(class = "max-w-prose") { (sycamore_text) }
                        }
                }
            }
        }
    }
}

#[engine_only_fn]
pub fn head() -> View {
    view! {
        title { (format!("{} | {}", t!( "comparisons-title"), t!( "perseus"))) }
    }
}

pub fn get_template() -> Template {
    Template::build("comparisons")
        .view_with_unreactive_state(comparisons_page)
        .head(head)
        .build_state_fn(get_build_state)
        .build()
}

#[engine_only_fn]
async fn get_build_state(
    StateGeneratorInfo { locale, .. }: StateGeneratorInfo<()>,
) -> Result<ComparisonsPageProps, BlamedError<Error>> {
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
        let entry = entry.map_err(Error::from)?;
        let path = entry.path();
        // Ignore any empty directories or the like
        if path.is_file() {
            // There shouldn't be any non-Unicode comparison files
            let path_str = path.to_str().unwrap();
            let contents = fs::read_to_string(path).map_err(Error::from)?;
            // If the file is `perseus.json`, we'll add this to a special variable,
            // otherwise it gets added to the generic map
            if path_str.ends_with("perseus.json") {
                // The Perseus comparison has no localized text
                let comparison =
                    serde_json::from_str::<Comparison>(&contents).map_err(Error::from)?;
                perseus_comparison = Some(comparison);
            } else {
                // Other comparisons have multiple comparison paragraphs, one
                // for each locale (we have to choose the right one)
                let raw_comparison =
                    serde_json::from_str::<RawComparison>(&contents).map_err(Error::from)?;
                let comparison_text = match raw_comparison.text.get(&locale) {
                    Some(text) => text.to_string(),
                    None => {
                        return Err(BlamedError {
                            error: format!(
                            "comparison {} does not have localized comparison text for locale {}",
                            raw_comparison.name, locale
                        )
                            .into(),
                            blame: ErrorBlame::Server(None),
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
            None => return Err(BlamedError {
                error: "perseus comparison data not recorded, please ensure `comparisons/perseus.json` exists".to_string().into(),
                blame: ErrorBlame::Server(None)
            })
        }
    };
    Ok(props)
}
