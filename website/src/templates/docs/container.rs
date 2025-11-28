use super::search_bar::SearchBar;
use crate::components::container::Container;
use crate::components::footer::Footer;
use crate::components::header::HeaderProps;
use crate::templates::docs::generation::{
    get_beta_versions, get_outdated_versions, get_stable_version, DocsManifest, DocsVersionStatus,
};
use perseus::prelude::*;
use std::rc::Rc;
use sycamore::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Clone, Props)]
struct DocsVersionSwitcherProps {
    manifest: DocsManifest,
    current_version: String,
}
#[component]
fn DocsVersionSwitcher(props: DocsVersionSwitcherProps) -> View {
    // We'll fill this in from the reactive scope
    // Astonishingly, this actually works...
    let locale = create_signal(String::new());

    // In Sycamore 0.9.2, we can use strings directly
    let current_version = props.current_version.to_string();
    let stable_version = get_stable_version(&props.manifest).0;
    let stable_version2 = stable_version.clone();
    let stable_version3 = stable_version.clone();

    let beta_versions = {
        let mut versions = get_beta_versions(&props.manifest)
            .into_keys()
            .collect::<Vec<String>>();
        versions.sort_by(|a, b| b.partial_cmp(a).unwrap());
        versions
            .into_iter()
            .map(|version| {
                let version_clone = version.clone();
                let version_clone2 = version.clone();
                view! {
                        option(value = version, selected = current_version == version_clone) { (t!( "docs-version-switcher.beta", {
                            "version" = &version_clone2
                        })) }
                }
            })
            .collect::<Vec<_>>()
    };
    let old_versions = {
        let mut versions = get_outdated_versions(&props.manifest)
            .into_keys()
            .collect::<Vec<String>>();
        versions.sort_by(|a, b| b.partial_cmp(a).unwrap());
        versions
            .into_iter()
            .map(|version| {
                let version_clone = version.clone();
                let version_clone2 = version.clone();
                view! {
                        option(value = version, selected = current_version == version_clone) { (t!( "docs-version-switcher.outdated", {
                            "version" = &version_clone2
                        })) }
                }
            })
            .collect::<Vec<_>>()
    };

    view! {
        ({
            locale.set(use_context::<Rc<Reactor>>().get_translator().get_locale());
            View::default()
        })

        // This doesn't navigate to the same page in the new version, because it may well not exist
        select(
            class = "p-2 rounded-md text-white bg-indigo-500",
            on:input = move |event: web_sys::Event| {
                let target: web_sys::HtmlSelectElement = event.target().unwrap().unchecked_into();
                let new_version = target.value();
                // This isn't a reactive scope, so we can't use `link!` here
                // The base path will be included by HTML automatically
                let link = format!("{}/docs/{}/intro", locale.get_clone(), new_version);
                navigate(&link);
            }
        ) {
            option(value = "next", selected = current_version == "next") {
                (t!( "docs-version-switcher.next"))
            }
            (beta_versions)
            option(value = stable_version, selected = current_version == stable_version2) {
                (t!( "docs-version-switcher.stable", {
                    "version" = &stable_version3
                }))
            }
            (old_versions)
        }
    }
}

#[derive(Props)]
pub struct DocsContainerProps {
    pub children: View,
    pub docs_links: String,
    pub status: DocsVersionStatus,
    pub manifest: DocsManifest,
    pub current_version: String,
}

#[component]
pub fn DocsContainer(props: DocsContainerProps) -> View {
    let docs_links = props.docs_links.clone();
    let docs_links_clone = docs_links.clone();
    let status = props.status.clone();
    let docs_version_switcher_props = DocsVersionSwitcherProps {
        manifest: props.manifest.clone(),
        current_version: props.current_version.clone(),
    };
    let dvsp_clone = docs_version_switcher_props.clone();
    // In Sycamore 0.9.2, we can use strings directly
    let stable_version = get_stable_version(&props.manifest).0;

    let header_props = HeaderProps {
        text_color: "text-black dark:text-white".to_string(),
        menu_color: "bg-black dark:bg-white".to_string(),
        title: t!( "perseus"),
        mobile_nav_extension: view! {
            hr()
            div(class = "text-left p-3 overflow-y-scroll h-[60vh]") {
                div(class = "flex w-full justify-center text-center") {
                    div(class = "max-w-3xl flex flex-col") {
                        SearchBar()
                        DocsVersionSwitcher(
                            manifest = docs_version_switcher_props.manifest,
                            current_version = docs_version_switcher_props.current_version,
                        )
                    }
                }
                div(class = "docs-links-markdown", dangerously_set_inner_html = docs_links)
            }
        },
        menu_open: None,
    };

    view! {
        Container(
            header = header_props,
            footer = false,
        ) {
                // TODO Use shadow DOM to avoid replicating all docs links etc. in initial loads
                div(class = "flex w-full") {
                    // The sidebar that'll display navigation through the docs
                    div(class = "h-screen pt-14 xs:pt-16 sm:pt-20 lg:pt-25 hidden md:block max-w-xs w-full border-r overflow-y-auto") {
                        div(class = "mr-5") {
                            div(class = "text-left text-black dark:text-white p-3") {
                                aside {
                                    div(class = "flex flex-col") {
                                        SearchBar()
                                        DocsVersionSwitcher(
                                            manifest = dvsp_clone.manifest,
                                            current_version = dvsp_clone.current_version,
                                        )
                                    }
                                    div(class = "docs-links-markdown", dangerously_set_inner_html = docs_links_clone)
                                }
                            }
                        }
                    }
                    div(class = "h-screen pt-14 xs:pt-16 sm:pt-20 lg:pt-25 grid grid-rows-[1fr_min-content] w-full overflow-y-auto") {
                        // These styles were meticulously arrived at through pure trial and error...
                        div(class = "px-3 w-full sm:mr-auto sm:ml-auto sm:max-w-prose lg:max-w-3xl xl:max-w-4xl 2xl:max-w-5xl min-w-0") {
                            (status.render( stable_version.to_string()))
                            main(class = "text-black dark:text-white") {
                                (props.children)
                            }
                        }
                        div(class = "row-start-2") {
                            Footer {}
                        }
                    }
                }
        }
    }
}
