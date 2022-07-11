use crate::components::container::NavLinks;
use crate::components::container::COPYRIGHT_YEARS;
use crate::templates::docs::generation::{
    get_beta_versions, get_outdated_versions, get_stable_version, DocsManifest, DocsVersionStatus,
};
use perseus::internal::i18n::Translator;
use perseus::{link, navigate, t};
use sycamore::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Clone)]
struct DocsVersionSwitcherProps {
    manifest: DocsManifest,
    current_version: String,
}
#[component(DocsVersionSwitcher<G>)]
fn DocsVersionSwitcher<G: Html>(cx: Scope, props: DocsVersionSwitcherProps) -> View<G> {
    // We'll fill this in from the reactive scope
    // Astonishingly, this actually works...
    let locale = create_signal(cx, String::new());

    let current_version = create_ref(cx, props.current_version.to_string());
    let stable_version = create_ref(cx, get_stable_version(&props.manifest).0);

    let beta_versions = View::new_fragment({
        let mut versions = get_beta_versions(&props.manifest)
            .into_keys()
            .collect::<Vec<String>>();
        versions.sort_by(|a, b| b.partial_cmp(a).unwrap());
        versions
            .into_iter()
            .map(|version| {
                let version = create_ref(cx, version);
                view! { cx,
                        option(value = &version, selected = current_version == version) { (t!("docs-version-switcher.beta", {
                            "version" = version.to_string()
                        }, cx)) }
                }
            })
            .collect()
    });
    let old_versions = View::new_fragment({
        let mut versions = get_outdated_versions(&props.manifest)
            .into_keys()
            .collect::<Vec<String>>();
        versions.sort_by(|a, b| b.partial_cmp(a).unwrap());
        versions
            .into_iter()
            .map(|version| {
                let version = create_ref(cx, version);
                view! { cx,
                        option(value = version, selected = current_version == version) { (t!("docs-version-switcher.outdated", {
                            "version" = version.to_string()
                        }, cx)) }
                }
            })
            .collect()
    });

    view! { cx,
        ({
            locale.set(use_context::<Signal<Translator>>(cx).get_untracked().get_locale());
            View::empty()
        })

        // This doesn't navigate to the same page in the new version, because it may well not exist
        select(
            class = "p-2 rounded-md text-white bg-indigo-500",
            on:input = move |event: web_sys::Event| {
                let target: web_sys::HtmlInputElement = event.target().unwrap().unchecked_into();
                let new_version = target.value();
                // This isn't a reactive scope, so we can't use `link!` here
                // The base path will be included by HTML automatically
                let link = format!("{}/docs/{}/intro", *locale.get(), new_version);
                navigate(&link);
            }
        ) {
            option(value = "next", selected = current_version == "next") {
                (t!("docs-version-switcher.next", cx))
            }
            (beta_versions)
            option(value = stable_version, selected = current_version == stable_version) {
                (t!("docs-version-switcher.stable", {
                    "version" = stable_version.to_string()
                }, cx))
            }
            (old_versions)
        }
    }
}

#[derive(Clone)]
pub struct DocsContainerProps<G: GenericNode> {
    pub children: View<G>,
    pub docs_links: String,
    pub status: DocsVersionStatus,
    pub manifest: DocsManifest,
    pub current_version: String,
}

#[component(DocsContainer<G>)]
pub fn DocsContainer<G: Html>(cx: Scope, props: DocsContainerProps<G>) -> View<G> {
    let docs_links = props.docs_links.clone();
    let docs_links_2 = docs_links.clone();
    let status = props.status.clone();
    let docs_version_switcher_props = DocsVersionSwitcherProps {
        manifest: props.manifest.clone(),
        current_version: props.current_version.clone(),
    };
    let docs_version_switcher_props_2 = docs_version_switcher_props.clone();

    let menu_open = create_signal(cx, false);
    let toggle_menu = |_| menu_open.set(!*menu_open.get());

    view! { cx,
        // TODO click-away events
        header(class = "shadow-md sm:p-2 w-full bg-white dark:text-white dark:bg-navy mb-20") {
            div(class = "flex justify-between") {
                a(class = "justify-self-start self-center m-3 ml-5 text-md sm:text-2xl", href = link!("/", cx)) {
                    (t!("perseus", cx))
                }
                // The button for opening/closing the hamburger menu on mobile
                // This is done by a Tailwind module
                div(
                    class = format!(
                        "md:hidden m-3 mr-5 tham tham-e-spin tham-w-6 {}",
                        if *menu_open.get() {
                            "tham-active"
                        } else {
                            ""
                        }
                    ),
                    on:click = toggle_menu
                ) {
                    div(class = "tham-box") {
                        div(class = "dark:bg-white tham-inner") {}
                    }
                }
                // This displays the navigation links on desktop
                // But it needs to hide at the same time as the sidebar
                nav(class = "hidden md:flex") {
                    ul(class = "mr-5 flex") {
                        NavLinks()
                    }
                }
            }
            // This displays the navigation links when the menu is opened on mobile
            // TODO click-away event
            nav(
                id = "mobile_nav_menu",
                class = format!(
                    "md:hidden w-full text-center justify-center overflow-y-scroll {}",
                    if *menu_open.get() {
                        "flex flex-col"
                    } else {
                        "hidden"
                    }
                )
            ) {
                // TODO find a solution that lets you scroll here that doesn't need a fixed height
                div(class = "mr-5 overflow-y-scroll", style = "max-height: 500px") {
                    ul {
                        NavLinks()
                    }
                    hr()
                    div(class = "text-left p-3") {
                        DocsVersionSwitcher(docs_version_switcher_props)
                        div(class = "docs-links-markdown", dangerously_set_inner_html = &docs_links)
                    }
                }
            }
        }
        div(
            class = format!(
                "mt-14 xs:mt-16 sm:mt-20 lg:mt-25 overflow-y-auto {}",
                if !*menu_open.get() {
                    "flex"
                } else {
                    "hidden"
                }
            )
        ) {
            div(class = "flex w-full") {
                // The sidebar that'll display navigation through the docs
                div(class = "h-full hidden md:block max-w-xs w-full border-r") {
                    div(class = "mr-5") {
                        div(class = "text-left text-black dark:text-white p-3") {
                            aside {
                                DocsVersionSwitcher(docs_version_switcher_props_2)
                                div(class = "docs-links-markdown", dangerously_set_inner_html = &docs_links_2)
                            }
                        }
                    }
                }
                div(class = "h-full flex w-full") {
                    // These styles were meticulously arrived at through pure trial and error...
                    div(class = "px-3 w-full sm:mr-auto sm:ml-auto sm:max-w-prose lg:max-w-3xl xl:max-w-4xl 2xl:max-w-5xl") {
                        (status.render(cx))
                        main(class = "text-black dark:text-white") {
                            (props.children.clone())
                        }
                    }
                }
            }
        }
        footer(class = "w-full flex justify-center py-5 bg-gray-100 dark:bg-navy-deep") {
            p(class = "dark:text-white mx-5 text-center") {
                span(dangerously_set_inner_html = &t!("footer.copyright", {
                    "years" = COPYRIGHT_YEARS
                }, cx))
            }
        }
    }
}
