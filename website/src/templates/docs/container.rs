use crate::templates::docs::generation::{
    get_beta_versions, get_outdated_versions, get_stable_version, DocsManifest, DocsVersionStatus,
};
use crate::components::container::Container;
use crate::components::header::HeaderProps;
use perseus::i18n::Translator;
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
    let docs_links_clone = docs_links.clone();
    let status = props.status.clone();
    let docs_version_switcher_props = DocsVersionSwitcherProps {
        manifest: props.manifest.clone(),
        current_version: props.current_version.clone(),
    };
    let dvsp_clone = docs_version_switcher_props.clone();

    // Create a parent `RcSignal` for controlling the header's menu, which we'll link into the sidebar
    let menu_open_root = create_rc_signal(false);
    // We then want it in a convenient form locally
    let menu_open = create_ref(cx, menu_open_root.clone());

    view! { cx,
        Container {
            header: HeaderProps {
                text_color: "text-black".to_string(),
                title: t!("perseus", cx),
                mobile_nav_extension: view! { cx,
                    hr()
                    div(class = "text-left p-3") {
                        DocsVersionSwitcher(docs_version_switcher_props)
                        div(class = "docs-links-markdown", dangerously_set_inner_html = &docs_links)
                    }
                },
                menu_open: Some(menu_open_root),
            },
            children: view! { cx,
                // TODO Dual-pane scroll on desktop
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
                                        DocsVersionSwitcher(dvsp_clone)
                                        div(class = "docs-links-markdown", dangerously_set_inner_html = &docs_links_clone)
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
            }
        }
    }
}
